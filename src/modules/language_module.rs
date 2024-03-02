use crate::print_error_message;
use log::warn;
use std::io;
use std::process::{exit, Command};

pub fn execute_language_action(language: &str, action: &str) {
    let language_actions = map_language_actions();

    if let Some(actions) = language_actions.get(language) {
        if actions.contains(&action) {
            match (language, action) {
                // Java commands
                ("java", "run") => {
                    execute_command("java", &["-jar", &input("Enter Java program JAR path: ")])
                }
                ("java", "build") => {
                    execute_command("javac", &[&input("Enter Java program source file path: ")])
                }
                ("java", "test") => {
                    execute_command("junit", &[&input("Enter Java test file path: ")])
                }
                ("java", "install") => execute_command("mvn", &["install"]),
                ("java", "clean") => execute_command("mvn", &["clean"]),

                // Python commands
                ("python", "run") => {
                    execute_command("python", &[&input("Enter Python program file path: ")])
                }
                ("python", "test") => {
                    execute_command("pytest", &[&input("Enter Python test file path: ")])
                }
                ("python", "install") => {
                    execute_command("pip", &["install", &input("Enter Python package name: ")])
                }
                ("python", "remove") => execute_command(
                    "pip",
                    &["uninstall", &input("Enter Python package name to remove: ")],
                ),
                ("python", "clean") => execute_command("rm", &["-rf", "__pycache__"]),

                // .NET commands
                ("dotnet", "run") => execute_command("dotnet", &["run"]),
                ("dotnet", "build") => execute_command("dotnet", &["build"]),
                ("dotnet", "clean") => execute_command("dotnet", &["clean"]),
                ("nuget", "install") => {
                    execute_command("nuget", &["install", &input("Enter NuGet package name: ")])
                }
                ("nuget", "remove") => execute_command(
                    "nuget",
                    &["uninstall", &input("Enter NuGet package name to remove: ")],
                ),

                // Rust commands
                ("cargo", "run") => execute_command("cargo", &["run"]),
                ("cargo", "build") => execute_command("cargo", &["build"]),
                ("cargo", "clean") => execute_command("cargo", &["clean"]),

                // Additional cargo commands
                ("cargo", "test") => execute_command("cargo", &["test"]),
                ("cargo", "doc") => execute_command("cargo", &["doc"]),
                ("cargo", "format") => execute_command("cargo", &["fmt"]),
                ("cargo", "check") => execute_command("cargo", &["check"]),
                ("cargo", "update") => execute_command("cargo", &["update"]),

                ("npm", "install") => execute_command("npm", &["install"]),
                ("npm", "run") => execute_command("npm", &["start"]),
                ("npm", "test") => execute_command("npm", &["test"]),
                ("npm", "clean") => execute_command("rm", &["-rf", "node_modules"]),

                // Additional npm commands
                ("npm", "build") => execute_command("npm", &["run", "build"]),
                ("npm", "publish") => execute_command("npm", &["publish"]),
                ("npm", "update") => execute_command("npm", &["update"]),
                _ => {
                    warn!("Unsupported action {} for language {}", action, language);
                    println!("Unsupported action {} for language {}", action, language);
                }
            }
        } else {
            println!(
                "Invalid action {} for language {}. Supported actions: {:?}",
                action, language, actions
            );
        }
    } else {
        warn!("Unsupported language: {}", language);
        println!("Unsupported language: {}", language);
    }
}

fn execute_command(command: &str, args: &[&str]) {
    let status = Command::new(command).args(args).status();

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

    map.insert("java", vec!["run", "build", "test", "install", "clean"]);
    map.insert("python", vec!["run", "test", "install", "remove", "clean"]);
    map.insert("dotnet", vec!["run", "build", "clean", "install", "remove"]);
    map.insert(
        "cargo",
        vec![
            "run", "build", "clean", "test", "doc", "format", "check", "update",
        ],
    );
    map.insert(
        "npm",
        vec![
            "install", "run", "test", "clean", "build", "publish", "update",
        ],
    );
    map
}

fn input(prompt: &str) -> String {
    eprint!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_valid_command() {
        // Test with a valid language and action
        let result = execute_language_action("cargo", "run");
        assert_eq!(result, ());
    }

    #[test]
    fn test_execute_invalid_action() {
        // Test with a valid language but an invalid action
        let result = execute_language_action("python", "invalid_action");
        assert_eq!(result, ());
    }

    #[test]
    fn test_execute_invalid_language() {
        // Test with an invalid language
        let result = execute_language_action("invalid_language", "run");
        assert_eq!(result, ());
    }
}
