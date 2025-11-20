use base64::{engine::general_purpose, Engine as _};
use colored::Colorize;
use url::{form_urlencoded, Url};

/// Base64 encode a string
pub fn base64_encode(input: &str) {
    let encoded = general_purpose::STANDARD.encode(input.as_bytes());
    println!("{} {}", "Encoded:".cyan().bold(), encoded.green());
}

/// Base64 decode a string
pub fn base64_decode(input: &str) {
    match general_purpose::STANDARD.decode(input) {
        Ok(decoded) => match String::from_utf8(decoded) {
            Ok(text) => println!("{} {}", "Decoded:".cyan().bold(), text.green()),
            Err(_) => eprintln!("{} Decoded data is not valid UTF-8", "Error:".red().bold()),
        },
        Err(e) => eprintln!("{} Failed to decode: {}", "Error:".red().bold(), e),
    }
}

/// URL encode a string
pub fn url_encode(input: &str) {
    let encoded: String = form_urlencoded::byte_serialize(input.as_bytes()).collect();
    println!("{} {}", "Encoded:".cyan().bold(), encoded.green());
}

/// URL decode a string
pub fn url_decode(input: &str) {
    match Url::parse(&format!("http://example.com?q={}", input)) {
        Ok(url) => {
            if let Some(decoded) = url.query_pairs().next() {
                println!("{} {}", "Decoded:".cyan().bold(), decoded.1.green());
            }
        }
        Err(_) => {
            // Try direct percent decoding
            match form_urlencoded::parse(input.as_bytes()).next() {
                Some((_, decoded)) => println!("{} {}", "Decoded:".cyan().bold(), decoded.green()),
                None => eprintln!("{} Failed to decode URL", "Error:".red().bold()),
            }
        }
    }
}

/// Convert text to uppercase
pub fn to_uppercase(input: &str) {
    println!(
        "{} {}",
        "Uppercase:".cyan().bold(),
        input.to_uppercase().green()
    );
}

/// Convert text to lowercase
pub fn to_lowercase(input: &str) {
    println!(
        "{} {}",
        "Lowercase:".cyan().bold(),
        input.to_lowercase().green()
    );
}

/// Convert text to title case
pub fn to_titlecase(input: &str) {
    let title: String = input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    println!("{} {}", "Title Case:".cyan().bold(), title.green());
}

/// Convert text to camelCase
pub fn to_camelcase(input: &str) {
    let words: Vec<&str> = input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect();
    if words.is_empty() {
        println!("{} {}", "camelCase:".cyan().bold(), "".green());
        return;
    }

    let camel = words[0].to_lowercase()
        + &words[1..]
            .iter()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect::<String>();
    println!("{} {}", "camelCase:".cyan().bold(), camel.green());
}

/// Convert text to snake_case
pub fn to_snakecase(input: &str) {
    let snake = input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect::<Vec<_>>()
        .join("_");
    println!("{} {}", "snake_case:".cyan().bold(), snake.green());
}

/// Convert text to kebab-case
pub fn to_kebabcase(input: &str) {
    let kebab = input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect::<Vec<_>>()
        .join("-");
    println!("{} {}", "kebab-case:".cyan().bold(), kebab.green());
}

/// Count lines, words, and characters in text
pub fn text_stats(input: &str) {
    let lines = input.lines().count();
    let words = input.split_whitespace().count();
    let chars = input.chars().count();
    let bytes = input.len();

    println!("{}", "Text Statistics:".cyan().bold());
    println!("  Lines:      {}", lines.to_string().green());
    println!("  Words:      {}", words.to_string().green());
    println!("  Characters: {}", chars.to_string().green());
    println!("  Bytes:      {}", bytes.to_string().green());
}

/// Find and replace in text
pub fn find_replace(input: &str, find: &str, replace: &str) {
    let result = input.replace(find, replace);
    let count = input.matches(find).count();

    println!(
        "{} Replaced {} occurrences",
        "✓".green().bold(),
        count.to_string().yellow()
    );
    println!("{}", "Result:".cyan().bold());
    println!("{}", result.green());
}
