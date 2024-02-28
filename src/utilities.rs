use std::io::{self, Write};
use colored::Colorize;

#[allow(dead_code)]
pub fn print_hyperlink(path: &std::path::Path) {
    if let Some(path_str) = path.to_str() {
        let hyperlink = format!("\x1B]8;;{}\x07{}\x1B]8;;\x07", path_str, path_str);
        io::stdout().write_all(hyperlink.as_bytes()).expect("Failed to write hyperlink to stdout");
        println!(); // Move to the next line after the hyperlink
    } else {
        eprintln!("Error converting path to string");
    }
}

pub fn print_colored_path(path: &std::path::Path) {
    let path_str = path.display().to_string();
    let colored_path = if path.is_file() {
        path_str.green().to_string()
    } else if path.is_dir() {
        path_str.blue().to_string()
    } else {
        path_str
    };

    println!("{}", colored_path);
}
