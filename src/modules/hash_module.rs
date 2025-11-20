use colored::Colorize;
use md5::{Digest as Md5Digest, Md5};
use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    MD5,
    SHA256,
    SHA512,
}

impl HashAlgorithm {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "md5" => Some(HashAlgorithm::MD5),
            "sha256" => Some(HashAlgorithm::SHA256),
            "sha512" => Some(HashAlgorithm::SHA512),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            HashAlgorithm::MD5 => "MD5",
            HashAlgorithm::SHA256 => "SHA256",
            HashAlgorithm::SHA512 => "SHA512",
        }
    }
}

/// Calculate hash of a file
pub fn hash_file(file_path: &PathBuf, algorithm: HashAlgorithm) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let hash = match algorithm {
        HashAlgorithm::MD5 => {
            let mut hasher = Md5::new();
            hasher.update(&buffer);
            format!("{:x}", hasher.finalize())
        }
        HashAlgorithm::SHA256 => {
            let mut hasher = Sha256::new();
            hasher.update(&buffer);
            format!("{:x}", hasher.finalize())
        }
        HashAlgorithm::SHA512 => {
            let mut hasher = Sha512::new();
            hasher.update(&buffer);
            format!("{:x}", hasher.finalize())
        }
    };

    Ok(hash)
}

/// Calculate hash of a string
pub fn hash_string(input: &str, algorithm: HashAlgorithm) -> String {
    match algorithm {
        HashAlgorithm::MD5 => {
            let mut hasher = Md5::new();
            hasher.update(input.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        HashAlgorithm::SHA256 => {
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        HashAlgorithm::SHA512 => {
            let mut hasher = Sha512::new();
            hasher.update(input.as_bytes());
            format!("{:x}", hasher.finalize())
        }
    }
}

/// Display hash for a file
pub fn display_file_hash(file_path: &PathBuf, algorithm: HashAlgorithm) {
    println!(
        "{} Calculating {} hash for: {}",
        "→".cyan(),
        algorithm.name().yellow(),
        file_path.display().to_string().green()
    );

    match hash_file(file_path, algorithm) {
        Ok(hash) => {
            println!("{} {}", "Hash:".cyan().bold(), hash.bright_white());
        }
        Err(e) => {
            eprintln!("{} Failed to calculate hash: {}", "Error:".red().bold(), e);
        }
    }
}

/// Verify file hash against expected value
pub fn verify_hash(file_path: &PathBuf, expected_hash: &str, algorithm: HashAlgorithm) {
    println!(
        "{} Verifying {} hash for: {}",
        "→".cyan(),
        algorithm.name().yellow(),
        file_path.display().to_string().green()
    );

    match hash_file(file_path, algorithm) {
        Ok(calculated_hash) => {
            let expected_lower = expected_hash.to_lowercase();
            let calculated_lower = calculated_hash.to_lowercase();

            if expected_lower == calculated_lower {
                println!("{} Hash verification successful!", "✓".green().bold());
                println!("  Expected:   {}", expected_lower.bright_white());
                println!("  Calculated: {}", calculated_lower.bright_white());
            } else {
                println!("{} Hash verification failed!", "✗".red().bold());
                println!("  Expected:   {}", expected_lower.yellow());
                println!("  Calculated: {}", calculated_lower.red());
            }
        }
        Err(e) => {
            eprintln!("{} Failed to calculate hash: {}", "Error:".red().bold(), e);
        }
    }
}

/// Calculate all common hashes for a file
pub fn hash_file_all(file_path: &PathBuf) {
    println!(
        "{} Calculating all hashes for: {}\n",
        "→".cyan(),
        file_path.display().to_string().green()
    );

    for algo in [
        HashAlgorithm::MD5,
        HashAlgorithm::SHA256,
        HashAlgorithm::SHA512,
    ] {
        match hash_file(file_path, algo) {
            Ok(hash) => {
                println!(
                    "{:8} {}",
                    format!("{}:", algo.name()).cyan().bold(),
                    hash.bright_white()
                );
            }
            Err(e) => {
                eprintln!(
                    "{} Failed to calculate {} hash: {}",
                    "Error:".red().bold(),
                    algo.name(),
                    e
                );
            }
        }
    }
}
