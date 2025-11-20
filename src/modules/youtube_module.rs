use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::error;
use rustube::Id;
use rustube::VideoFetcher;

pub async fn download_video(
    url: &str,
    download_path: &str,
    quality: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "YouTube Video Downloader".cyan().bold());
    println!("{}", "=".repeat(80).cyan());

    // Parse video ID from URL
    let id = match Id::from_raw(url) {
        Ok(id) => {
            println!("{} Parsed video ID: {}", "✓".green(), id.as_str().yellow());
            id
        }
        Err(err) => {
            error!("Error parsing video ID from URL: {}", err);
            eprintln!("{} Invalid YouTube URL", "Error:".red().bold());
            eprintln!("  Please provide a valid YouTube video URL");
            eprintln!("  Example: https://www.youtube.com/watch?v=VIDEO_ID");
            return Err(Box::new(err));
        }
    };

    // Create a VideoFetcher instance from the video ID
    println!("{} Fetching video information...", "→".cyan());
    let fetcher = match VideoFetcher::from_id(id.into_owned()) {
        Ok(fetcher) => fetcher,
        Err(err) => {
            error!("Error creating VideoFetcher: {}", err);
            eprintln!("{} Failed to create video fetcher", "Error:".red().bold());
            return Err(Box::new(err));
        }
    };

    // Fetch video information
    let descrambler = match fetcher.fetch().await {
        Ok(descrambler) => {
            println!("{} Video information retrieved", "✓".green());
            descrambler
        }
        Err(err) => {
            error!("Error fetching video information: {}", err);
            eprintln!(
                "{} Failed to fetch video information",
                "Error:".red().bold()
            );
            eprintln!("  The video might be private, age-restricted, or unavailable");
            return Err(Box::new(err));
        }
    };

    // Descramble the video
    println!("{} Processing video data...", "→".cyan());
    let video = match descrambler.descramble() {
        Ok(video) => {
            let title = video.video_details().title.clone();
            let author = video.video_details().author.clone();

            println!("\n{}", "Video Details:".yellow().bold());
            println!("  Title:  {}", title.green());
            println!("  Author: {}", author.green());
            println!();

            video
        }
        Err(err) => {
            error!("Error descrambling video: {}", err);
            eprintln!("{} Failed to process video data", "Error:".red().bold());
            return Err(Box::new(err));
        }
    };

    // Create progress bar
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    pb.set_message("Downloading...");

    // Select stream based on quality
    let stream = match quality.to_lowercase().as_str() {
        "worst" => video.worst_quality(),
        "audio" => video.best_audio(),
        _ => video.best_quality(), // Default to best
    };

    let stream = match stream {
        Some(s) => s,
        None => {
            eprintln!(
                "{} Requested quality '{}' not found, falling back to best quality",
                "Warning:".yellow().bold(),
                quality
            );
            video.best_quality().ok_or("No streams available")?
        }
    };

    // Download the selected stream to the specified path
    println!(
        "{} Starting download ({}) to: {}",
        "→".cyan(),
        quality.yellow(),
        download_path.yellow()
    );

    match stream.download_to_dir(download_path).await {
        Ok(path) => {
            pb.finish_with_message("Complete!");
            println!("\n{} Download complete!", "✓".green().bold());
            println!("  Saved to: {}", path.display().to_string().green());
            Ok(())
        }
        Err(err) => {
            pb.finish_with_message("Failed");
            error!("Error downloading video: {}", err);
            eprintln!("\n{} Download failed", "✗".red().bold());
            eprintln!("  Error: {}", err.to_string().yellow());
            eprintln!("\n{}", "Troubleshooting:".yellow().bold());
            eprintln!("  • Check your internet connection");
            eprintln!("  • Verify the download path is writable");
            eprintln!("  • Try a different video URL");
            Err(Box::new(err))
        }
    }
}
