use colored::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn search_data_in_files(data: &str, output_path: &PathBuf, root_level: usize, limit: usize) {
    let mut results = Vec::new();
    search_files_recursive(data, output_path, root_level, &mut results);
    if results.is_empty() {
        let mut stderr = StandardStream::stderr(ColorChoice::Auto);
        stderr
            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
            .unwrap();
        write!(stderr, "No results found for your input '{}'", data).unwrap();
        stderr.reset().unwrap();
        writeln!(stderr).unwrap();
        return;
    }

    for (file_path, lines) in results {
        if limit == 0 {
            for (line_number, line) in lines {
                print_colored_line(&file_path, line_number, &line, &data);
            }
        } else {
            for (line_number, line) in lines.iter().take(limit) {
                print_colored_line(&file_path, *line_number, line, &data);
            }
        }
    }
}

// Old way of printing colors (High performance but less relaiblity)
// fn print_colored_lines(file_path: &PathBuf, line_number: usize, line: &str, data: &str) {
//     let colored_file_path = get_colored_path(file_path);
//     let colored_line_number = format!("{: <4}", line_number).blue().to_string();
//     let colored_line = line.replace(data, &data.yellow().to_string());
//     println!(
//         "{} : {} : {}",
//         colored_file_path,
//         colored_line_number,
//         colored_line.trim()
//     );
// }

fn print_colored_line(file_path: &PathBuf, line_number: usize, line: &str, data: &str) {
    let colored_file_path = get_colored_path(file_path);
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    // Set color for file path
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
        .unwrap();
    write!(stdout, "{}", colored_file_path).unwrap();
    stdout.reset().unwrap();

    write!(stdout, " : ").unwrap(); // Separator

    // Set color for line number (blue)
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
        .unwrap();
    write!(stdout, "{: <4}", line_number).unwrap();
    stdout.reset().unwrap();

    write!(stdout, " : ").unwrap(); // Separator

    // Set color for data (yellow)
    let colored_line = line.replace(data, &format!("{}", &data.color("yellow")));
    write!(stdout, "{}", colored_line.trim()).unwrap();
    stdout.reset().unwrap();

    writeln!(stdout).unwrap(); // Newline
}

fn get_colored_path(path: &PathBuf) -> String {
    let path_str = path.display().to_string();
    if path.is_file() {
        path_str.green().to_string()
    } else if path.is_dir() {
        path_str.blue().to_string()
    } else {
        path_str
    }
}
fn search_files_recursive(
    data: &str,
    current_path: &PathBuf,
    remaining_levels: usize,
    results: &mut Vec<(PathBuf, Vec<(usize, String)>)>,
) {
    if let Ok(entries) = fs::read_dir(current_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        let mut matching_lines = Vec::new();
                        for (line_number, line) in contents.lines().enumerate() {
                            if line.contains(data) {
                                matching_lines.push((line_number + 1, line.to_string()));
                            }
                        }
                        if !matching_lines.is_empty() {
                            results.push((path.clone(), matching_lines));
                        }
                    }
                } else if path.is_dir() && remaining_levels > 0 {
                    search_files_recursive(data, &path, remaining_levels - 1, results);
                }
            }
        }
    }
}
