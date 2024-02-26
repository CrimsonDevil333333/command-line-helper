mod modules;

use modules::youtube_module::download_video;
use clap::Parser;
use std::env;

/// Simple utility program
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: Option<String>,

    /// Youtube URL
    #[clap(short, long)]
    url: Option<String>,

    /// Number of times to greet
    #[clap(short, long, default_value = "1")]
    count: u8,

    /// Output path for downloaded video
    #[clap(short, long, default_value = ".")]
    output_path: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let mut args = Args::parse();

    // Set default output path to the current working directory
    if args.output_path.to_string_lossy() == "." {
        if let Ok(current_dir) = env::current_dir() {
            args.output_path = current_dir;
        }
    }

    // Check if the name is present
    if let Some(name) = &args.name {
        // Print greetings
        for _ in 0..args.count {
            println!("Hello {}!", name);
        }
    }

    // Check if the URL is present before attempting to download
    if let Some(url) = &args.url {
        // Download the video
        println!("Downloading video ...");
        if let Err(err) = download_video(url, &args.output_path.to_string_lossy()).await {
            eprintln!("Error: {}", err);
        }
    }

    if args.name.is_none() && args.url.is_none() {
        eprintln!("Error: You must provide either a name or a URL. Use --help to see the help message.");
    }
}
