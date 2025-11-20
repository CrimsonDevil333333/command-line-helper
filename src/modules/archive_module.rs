use colored::Colorize;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use tar::Archive;
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

/// Create a zip archive from a directory or file
pub fn create_zip(source: &Path, output: &Path) -> io::Result<()> {
    let file = File::create(output)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    if source.is_file() {
        let name = source.file_name().unwrap().to_string_lossy();
        zip.start_file(name.as_ref(), options)?;
        let mut f = File::open(source)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        zip.write_all(&buffer)?;
        println!("{} Added: {}", "✓".green(), name.yellow());
    } else if source.is_dir() {
        let walkdir = WalkDir::new(source);
        let it = walkdir.into_iter().filter_map(|e| e.ok());

        for entry in it {
            let path = entry.path();
            let name = path.strip_prefix(source).unwrap();

            if path.is_file() {
                zip.start_file(name.to_string_lossy().as_ref(), options)?;
                let mut f = File::open(path)?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
                println!(
                    "{} Added: {}",
                    "✓".green(),
                    name.display().to_string().yellow()
                );
            } else if !name.as_os_str().is_empty() {
                zip.add_directory(name.to_string_lossy().as_ref(), options)?;
            }
        }
    }

    zip.finish()?;
    println!(
        "\n{} Created archive: {}",
        "Success:".green().bold(),
        output.display()
    );
    Ok(())
}

/// Extract a zip archive
pub fn extract_zip(archive_path: &Path, output_dir: &Path) -> io::Result<()> {
    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;

    fs::create_dir_all(output_dir)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => output_dir.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
            println!(
                "{} Extracted: {}",
                "✓".green(),
                outpath.display().to_string().yellow()
            );
        }
    }

    println!(
        "\n{} Extracted to: {}",
        "Success:".green().bold(),
        output_dir.display()
    );
    Ok(())
}

/// Create a tar.gz archive
pub fn create_tar_gz(source: &Path, output: &Path) -> io::Result<()> {
    let tar_gz = File::create(output)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    if source.is_file() {
        tar.append_path_with_name(source, source.file_name().unwrap())?;
        println!(
            "{} Added: {}",
            "✓".green(),
            source.file_name().unwrap().to_string_lossy().yellow()
        );
    } else if source.is_dir() {
        tar.append_dir_all(".", source)?;
        println!(
            "{} Added directory: {}",
            "✓".green(),
            source.display().to_string().yellow()
        );
    }

    tar.finish()?;
    println!(
        "\n{} Created archive: {}",
        "Success:".green().bold(),
        output.display()
    );
    Ok(())
}

/// Extract a tar.gz archive
pub fn extract_tar_gz(archive_path: &Path, output_dir: &Path) -> io::Result<()> {
    let tar_gz = File::open(archive_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    fs::create_dir_all(output_dir)?;
    archive.unpack(output_dir)?;

    println!(
        "{} Extracted to: {}",
        "Success:".green().bold(),
        output_dir.display()
    );
    Ok(())
}

/// List contents of a zip archive
pub fn list_zip_contents(archive_path: &Path) -> io::Result<()> {
    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;

    println!("{}", "Archive Contents:".cyan().bold());
    println!("{}", "=".repeat(80).cyan());
    println!("{:60} {:>15}", "Name".yellow(), "Size".yellow());
    println!("{}", "-".repeat(80));

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let size = file.size();
        let name = file.name();

        if name.ends_with('/') {
            println!("{:60} {:>15}", name.cyan(), "-");
        } else {
            println!("{:60} {:>15}", name.green(), format_size(size));
        }
    }

    println!("\n{} Total files: {}", "Info:".cyan().bold(), archive.len());
    Ok(())
}

/// List contents of a tar.gz archive
pub fn list_tar_gz_contents(archive_path: &Path) -> io::Result<()> {
    let tar_gz = File::open(archive_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    println!("{}", "Archive Contents:".cyan().bold());
    println!("{}", "=".repeat(80).cyan());
    println!("{:60} {:>15}", "Name".yellow(), "Size".yellow());
    println!("{}", "-".repeat(80));

    let mut count = 0;
    for entry in archive.entries()? {
        let entry = entry?;
        let path = entry.path()?;
        let size = entry.size();

        if path.to_string_lossy().ends_with('/') {
            println!("{:60} {:>15}", path.display().to_string().cyan(), "-");
        } else {
            println!(
                "{:60} {:>15}",
                path.display().to_string().green(),
                format_size(size)
            );
        }
        count += 1;
    }

    println!("\n{} Total entries: {}", "Info:".cyan().bold(), count);
    Ok(())
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
