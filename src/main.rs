// main.rs

mod modules;
mod utilities;

use clap::Parser;
use std::path::PathBuf;

use modules::language_identifier_module::identify_project_type;
use modules::language_module::execute_language_action;
use modules::logging_module::setup_logging;
use modules::os_modules::{copy_file, move_file, search_files};
use modules::search_data_module::search_data_in_files;
use modules::youtube_module::download_video;
use utilities::{
    clean_action_string, clean_language_string, is_language_installed, print_colored_path,
    print_error_message, suggest_installation,
};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Specifies the name to be used in operations like --copy and --move.
    #[clap(short = 'n', long)]
    name: Option<String>,

    /// Downloads YouTube videos or shots from the provided URL.
    #[clap(short = 'u', long)]
    url: Option<String>,

    /// Sets the output path for downloaded videos or destination for --copy/--move operations.
    #[clap(short = 'p', long, default_value = ".")]
    output_path: std::path::PathBuf,

    /// Enables verbose mode, providing detailed logs during execution.
    #[clap(short = 'v', long)]
    verbose: bool,

    /// Outputs trace logs to a file for debugging purposes.
    #[clap(short = 'o', long = "logs-out")]
    log_out: bool,

    /// Searches for files in the specified output path using the provided search pattern.
    #[clap(short = 's', long)]
    search: Option<String>,

    /// Limits the number of search results when using the --search/--data-search operation.
    #[clap(short = 'l', long, default_value = "0")]
    limit: usize,

    /// Copies files to the specified output path.
    #[clap(long = "copy")]
    copy: Option<PathBuf>,

    /// Moves files to the specified output path.
    #[clap(long = "move")]
    move_files: Option<PathBuf>,

    /// Specifies the programming language for targeted actions (enhances performance).
    #[clap(short = 'L', long)]
    language: Option<String>,

    /// Specifies the action to perform on the project (e.g., run, build, test).
    /// Use --goto along with this option for complex project structures.
    #[clap(short = 'r', long)]
    action: Option<String>,

    /// Navigates to the specified path from within the tool.
    #[clap(short = 'g', long)]
    goto: Option<PathBuf>,

    /// Search for specific data in files.
    #[clap(short = 'S', long = "data-search")]
    data_search: Option<String>,

    /// Specify the depth of search roots.
    #[clap(long = "root-level", default_value = "3")]
    root_level: usize,
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

    // No need "execute_language_action" will handle validation !
    // if let Err(err) = validate_language_action(&args.language, &args.action) {
    //     print_error_message(&format!("Error: {}\n", err));
    //     return;
    // }

    // Check if the user provided the --goto option
    if let Some(goto_path) = &args.goto {
        // Navigate to the specified path
        if goto_path.is_dir() {
            // The path is a directory
            std::env::set_current_dir(goto_path).unwrap_or_else(|err| {
                eprintln!("Error navigating to {}: {}", goto_path.display(), err);
            });
        } else {
            // The path is a file
            if let Some(parent_dir) = goto_path.parent() {
                std::env::set_current_dir(parent_dir).unwrap_or_else(|err| {
                    eprintln!("Error navigating to {}: {}", parent_dir.display(), err);
                });
            } else {
                eprintln!("Error getting parent directory of {}", goto_path.display());
            }
        }
        println!(
            "Navigated to: {}",
            std::env::current_dir().unwrap().display()
        );
    }

    // Check if the data_search option is present
    if let Some(search_data) = &args.data_search {
        // Search for data in files
        search_data_in_files(search_data, &args.output_path, args.root_level, args.limit);
    }

    if let (Some(fetched_language), Some(fetched_action)) = (&args.language, &args.action) {
        let cleaned_language_str = clean_language_string(&fetched_language);
        let cleaned_action_str = clean_action_string(&fetched_action);

        if !is_language_installed(&cleaned_language_str) {
            suggest_installation(&cleaned_language_str);
        }
        // Perform the action
        execute_language_action(&cleaned_language_str, &cleaned_action_str);
    } else if args.language.is_none() && !args.action.is_none() {
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
            println!("Hello {}!", name);
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
        && args.goto.is_none()
        && args.data_search.is_none()
    {
        print_error_message(
            "Error: You must provide either a name, a URL, use --search, or use --copy/--move. \
            Additionally, provide --language and --action. Use --help to see the help message.\n",
        );
    }
}
