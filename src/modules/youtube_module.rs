use colored::Colorize;
use std::path::Path;
use std::process::{Command, Stdio};

/// Downloads a YouTube video using yt-dlp
pub async fn download_video(
    url: &str,
    download_path: &str,
    quality: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "YouTube Video Downloader".cyan().bold());
    println!("{}", "=".repeat(80).cyan());

    // Check if yt-dlp is installed
    let yt_dlp_check = if cfg!(target_os = "windows") {
        Command::new("where").arg("yt-dlp").output()
    } else {
        Command::new("which").arg("yt-dlp").output()
    };

    match yt_dlp_check {
        Ok(output) if !output.status.success() => {
            eprintln!("{} yt-dlp is not installed", "Error:".red().bold());
            eprintln!("\n{}", "Installation Instructions:".yellow().bold());

            if cfg!(target_os = "windows") {
                eprintln!("  Windows:");
                eprintln!("    • Using winget: {}", "winget install yt-dlp".green());
                eprintln!("    • Using chocolatey: {}", "choco install yt-dlp".green());
                eprintln!("    • Using pip: {}", "pip install yt-dlp".green());
            } else if cfg!(target_os = "macos") {
                eprintln!("  macOS:");
                eprintln!("    • Using homebrew: {}", "brew install yt-dlp".green());
                eprintln!("    • Using pip: {}", "pip install yt-dlp".green());
            } else {
                eprintln!("  Linux:");
                eprintln!("    • Using pip: {}", "pip install yt-dlp".green());
                eprintln!(
                    "    • Or download from: {}",
                    "https://github.com/yt-dlp/yt-dlp".cyan()
                );
            }

            return Err("yt-dlp not found".into());
        }
        Err(e) => {
            eprintln!(
                "{} Failed to check for yt-dlp: {}",
                "Error:".red().bold(),
                e
            );
            return Err(e.into());
        }
        _ => {
            println!("{} yt-dlp found", "✓".green());
        }
    }

    // Build format string based on quality
    let format_arg = match quality.to_lowercase().as_str() {
        "worst" => "worstvideo+worstaudio/worst",
        "audio" => "bestaudio/best",
        _ => "bestvideo+bestaudio/best", // Default to best
    };

    println!("{} Quality: {}", "→".cyan(), quality.yellow());
    println!("{} Format: {}", "→".cyan(), format_arg.yellow());
    println!("{} Destination: {}", "→".cyan(), download_path.yellow());
    println!();

    // Build yt-dlp command
    let mut cmd = Command::new("yt-dlp");

    // Add format argument
    if quality.to_lowercase() == "audio" {
        cmd.arg("-x")
            .arg("--audio-format")
            .arg("mp3")
            .arg("-f")
            .arg(format_arg);
    } else {
        cmd.arg("-f").arg(format_arg);
    }

    // Add other arguments
    cmd.arg("--progress")
        .arg("--newline")
        .arg("-o")
        .arg(format!("{}/%(title)s.%(ext)s", download_path))
        .arg(url)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    println!("{} Starting download...", "→".cyan());

    // Execute command
    let output = cmd.output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Print relevant output lines
        for line in stdout.lines() {
            if line.contains("Destination:")
                || line.contains("100%")
                || line.contains("has already been downloaded")
            {
                println!("{}", line);
            }
        }

        println!("\n{} Download complete!", "✓".green().bold());
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("\n{} Download failed", "✗".red().bold());
        eprintln!("{}", stderr);

        eprintln!("\n{}", "Troubleshooting:".yellow().bold());
        eprintln!("  • Check your internet connection");
        eprintln!("  • Verify the YouTube URL is correct");
        eprintln!("  • Make sure the video is not private or restricted");
        eprintln!(
            "  • Try updating yt-dlp: {}",
            "pip install -U yt-dlp".green()
        );

        Err(format!("yt-dlp failed with exit code: {}", output.status).into())
    }
}
