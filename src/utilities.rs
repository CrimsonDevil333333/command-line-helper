use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// Helper function to clean language string
pub fn clean_language_string(language: &String) -> String {
    format!("{:?}", language)
        .to_lowercase()
        .replace("rust", "cargo")
        .replace("js", "npm")
        .replace("\"", "")
}

// Helper function to clean action string
pub fn clean_action_string(action: &String) -> String {
    format!("{:?}", action).to_lowercase().replace("\"", "")
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
    std::env::consts::OS
}

pub fn is_language_installed(language: &str) -> bool {
    let status = Command::new(language)
        .arg("--version")
        .stdout(Stdio::null()) // Redirect standard output to null
        .stderr(Stdio::null()) // Redirect standard error to null
        .status();

    if let Ok(status) = status {
        status.success()
    } else {
        print_error_message(&format!("Failed to execute command {}\n", language));
        false
    }
}

pub fn suggest_installation(language: &str) {
    let os_info = get_current_os();

    print_error_message(&format!("{} is not installed on {}.\n", language, os_info));

    // Add installation suggestions based on the OS
    match os_info {
        "linux" => println!(
            "Consider using your package manager to install {}.",
            language
        ),
        "macos" => println!("Consider using Homebrew to install {}.", language),
        "windows" => println!(
            "Consider downloading {} from the official website.",
            language
        ),
        _ => println!(
            "Unable to provide installation suggestions for {} on {}.",
            language, os_info
        ),
    }
}

#[allow(dead_code)]
pub fn validate_and_suggest_installation(language: &str) {
    if is_language_installed(language) {
        println!("{} is already installed.", language);
    } else {
        let os_info = get_current_os();
        print_error_message(&format!("{} is not installed on {}.\n", language, os_info));

        // Add installation suggestions based on the OS
        match os_info {
            "linux" => println!(
                "Consider using your package manager to install {}.",
                language
            ),
            "macos" => println!("Consider using Homebrew to install {}.", language),
            "windows" => println!(
                "Consider downloading {} from the official website.",
                language
            ),
            _ => println!(
                "Unable to provide installation suggestions for {} on {}.",
                language, os_info
            ),
        }
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
        env::split_paths(&system_path).collect()
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
    fn test_is_language_installed() {
        // Test with an installed language
        assert_eq!(is_language_installed("rustc"), true);

        // Test with a non-existent language
        assert_eq!(is_language_installed("npm"), true);
    }

    #[test]
    fn test_os_info() {
        assert_eq!(get_current_os(), "windows");
    }

    #[test]
    fn test_validate_and_suggest_installation() {
        // Test with an installed language
        println!("Testing with an installed language...");
        validate_and_suggest_installation("python");

        // Test with a nonexistent language
        println!("Testing with a nonexistent language...");
        validate_and_suggest_installation("nonexistent_language");
    }
}
