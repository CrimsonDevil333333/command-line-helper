// main.rs

mod modules;
mod utilities;

use clap::Parser;
use log::info;
use std::path::PathBuf;

use modules::language_identifier_module::identify_project_type;
use modules::language_module::execute_language_action;
use modules::logging_module::setup_logging;
use modules::os_modules::{copy_file, move_file, search_files};
use modules::youtube_module::download_video;
use utilities::{
    clean_action_string, clean_language_string, is_language_installed, print_colored_path,
    print_error_message, suggest_installation,
};


#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Name of the person to greet
    #[clap(short = 'n', long)]
    name: Option<String>,

    /// Youtube URL
    #[clap(short = 'u', long)]
    url: Option<String>,

    /// Number of times to greet
    #[clap(short = 'c', long, default_value = "1")]
    count: u8,

    /// Output path for downloaded video
    #[clap(short = 'p', long, default_value = ".")]
    output_path: std::path::PathBuf,

    /// Verbose mode
    #[clap(short = 'v', long)]
    verbose: bool,

    /// Logs output in a file for debug
    #[clap(short = 'o', long = "logs-out")]
    log_out: bool,

    /// Search for files in the provided output path
    #[clap(short = 's', long)]
    search: Option<String>,

    /// Limit the number of search results
    #[clap(short = 'l', long, default_value = "0")]
    limit: usize,

    /// Copy files to the output path
    #[clap(long = "copy")]
    copy: Option<PathBuf>,

    /// Move files to the output path
    #[clap(long = "move")]
    move_files: Option<PathBuf>,

    /// Language to perform the action on
    #[clap(short = 'L', long)]
    language: Option<String>,

    /// Action to perform
    #[clap(short = 'r', long)]
    action: Option<String>,
}

#[allow(dead_code)]
fn validate_language_action(
    language: &Option<String>,
    action: &Option<String>,
) -> Result<(), String> {
    match (language.as_deref(), action.as_deref()) {
        (Some(lang), Some(act)) => {
            let lang_lower = lang.to_lowercase();
            let act_lower = act.to_lowercase();

            if !["java", "python", "dotnet", "rust", "js"].contains(&lang_lower.as_str()) {
                return Err(format!("Invalid language: {}", lang));
            }

            if !["run", "build", "test", "clean"].contains(&act_lower.as_str()) {
                return Err(format!("Invalid action: {}", act));
            }

            Ok(())
        }
        _ => Ok(()), // Both language and action are empty, so skip validation
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    setup_logging(args.verbose, args.log_out);

    info!("This is the starting of a new run!!!");

    // No need "execute_language_action" will handle validation !
    // if let Err(err) = validate_language_action(&args.language, &args.action) {
    //     print_error_message(&format!("Error: {}\n", err));
    //     return;
    // }

    if let (Some(fetched_language), Some(fetched_action)) = (&args.language, &args.action) {
        let cleaned_language_str = clean_language_string(&fetched_language);
        let cleaned_action_str = clean_action_string(&fetched_action);

        if !is_language_installed(&cleaned_language_str) {
            suggest_installation(&cleaned_language_str);
        }
        // Perform the action
        execute_language_action(&cleaned_language_str, &cleaned_action_str);
    } else if args.language.is_none() {
        // If action is present but language is not, try to dynamically identify the language using the current path
        match identify_project_type(".") {
            Ok(project_type) => {
                println!("Identified project type: {}", project_type);
                if let Some(fetched_action) = &args.action {
                    let cleaned_language_str =
                        project_type.replace("rust", "cargo").replace("\"", "");
                    let cleaned_action_str = clean_action_string(fetched_action);

                    if !is_language_installed(&cleaned_language_str) {
                        suggest_installation(&cleaned_language_str);
                    }
                    // Perform the action
                    execute_language_action(&cleaned_language_str, &cleaned_action_str);
                }
            }
            Err(error) => eprintln!("Error identifying project type: {}", error),
        }
    } else {
        // Check if both language and action are empty before printing an error
        if args.language.is_none() && args.action.is_none() {
            // Do nothing when both are empty
        } else {
            print_error_message("Error: You must provide both --language and --action. Use --help to see the help message.\n");
        }
    }

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
        search_files(pattern, &args.output_path, &args.limit);
    }

    // Perform copy based on arguments
    if let Some(copy_path) = &args.copy {
        let _ = copy_file(copy_path, &args.output_path, &args.name);
    }

    // Perform move based on arguments
    if let Some(move_path) = &args.move_files {
        let _ = move_file(move_path, &args.output_path, &args.name);
    }

    if args.name.is_none()
        && args.url.is_none()
        && args.search.is_none()
        && args.copy.is_none()
        && args.move_files.is_none()
        && args.language.is_none()
        && args.action.is_none()
    {
        print_error_message(
            "Error: You must provide either a name, a URL, use --search, or use --copy/--move. \
            Additionally, provide --language and --action. Use --help to see the help message.\n",
        );
    }
}
