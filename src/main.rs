// main.rs

mod modules;
mod utilities;

use modules::youtube_module::download_video;
use modules::logging_module::setup_logging;
use utilities::print_colored_path;
use globwalk::GlobWalkerBuilder;
use std::path::PathBuf;
use clap::Parser;
use log::info;

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
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    setup_logging(args.verbose, args.log_out); // Set up logging with verbose mode

    info!("This is starting of new run!!!");

    // Check if the search option is present
    if let Some(pattern) = &args.search {
        // Search for files in the provided output path recursively using globwalk
        let search_path = PathBuf::from(&args.output_path);
        let walker = GlobWalkerBuilder::from_patterns(&search_path, &[pattern])
            .max_depth(if args.limit > 0 { args.limit } else { usize::MAX })
            .build()
            .unwrap();

        let mut count = 0;
        for entry in walker {
            match entry {
                Ok(entry) => {
                    if args.limit > 0 && count >= args.limit {
                        break;
                    }
                    print_colored_path(&entry.path());
                    // println!("{}", entry.path().display());
                    count += 1;
                }
                Err(e) => eprintln!("Error: {}", e),
            }
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

    if args.name.is_none() && args.url.is_none() && args.search.is_none() {
        eprintln!("Error: You must provide either a name, a URL, or use --search. Use --help to see the help message.");
    }
}
