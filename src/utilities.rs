use log::info;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// Helper function to clean language string
pub fn clean_language_string(language: &String) -> String {
    let cleaned_language = format!("{:?}", language)
        .to_lowercase()
        .replace("rust", "cargo")
        .replace("js", "npm")
        .replace("\"", "");

    info!("Cleaned language string: {}", cleaned_language);

    cleaned_language
}

// Helper function to clean action string
pub fn clean_action_string(action: &String) -> String {
    let cleaned_action = format!("{:?}", action).to_lowercase().replace("\"", "");

    info!("Cleaned action string: {}", cleaned_action);

    cleaned_action
}

#[allow(dead_code)]
pub fn print_hyperlink(path: &std::path::Path) {
    if let Some(path_str) = path.to_str() {
        let hyperlink = format!("\x1B]8;;{}\x07{}\x1B]8;;\x07", path_str, path_str);
        io::stdout()
            .write_all(hyperlink.as_bytes())
            .expect("Failed to write hyperlink to stdout");
        println!(); // Move to the next line after the hyperlink
    } else {
        print_error_message("Error converting path to string\n");
    }
}

pub fn get_current_os() -> &'static str {
    let os_info = std::env::consts::OS;

    info!("Current OS: {}", os_info);

    os_info
}

pub fn is_language_installed(language: &str) -> bool {
    let status = if cfg!(windows) && language == "mvn" || language == "npm" {
        // On Windows, check if the command has a `.cmd` suffix for specific commands
        let cmd = format!("{}.cmd", language);
        if which::which(&cmd).is_ok() {
            // Use the command with `.cmd` suffix if found
            Command::new(cmd)
        } else {
            // Fall back to the original command if `.cmd` version not found
            Command::new(language)
        }
    } else {
        Command::new(language)
    }
    .arg("--version")
    .stdout(Stdio::null()) // Redirect standard output to null
    .stderr(Stdio::null()) // Redirect standard error to null
    .status();

    if let Ok(status) = status {
        let is_installed = status.success();
        info!("Is {} installed: {}", language, is_installed);
        is_installed
    } else {
        let error_message = format!("Failed to execute command {}", language);
        print_error_message(&error_message);
        false
    }
}

pub fn suggest_installation(language: &str) {
    // Use the enhanced installer from installer module
    crate::installer::suggest_installation_enhanced(language);
}

#[allow(dead_code)]
pub fn validate_and_suggest_installation(language: &str) {
    if is_language_installed(language) {
        println!("{} is already installed.", language);
    } else {
        suggest_installation(language);
    }
}

#[allow(dead_code)]
pub fn get_system_path() -> Option<String> {
    match env::var_os("PATH") {
        Some(paths) => {
            if let Some(paths_str) = paths.to_str() {
                Some(paths_str.to_string())
            } else {
                None
            }
        }
        None => None,
    }
}

#[allow(dead_code)]
pub fn load_system_paths() -> Vec<PathBuf> {
    // Retrieve the system PATH
    if let Some(system_path) = env::var_os("PATH") {
        // Split the system PATH into individual paths
        let paths: Vec<PathBuf> = env::split_paths(&system_path).collect();

        info!("Loaded system paths: {:?}", paths);

        paths
    } else {
        eprintln!("Failed to retrieve system PATH.");
        Vec::new()
    }
}

#[allow(dead_code)]
pub fn update_path_with_current_exe() {
    // Get the current executable's path
    if let Ok(current_exe) = env::current_exe() {
        // Append the parent directory of the executable to the system PATH
        if let Some(parent_dir) = current_exe.parent() {
            let mut program_paths = load_system_paths();
            program_paths.push(parent_dir.to_path_buf());

            // Join and update the PATH environment variable
            let new_path = env::join_paths(program_paths).expect("Failed to join paths");
            env::set_var("PATH", new_path);
        } else {
            eprintln!("Failed to get the parent directory of the current executable.");
        }
    } else {
        eprintln!("Failed to get the path of the current executable.");
    }
}

pub fn print_colored_path(path: &std::path::Path) {
    let path_str = path.display().to_string();
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    if path.is_file() {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            .unwrap();
    } else if path.is_dir() {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
            .unwrap();
    }

    write!(stdout, "{}", path_str).unwrap();
    stdout.reset().unwrap();
    println!();
}

pub fn print_error_message(message: &str) {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    stderr
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    write!(stderr, "{}", message).unwrap();
    stderr.reset().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_info() {
        assert_eq!(get_current_os(), std::env::consts::OS);
    }

    #[test]
    fn test_validate_and_suggest_installation() {
        // Test with an installed language
        println!("Testing with an installed language...");
        validate_and_suggest_installation("python");
    }
}
