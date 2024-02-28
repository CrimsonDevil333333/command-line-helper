// main.rs

mod modules;

use modules::youtube_module::download_video;
use clap::Parser;
use std::env;
use log::{info, error};
use log4rs;

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

    /// Verbose mode
    #[clap(short, long)]
    verbose: bool,
}

fn setup_logging(verbose: bool) {
    // Initialize logging using log4rs programmatically
    if verbose {
        let log_format = "[{d(%Y-%m-%dT%H:%M:%S%.f%:z)}] : {l} : {m}{n}";
        let config = log4rs::Config::builder()
            .appender(
                log4rs::config::Appender::builder()
                    .build("console", Box::new(
                        log4rs::append::console::ConsoleAppender::builder()
                            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(log_format)))
                            .build(),
                    )),
            )
            .appender(
                log4rs::config::Appender::builder()
                    .build("file_verbose", Box::new(
                        log4rs::append::file::FileAppender::builder()
                            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(log_format)))
                            .build("logs.log")
                            .unwrap(),
                    )),
            )
            .build(
                log4rs::config::Root::builder()
                    .appender("console")
                    .appender("file_verbose")
                    .build(log::LevelFilter::Trace),
            )
            .unwrap();

        if let Err(e) = log4rs::init_config(config) {
            eprintln!("Error initializing logging: {}", e);
        }
    } else {
        let log_format = "[{d(%Y-%m-%dT%H:%M:%S%.f%:z)}] : {l} : {m}{n}";
        let config = log4rs::Config::builder()
            .appender(
                log4rs::config::Appender::builder()
                    .build("console", Box::new(
                        log4rs::append::console::ConsoleAppender::builder()
                            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(log_format)))
                            .build(),
                    )),
            )
            .build(
                log4rs::config::Root::builder()
                    .appender("console")
                    .build(log::LevelFilter::Info),
            )
            .unwrap();

        if let Err(e) = log4rs::init_config(config) {
            eprintln!("Error initializing logging: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Set default output path to the current working directory
    let mut output_path = args.output_path.clone();
    if output_path.to_string_lossy() == "." {
        if let Ok(current_dir) = env::current_dir() {
            output_path = current_dir;
        }
    }

    setup_logging(args.verbose); // Set up logging with verbose mode

    // info!("This is Test Log");

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
