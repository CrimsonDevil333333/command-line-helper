use colored::Colorize;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// List all environment variables
pub fn list_env_vars() {
    println!("{}", "Environment Variables:".cyan().bold());
    println!("{}", "=".repeat(80).cyan());

    let mut vars: Vec<_> = env::vars().collect();
    vars.sort_by(|a, b| a.0.cmp(&b.0));

    for (key, value) in vars {
        println!("{} = {}", key.green(), value.yellow());
    }
}

/// Get a specific environment variable
pub fn get_env_var(key: &str) {
    match env::var(key) {
        Ok(value) => {
            println!("{} = {}", key.green(), value.yellow());
        }
        Err(_) => {
            eprintln!(
                "{} Environment variable '{}' not found",
                "Error:".red().bold(),
                key
            );
        }
    }
}

/// Load environment variables from a .env file
pub fn load_env_file(file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;

    let mut count = 0;
    for line in content.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse KEY=VALUE format
        if let Some(pos) = line.find('=') {
            let key = line[..pos].trim();
            let value = line[pos + 1..].trim();

            // Remove quotes if present
            let value = value.trim_matches('"').trim_matches('\'');

            env::set_var(key, value);
            println!(
                "{} Loaded: {} = {}",
                "✓".green(),
                key.cyan(),
                value.yellow()
            );
            count += 1;
        }
    }

    println!(
        "\n{} Loaded {} environment variables from {}",
        "Success:".green().bold(),
        count,
        file_path.display()
    );
    Ok(())
}

/// Export environment variables to a file
pub fn export_env_vars(
    file_path: &PathBuf,
    filter: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::create(file_path)?;

    let mut vars: Vec<_> = env::vars().collect();
    vars.sort_by(|a, b| a.0.cmp(&b.0));

    let mut count = 0;
    for (key, value) in vars {
        // Apply filter if provided
        if let Some(filter_str) = filter {
            if !key.to_lowercase().contains(&filter_str.to_lowercase()) {
                continue;
            }
        }

        // Escape quotes in value
        let escaped_value = value.replace('"', "\\\"");
        writeln!(file, "{}=\"{}\"", key, escaped_value)?;
        count += 1;
    }

    println!(
        "{} Exported {} environment variables to {}",
        "Success:".green().bold(),
        count,
        file_path.display()
    );
    Ok(())
}

/// Set an environment variable (session only)
pub fn set_env_var(key: &str, value: &str) {
    env::set_var(key, value);
    println!("{} Set {} = {}", "✓".green(), key.cyan(), value.yellow());
    println!(
        "{} This change is only for the current session",
        "Note:".yellow().bold()
    );
}
