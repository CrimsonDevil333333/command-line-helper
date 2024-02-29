use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::fs::{copy, rename};
use globwalk::GlobWalkerBuilder;


use crate::print_colored_path;
use crate::print_error_message;

// Function to copy a file
pub fn copy_file(src: &PathBuf, dest: &Path, custom_name: &Option<String>) -> io::Result<()> {
    let src_str = src.to_str().expect("Invalid source file path");
    let dest_file_name = custom_name.as_deref().unwrap_or_else(|| Path::new(src_str).file_name().unwrap_or_default().to_str().unwrap_or_default());
    let dest_path = dest.join(dest_file_name);
    copy(src, &dest_path)?;
    println!("File copied to: {}", dest_path.display());
    Ok(())
}

// Function to move a file
pub fn move_file(src: &PathBuf, dest: &Path, custom_name: &Option<String>) -> io::Result<()> {
    let src_str = src.to_str().expect("Invalid source file path");
    let dest_file_name = custom_name.as_deref().unwrap_or_else(|| Path::new(src_str).file_name().unwrap_or_default().to_str().unwrap_or_default());
    let dest_path = dest.join(dest_file_name);
    rename(src, &dest_path)?;
    println!("File moved to: {}", dest_path.display());
    Ok(())
}

// Function to search a file
pub fn search_files(pattern: &String,s_path: &PathBuf, size: &usize) {
        let search_path = PathBuf::from(s_path);
        let walker = GlobWalkerBuilder::from_patterns(search_path, &[pattern])
            .max_depth(if *size > 0 { *size } else { usize::MAX })
            .build()
            .unwrap();

        let mut count = 0;
        for entry in walker {
            match entry {
                Ok(entry) => {
                    if *size > 0 && count >= *size {
                        break;
                    }
                    print_colored_path(entry.path());
                    // println!("{}", entry.path().display());
                    count += 1;
                }
                Err(e) => print_error_message(&format!("Error: {}\n", e)),
            }
        }
}