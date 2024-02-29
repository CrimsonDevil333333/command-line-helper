use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[allow(dead_code)]
pub fn print_hyperlink(path: &std::path::Path) {
    if let Some(path_str) = path.to_str() {
        let hyperlink = format!("\x1B]8;;{}\x07{}\x1B]8;;\x07", path_str, path_str);
        io::stdout().write_all(hyperlink.as_bytes()).expect("Failed to write hyperlink to stdout");
        println!(); // Move to the next line after the hyperlink
    } else {
        print_error_message("Error converting path to string\n");
    }
}

pub fn print_colored_path(path: &std::path::Path) {
    let path_str = path.display().to_string();
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    
    if path.is_file() {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    } else if path.is_dir() {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).unwrap();
    }
    
    write!(stdout, "{}", path_str).unwrap();
    stdout.reset().unwrap();
    println!();
}

pub fn print_error_message(message: &str) {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
    write!(stderr, "{}", message).unwrap();
    stderr.reset().unwrap();
}
