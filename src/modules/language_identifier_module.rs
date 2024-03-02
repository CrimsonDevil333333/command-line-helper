use log::{info, warn};
use std::fs;

pub fn identify_project_type(path: &str) -> Result<&'static str, &'static str> {
    let project_types = ["rust", "java", "python", "js", "dotnet"];

    let file_name = path.rsplit('/').next().ok_or("Invalid path")?;

    for &project_type in &project_types {
        if file_name.contains(project_type) || contains_project_files(path, project_type) {
            info!("Identified project type: {}", project_type);
            return Ok(project_type);
        }
    }

    warn!("Unknown project type");
    Err("Unknown project type")
}

fn contains_project_files(path: &str, project_type: &str) -> bool {
    match project_type {
        "rust" => is_rust_project(path),
        "java" => is_java_project(path),
        "python" => is_python_project(path),
        "js" => is_js_project(path),
        "dotnet" => is_dotnet_project(path),
        _ => false,
    }
}

fn is_rust_project(path: &str) -> bool {
    // Check for common Rust project files or folders
    let result = file_exists(path, "Cargo.toml");
    info!("Checking for Rust project: {} - Result: {}", path, result);
    result
}

fn is_java_project(path: &str) -> bool {
    // Check for common Java project files or folders
    let result = file_exists(path, "pom.xml")
        || file_exists(path, "build.gradle")
        || file_exists(path, "build.gradle.kts");

    info!("Checking for Java project: {} - Result: {}", path, result);
    result
}

fn is_python_project(path: &str) -> bool {
    // Check for common Python project files or folders
    let result = file_exists(path, "requirements.txt")
        || file_exists(path, "main.py")
        || file_exists(path, "config.py");

    info!("Checking for Python project: {} - Result: {}", path, result);
    result
}

fn is_js_project(path: &str) -> bool {
    // Check for common JavaScript project files or folders
    let result = file_exists(path, "package.json");
    info!(
        "Checking for JavaScript project: {} - Result: {}",
        path, result
    );
    result
}

fn is_dotnet_project(path: &str) -> bool {
    // Check for common .NET project files or folders
    let result = file_exists(path, "*.csproj") || file_exists(path, "*.sln");
    info!("Checking for .NET project: {} - Result: {}", path, result);
    result
}

fn file_exists(path: &str, file_name: &str) -> bool {
    let file_path = format!("{}/{}", path, file_name);
    let result = fs::metadata(&file_path).is_ok();
    info!(
        "Checking file existence: {} - Result: {}",
        file_path, result
    );
    result
}

// Uncomment the following function if needed
// fn directory_exists(path: &str, dir_name: &str) -> bool {
//     fs::read_dir(path)
//         .map(|mut entries| entries.any(|entry| entry.unwrap().file_name() == dir_name))
//         .unwrap_or_else(|e| {
//             eprintln!("Error checking directory existence: {}", e);
//             false
//         })
// }
