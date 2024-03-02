use rustube::Id;
use rustube::VideoFetcher;

pub async fn download_video(
    url: &str,
    download_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse video ID from URL
    let id = Id::from_raw(url)?;

    // Create a VideoFetcher instance from the video ID
    let fetcher = VideoFetcher::from_id(id.into_owned())?;

    // Fetch video information
    let descrambler = fetcher.fetch().await?;

    // Descramble the video
    let video = descrambler.descramble()?;

    // Download the best quality video to the specified path
    video
        .best_quality()
        .unwrap()
        .download_to_dir(download_path)
        .await?;

    println!("Download complete");
    Ok(())
}
