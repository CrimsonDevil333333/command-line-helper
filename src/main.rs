// main.rs

mod config;
mod installer;
mod modules;
mod utilities;

use clap::Parser;
use std::path::{Path, PathBuf};

use modules::archive_module;
use modules::env_module;
use modules::format_module;
use modules::git_module;
use modules::hash_module::{self, HashAlgorithm};
use modules::language_identifier_module::identify_project_type;
use modules::language_module::execute_language_action;
use modules::logging_module::setup_logging;
use modules::network_module;
use modules::os_modules::{copy_file, move_file, search_files};
use modules::search_data_module::search_data_in_files;
use modules::server_module;
use modules::system_module;
use modules::text_module;
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

    /// Sets the quality for YouTube downloads (best, worst, audio).
    #[clap(long, default_value = "best")]
    quality: String,

    /// Sets the output path for downloaded videos or destination for --copy/--move operations.
    #[clap(short = 'o', long, default_value = ".")]
    output_path: std::path::PathBuf,

    /// Outputs trace logs to a file for debugging purposes.
    #[clap(short = 'O', long = "logs-out")]
    log_out: bool,

    /// Enables verbose mode, providing detailed logs during execution.
    #[clap(short = 'v', long)]
    verbose: bool,

    /// Searches for files in the specified output path using the provided search pattern.
    #[clap(short = 's', long)]
    search: Option<String>,

    /// Search for specific data in files.
    #[clap(short = 'S', long = "data-search")]
    data_search: Option<String>,

    /// Limits the number of search results when using the --search/--data-search operation.
    #[clap(short = 'l', long, default_value = "0")]
    limit: usize,

    /// Specifies the programming language for targeted actions (enhances performance).
    #[clap(short = 'L', long)]
    language: Option<String>,

    /// Navigates to the specified path from within the tool.
    #[clap(short = 'g', long)]
    goto: Option<PathBuf>,

    /// Specify the depth of search roots.
    #[clap(long = "root-level", default_value = "3")]
    root_level: usize,

    /// Copies files to the specified output path.
    #[clap(long = "copy")]
    copy: Option<PathBuf>,

    /// Moves files to the specified output path.
    #[clap(long = "move")]
    move_files: Option<PathBuf>,

    /// Specifies the action to perform on the project (e.g., run, build, test).
    /// Use --goto along with this option for complex project structures.
    #[clap(short = 'a', long)]
    action: Option<String>,

    // ========== Environment Variables ==========
    /// List all environment variables
    #[clap(long = "env-list")]
    env_list: bool,

    /// Get specific environment variable
    #[clap(long = "env-get")]
    env_get: Option<String>,

    /// Set environment variable (format: KEY=VALUE)
    #[clap(long = "env-set")]
    env_set: Option<String>,

    /// Load environment variables from file
    #[clap(long = "env-load")]
    env_load: Option<PathBuf>,

    /// Export environment variables to file
    #[clap(long = "env-export")]
    env_export: Option<PathBuf>,

    /// Filter for env export (optional)
    #[clap(long = "env-filter")]
    env_filter: Option<String>,

    // ========== Hashing ==========
    /// Calculate hash of a file
    #[clap(long = "hash-file")]
    hash_file: Option<PathBuf>,

    /// Calculate hash of a string
    #[clap(long = "hash-string")]
    hash_string: Option<String>,

    /// Hash algorithm (md5, sha256, sha512)
    #[clap(long = "hash-algo", default_value = "sha256")]
    hash_algo: String,

    /// Verify file hash
    #[clap(long = "hash-verify")]
    hash_verify: Option<String>,

    /// Calculate all hashes for a file
    #[clap(long = "hash-all")]
    hash_all: Option<PathBuf>,

    // ========== Text Processing ==========
    /// Base64 encode text
    #[clap(long = "base64-encode")]
    base64_encode: Option<String>,

    /// Base64 decode text
    #[clap(long = "base64-decode")]
    base64_decode: Option<String>,

    /// URL encode text
    #[clap(long = "url-encode")]
    url_encode: Option<String>,

    /// URL decode text
    #[clap(long = "url-decode")]
    url_decode: Option<String>,

    /// Convert text to case (upper, lower, title, camel, snake, kebab)
    #[clap(long = "text-case")]
    text_case: Option<String>,

    /// Text to convert
    #[clap(long = "text")]
    text: Option<String>,

    /// Text statistics
    #[clap(long = "text-stats")]
    text_stats: Option<String>,

    // ========== System Information ==========
    /// Display all system information
    #[clap(long = "system-info")]
    system_info: bool,

    /// Display CPU information
    #[clap(long = "cpu-info")]
    cpu_info: bool,

    /// Display memory information
    #[clap(long = "memory-info")]
    memory_info: bool,

    /// Display disk information
    #[clap(long = "disk-info")]
    disk_info: bool,

    /// Display network information
    #[clap(long = "network-info")]
    network_info: bool,

    // ========== Git Operations ==========
    /// Git status
    #[clap(long = "git-status")]
    git_status: bool,

    /// Clone git repository
    #[clap(long = "git-clone")]
    git_clone: Option<String>,

    /// Create git branch
    #[clap(long = "git-branch")]
    git_branch: Option<String>,

    /// Git add all
    #[clap(long = "git-add")]
    git_add: bool,

    /// Git commit with message
    #[clap(long = "git-commit")]
    git_commit: Option<String>,

    /// List git branches
    #[clap(long = "git-branches")]
    git_branches: bool,

    /// Show git log (number of commits)
    #[clap(long = "git-log")]
    git_log: Option<usize>,

    // ========== Archive Operations ==========
    /// Create zip archive
    #[clap(long = "zip-create")]
    zip_create: Option<PathBuf>,

    /// Extract zip archive
    #[clap(long = "zip-extract")]
    zip_extract: Option<PathBuf>,

    /// List zip contents
    #[clap(long = "zip-list")]
    zip_list: Option<PathBuf>,

    /// Create tar.gz archive
    #[clap(long = "tar-create")]
    tar_create: Option<PathBuf>,

    /// Extract tar.gz archive
    #[clap(long = "tar-extract")]
    tar_extract: Option<PathBuf>,

    /// List tar.gz contents
    #[clap(long = "tar-list")]
    tar_list: Option<PathBuf>,

    /// Archive source path
    #[clap(long = "archive-source")]
    archive_source: Option<PathBuf>,

    // ========== Format Operations ==========
    /// Format JSON
    #[clap(long = "json-format")]
    json_format: Option<String>,

    /// Minify JSON
    #[clap(long = "json-minify")]
    json_minify: Option<String>,

    /// Validate JSON
    #[clap(long = "json-validate")]
    json_validate: Option<String>,

    /// Format YAML
    #[clap(long = "yaml-format")]
    yaml_format: Option<String>,

    /// Validate YAML
    #[clap(long = "yaml-validate")]
    yaml_validate: Option<String>,

    /// Convert JSON to YAML
    #[clap(long = "json-to-yaml")]
    json_to_yaml: Option<String>,

    /// Convert YAML to JSON
    #[clap(long = "yaml-to-json")]
    yaml_to_json: Option<String>,

    /// Query JSON path
    #[clap(long = "json-query")]
    json_query: Option<String>,

    /// JSON query path
    #[clap(long = "query-path")]
    query_path: Option<String>,

    // ========== Network Operations ==========
    /// Check if port is open
    #[clap(long = "port-check")]
    port_check: Option<u16>,

    /// Host for port check
    #[clap(long = "host")]
    host: Option<String>,

    /// Ping host
    #[clap(long = "ping")]
    ping: Option<String>,

    /// Get public IP
    #[clap(long = "public-ip")]
    public_ip: bool,

    /// HTTP GET request
    #[clap(long = "http-get")]
    http_get: Option<String>,

    /// HTTP POST request
    #[clap(long = "http-post")]
    http_post: Option<String>,

    /// POST body
    #[clap(long = "post-body")]
    post_body: Option<String>,

    /// DNS lookup
    #[clap(long = "dns-lookup")]
    dns_lookup: Option<String>,

    // ========== Configuration ==========
    /// Show current configuration
    #[clap(long = "config-show")]
    config_show: bool,

    /// Load configuration from file
    #[clap(long = "config-load")]
    config_load: Option<PathBuf>,

    // ========== Web Server ==========
    /// Start the web dashboard server
    #[clap(long = "server-start")]
    server_start: bool,

    /// Port for the web server
    #[clap(long = "server-port", default_value = "3000")]
    server_port: u16,
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
                    let cleaned_language_str = project_type
                        .replace("rust", "cargo")
                        .replace("\"", "")
                        .replace("js", "npm");
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
        if let Err(err) =
            download_video(url, &args.output_path.to_string_lossy(), &args.quality).await
        {
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

    // ========== Environment Variables ==========
    if args.env_list {
        env_module::list_env_vars();
        return;
    }

    if let Some(key) = &args.env_get {
        env_module::get_env_var(key);
        return;
    }

    if let Some(value) = &args.env_set {
        if let Some((key, val)) = value.split_once('=') {
            env_module::set_env_var(key, val);
        } else {
            print_error_message("Error: --env-set requires KEY=VALUE format\n");
        }
        return;
    }

    if let Some(path) = &args.env_load {
        if let Err(e) = env_module::load_env_file(path) {
            print_error_message(&format!("Error loading env file: {}\n", e));
        }
        return;
    }

    if let Some(path) = &args.env_export {
        if let Err(e) = env_module::export_env_vars(path, args.env_filter.as_deref()) {
            print_error_message(&format!("Error exporting env vars: {}\n", e));
        }
        return;
    }

    // ========== Hashing ==========
    if let Some(path) = &args.hash_all {
        hash_module::hash_file_all(path);
        return;
    }

    if let Some(path) = &args.hash_file {
        let algo = match args.hash_algo.to_lowercase().as_str() {
            "md5" => HashAlgorithm::MD5,
            "sha512" => HashAlgorithm::SHA512,
            _ => HashAlgorithm::SHA256,
        };

        if let Some(expected) = &args.hash_verify {
            hash_module::verify_hash(path, expected, algo);
        } else {
            hash_module::display_file_hash(path, algo);
        }
        return;
    }

    if let Some(text) = &args.hash_string {
        let algo = match args.hash_algo.to_lowercase().as_str() {
            "md5" => HashAlgorithm::MD5,
            "sha512" => HashAlgorithm::SHA512,
            _ => HashAlgorithm::SHA256,
        };
        let hash = hash_module::hash_string(text, algo);
        println!("Hash: {}", hash);
        return;
    }

    // ========== Text Processing ==========
    if let Some(text) = &args.base64_encode {
        text_module::base64_encode(text);
        return;
    }

    if let Some(text) = &args.base64_decode {
        text_module::base64_decode(text);
        return;
    }

    if let Some(text) = &args.url_encode {
        text_module::url_encode(text);
        return;
    }

    if let Some(text) = &args.url_decode {
        text_module::url_decode(text);
        return;
    }

    if let Some(case_type) = &args.text_case {
        if let Some(text) = &args.text {
            match case_type.to_lowercase().as_str() {
                "upper" => text_module::to_uppercase(text),
                "lower" => text_module::to_lowercase(text),
                "title" => text_module::to_titlecase(text),
                "camel" => text_module::to_camelcase(text),
                "snake" => text_module::to_snakecase(text),
                "kebab" => text_module::to_kebabcase(text),
                _ => print_error_message(
                    "Error: Invalid case type. Use: upper, lower, title, camel, snake, kebab\n",
                ),
            }
        } else {
            print_error_message("Error: --text is required with --text-case\n");
        }
        return;
    }

    if let Some(text) = &args.text_stats {
        text_module::text_stats(text);
        return;
    }

    // ========== System Information ==========
    if args.system_info {
        system_module::display_system_info();
        return;
    }

    if args.cpu_info {
        system_module::display_cpu_info();
        return;
    }

    if args.memory_info {
        system_module::display_memory_info();
        return;
    }

    if args.disk_info {
        system_module::display_disk_info();
        return;
    }

    if args.network_info {
        system_module::display_network_info();
        return;
    }

    // ========== Git Operations ==========
    if args.git_status {
        git_module::git_status(Path::new("."));
        return;
    }

    if let Some(url) = &args.git_clone {
        git_module::git_clone(url, &args.output_path);
        return;
    }

    if let Some(branch_name) = &args.git_branch {
        git_module::git_create_branch(Path::new("."), branch_name);
        return;
    }

    if args.git_add {
        git_module::git_add_all(Path::new("."));
        return;
    }

    if let Some(message) = &args.git_commit {
        git_module::git_commit(Path::new("."), message);
        return;
    }

    if args.git_branches {
        git_module::git_list_branches(Path::new("."));
        return;
    }

    if let Some(count) = args.git_log {
        git_module::git_log(Path::new("."), count);
        return;
    }

    // ========== Archive Operations ==========
    if let Some(output) = &args.zip_create {
        if let Some(source) = &args.archive_source {
            archive_module::create_zip(source, output);
        } else {
            print_error_message("Error: --archive-source is required with --zip-create\n");
        }
        return;
    }

    if let Some(archive) = &args.zip_extract {
        archive_module::extract_zip(archive, &args.output_path);
        return;
    }

    if let Some(archive) = &args.zip_list {
        archive_module::list_zip_contents(archive);
        return;
    }

    if let Some(output) = &args.tar_create {
        if let Some(source) = &args.archive_source {
            archive_module::create_tar_gz(source, output);
        } else {
            print_error_message("Error: --archive-source is required with --tar-create\n");
        }
        return;
    }

    if let Some(archive) = &args.tar_extract {
        archive_module::extract_tar_gz(archive, &args.output_path);
        return;
    }

    if let Some(archive) = &args.tar_list {
        archive_module::list_tar_gz_contents(archive);
        return;
    }

    // ========== Format Operations ==========
    if let Some(json) = &args.json_format {
        format_module::format_json(json);
        return;
    }

    if let Some(json) = &args.json_minify {
        format_module::minify_json(json);
        return;
    }

    if let Some(json) = &args.json_validate {
        format_module::validate_json(json);
        return;
    }

    if let Some(yaml) = &args.yaml_format {
        format_module::format_yaml(yaml);
        return;
    }

    if let Some(yaml) = &args.yaml_validate {
        format_module::validate_yaml(yaml);
        return;
    }

    if let Some(json) = &args.json_to_yaml {
        format_module::json_to_yaml(json);
        return;
    }

    if let Some(yaml) = &args.yaml_to_json {
        format_module::yaml_to_json(yaml);
        return;
    }

    if let Some(json) = &args.json_query {
        if let Some(path) = &args.query_path {
            format_module::json_query(json, path);
        } else {
            print_error_message("Error: --query-path is required with --json-query\n");
        }
        return;
    }

    // ========== Network Operations ==========
    if let Some(port) = args.port_check {
        let host = args.host.as_deref().unwrap_or("localhost");
        network_module::check_port(host, port);
        return;
    }

    if let Some(host) = &args.ping {
        network_module::ping_host(host);
        return;
    }

    if args.public_ip {
        network_module::get_public_ip();
        return;
    }

    if let Some(url) = &args.http_get {
        network_module::http_get(url);
        return;
    }

    if let Some(url) = &args.http_post {
        let body = args.post_body.as_deref().unwrap_or("");
        network_module::http_post(url, body, None);
        return;
    }

    if let Some(domain) = &args.dns_lookup {
        network_module::dns_lookup(domain);
        return;
    }

    // ========== Configuration ==========
    if args.config_show {
        match config::Config::load() {
            Ok(cfg) => cfg.display(),
            Err(e) => print_error_message(&format!("Error loading config: {}\n", e)),
        }
        return;
    }

    if let Some(path) = &args.config_load {
        match config::Config::load_from_file(path) {
            Ok(cfg) => {
                println!("Configuration loaded from: {}", path.display());
                cfg.display();
            }
            Err(e) => print_error_message(&format!("Error loading config: {}\n", e)),
        }
        return;
    }

    // ========== Web Server ==========
    if args.server_start {
        if let Err(e) = server_module::start_server(args.server_port).await {
            print_error_message(&format!("Server error: {}\n", e));
        }
        return;
    }

    // Final check - update to include new features
    if args.name.is_none()
        && args.url.is_none()
        && args.search.is_none()
        && args.copy.is_none()
        && args.move_files.is_none()
        && args.language.is_none()
        && args.action.is_none()
        && args.goto.is_none()
        && args.data_search.is_none()
        && !args.env_list
        && !args.system_info
        && !args.git_status
        && !args.public_ip
        && !args.config_show
        && !args.server_start
    {
        print_error_message(
            "Error: No operation specified. Use --help to see available options.\n",
        );
    }
}
