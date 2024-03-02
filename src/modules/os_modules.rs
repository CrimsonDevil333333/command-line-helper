use globwalk::GlobWalkerBuilder;
use log::{trace, warn};
use std::fs::{copy, rename};
use std::io;
use std::path::{Path, PathBuf};

use crate::{print_colored_path, print_error_message};

// Function to copy a file
pub fn copy_file(src: &PathBuf, dest: &Path, custom_name: &Option<String>) -> io::Result<()> {
    let src_str = src.to_str().expect("Invalid source file path");
    let dest_file_name = custom_name.as_deref().unwrap_or_else(|| {
        Path::new(src_str)
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    });
    let dest_path = dest.join(dest_file_name);

    if let Err(e) = copy(src, &dest_path) {
        let error_message = format!(
            "Error copying file: {} to {}: {}",
            src.display(),
            dest_path.display(),
            e
        );
        print_error_message(&error_message);
        warn!("{}", error_message);
        return Err(e);
    }

    println!(
        "File copied from {} to {}",
        src.display(),
        dest_path.display()
    );
    Ok(())
}

// Function to move a file
pub fn move_file(src: &PathBuf, dest: &Path, custom_name: &Option<String>) -> io::Result<()> {
    let src_str = src.to_str().expect("Invalid source file path");
    let dest_file_name = custom_name.as_deref().unwrap_or_else(|| {
        Path::new(src_str)
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    });
    let dest_path = dest.join(dest_file_name);

    if let Err(e) = rename(src, &dest_path) {
        let error_message = format!(
            "Error moving file: {} to {}: {}",
            src.display(),
            dest_path.display(),
            e
        );
        print_error_message(&error_message);
        warn!("{}", error_message);
        return Err(e);
    }

    println!(
        "File moved from {} to {}",
        src.display(),
        dest_path.display()
    );
    Ok(())
}

// Function to search for files
pub fn search_files(pattern: &String, s_path: &PathBuf, size: &usize) {
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
                trace!("Found file: {}", entry.path().display());
            }
            Err(e) => {
                let error_message = format!("Error during file search: {}", e);
                print_error_message(&error_message);
                warn!("{}", error_message);
            }
        }
    }
}
