use std::process::{Command, exit};
use std::io;
use crate::print_error_message;

pub fn execute_language_action(language: &str, action: &str) {
    let language_actions = map_language_actions();

    if let Some(actions) = language_actions.get(language) {
        if actions.contains(&action) {
            match (language, action) {
                ("java", "run") => execute_command("java", &["-jar", &input("Enter Java program JAR path: ")]),
                ("java", "build") => execute_command("javac", &[&input("Enter Java program source file path: ")]),
                ("java", "test") => execute_command("junit", &[&input("Enter Java test file path: ")]),
                ("python", "run") => execute_command("python", &[&input("Enter Python program file path: ")]),
                ("python", "test") => execute_command("pytest", &[&input("Enter Python test file path: ")]),
                ("python", "clean") => execute_command("rm", &["-rf", "build"]),
                ("dotnet", "run") => execute_command("dotnet", &["run"]),
                ("dotnet", "build") => execute_command("dotnet", &["build"]),
                ("dotnet", "clean") => execute_command("dotnet", &["clean"]),
                ("cargo", "run") => execute_command("cargo", &["run"]),
                ("cargo", "build") => execute_command("cargo", &["build"]),
                ("cargo", "clean") => execute_command("cargo", &["clean"]),
                ("npm", "install") => execute_command("npm", &["install"]),
                ("npm", "test") => execute_command("npm", &["test"]),
                ("npm", "clean") => execute_command("rm", &["-rf", "node_modules"]),
                ("java", "install") => execute_command("mvn", &["install"]),
                ("java", "clean") => execute_command("mvn", &["clean"]),
                ("python", "install") => execute_command("pip", &["install", &input("Enter Python package name: ")]),
                ("python", "remove") => execute_command("pip", &["uninstall", &input("Enter Python package name to remove: ")]),
                ("nuget", "install") => execute_command("nuget", &["install", &input("Enter NuGet package name: ")]),
                ("nuget", "remove") => execute_command("nuget", &["uninstall", &input("Enter NuGet package name to remove: ")]),
                _ => println!("Unsupported action {} for language {}", action, language),
            }
        } else {
            println!("Invalid action {} for language {}. Supported actions: {:?}", action, language, actions);
        }
    } else {
        println!("Unsupported language: {}", language);
    }
}

fn execute_command(command: &str, args: &[&str]) {
    let status = Command::new(command)
        .args(args)
        .status();

    match status {
        Ok(exit_status) => {
            if !exit_status.success() {
                let error_message = format!("Error executing command: {} {:?}", command, args);
                print_error_message(&error_message);
                exit(exit_status.code().unwrap_or(1));
            }
        }
        Err(err) => {
            let error_message = format!("Error executing command: {}", err);
            print_error_message(&error_message);
            exit(1);
        }
    }
}

fn map_language_actions() -> std::collections::HashMap<&'static str, Vec<&'static str>> {
    let mut map = std::collections::HashMap::new();

    map.insert("java", vec!["run", "build", "test", "clean"]);
    map.insert("python", vec!["run", "test", "clean"]);
    map.insert("dotnet", vec!["run", "build", "clean"]);
    map.insert("cargo", vec!["run", "build", "clean"]);
    map.insert("npm", vec!["install", "test", "clean"]);
    map.insert("mvn", vec!["install", "clean"]);
    map.insert("pip", vec!["install", "remove"]);
    map.insert("nuget", vec!["install", "remove"]);

    map
}

fn input(prompt: &str) -> String {
    eprint!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_valid_command() {
        let result = execute_language_action("cargo", "runs");
        assert_eq!(result, ());
    }

    #[test]
    fn test_execute_invalid_action() {
        let result = execute_language_action("python", "invalid_action");
        assert_eq!(result, ());
    }

    #[test]
    fn test_execute_invalid_language() {
        let result = execute_language_action("invalid_language", "run");
        assert_eq!(result, ());
    }
}
