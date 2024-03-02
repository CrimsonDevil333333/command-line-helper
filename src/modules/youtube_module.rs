use log::error;
use rustube::Id;
use rustube::VideoFetcher;

pub async fn download_video(
    url: &str,
    download_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse video ID from URL
    let id = match Id::from_raw(url) {
        Ok(id) => id,
        Err(err) => {
            error!("Error parsing video ID from URL: {}", err);
            return Err(Box::new(err));
        }
    };

    // Create a VideoFetcher instance from the video ID
    let fetcher = match VideoFetcher::from_id(id.into_owned()) {
        Ok(fetcher) => fetcher,
        Err(err) => {
            error!("Error creating VideoFetcher: {}", err);
            return Err(Box::new(err));
        }
    };

    // Fetch video information
    let descrambler = match fetcher.fetch().await {
        Ok(descrambler) => descrambler,
        Err(err) => {
            error!("Error fetching video information: {}", err);
            return Err(Box::new(err));
        }
    };

    // Descramble the video
    let video = match descrambler.descramble() {
        Ok(video) => video,
        Err(err) => {
            error!("Error descrambling video: {}", err);
            return Err(Box::new(err));
        }
    };

    // Download the best quality video to the specified path
    match video
        .best_quality()
        .unwrap()
        .download_to_dir(download_path)
        .await
    {
        Ok(_) => {
            println!("Download complete");
            Ok(())
        }
        Err(err) => {
            error!("Error downloading video: {}", err);
            Err(Box::new(err))
        }
    }
}
