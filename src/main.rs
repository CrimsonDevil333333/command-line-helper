// main.rs

mod modules;
mod utilities;

use log::info;
use clap::Parser;
use std::path::PathBuf;

use modules::logging_module::setup_logging;
use modules::youtube_module::download_video;
use modules::os_modules::{copy_file,move_file,search_files};
use utilities::{print_colored_path, print_error_message};

/// Simple utility program
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Name of the person to greet
    #[clap(short='n', long)]
    name: Option<String>,

    /// Youtube URL
    #[clap(short='u', long)]
    url: Option<String>,

    /// Number of times to greet
    #[clap(short='c', long, default_value = "1")]
    count: u8,

    /// Output path for downloaded video
    #[clap(short='p', long, default_value = ".")]
    output_path: std::path::PathBuf,

    /// Verbose mode
    #[clap(short='v', long)]
    verbose: bool,

    /// Logs output in a file for debug
    #[clap(short='o', long)]
    log_out: bool,

    /// Search for files in the provided output path
    #[clap(short='s', long)]
    search: Option<String>,

    /// Limit the number of search results
    #[clap(short='l', long, default_value = "0")]
    limit: usize,

    /// Copy files to the output path
    #[clap(long="copy")]
    copy: Option<PathBuf>,

    /// Move files to the output path
    #[clap(long="move")]
    move_files: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    setup_logging(args.verbose, args.log_out); // Set up logging with verbose mode

    info!("This is the starting of a new run!!!");

    // Check if the name is present and neither --copy nor --move are present
    if let Some(name) = &args.name {
        if args.copy.is_none() && args.move_files.is_none() {
            // Print greetings
            for _ in 0..args.count {
                println!("Hello {}!", name);
            }
        }
    }

    // Check if the URL is present before attempting to download
    if let Some(url) = &args.url {
        // Download the video
        println!("Downloading video ...");
        if let Err(err) = download_video(url, &args.output_path.to_string_lossy()).await {
            print_error_message(&format!("Error: {}\n", err));
        }
    }

    // Check if the search option is present
    if let Some(pattern) = &args.search {
        search_files(pattern ,&args.output_path, &args.limit);
    }

    // Perform copy based on arguments
    if let Some(copy_path) = &args.copy {
        let _ = copy_file(copy_path, &args.output_path, &args.name);
    }

    // Perform move based on arguments
    if let Some(move_path) = &args.move_files {
        let _ = move_file(move_path, &args.output_path, &args.name);
    }

    if args.name.is_none() && args.url.is_none() && args.search.is_none() && args.copy.is_none() && args.move_files.is_none() {
        print_error_message("Error: You must provide either a name, a URL, use --search, or use --copy/--move. Use --help to see the help message.\n");
    }
}