use colored::Colorize;
use serde_json::{self, Value};
use serde_yaml;

/// Pretty print JSON
pub fn format_json(input: &str) {
    match serde_json::from_str::<Value>(input) {
        Ok(json) => match serde_json::to_string_pretty(&json) {
            Ok(pretty) => {
                println!("{}", "Formatted JSON:".cyan().bold());
                println!("{}", pretty.green());
            }
            Err(e) => eprintln!("{} Failed to format JSON: {}", "Error:".red().bold(), e),
        },
        Err(e) => eprintln!("{} Invalid JSON: {}", "Error:".red().bold(), e),
    }
}

/// Minify JSON
pub fn minify_json(input: &str) {
    match serde_json::from_str::<Value>(input) {
        Ok(json) => match serde_json::to_string(&json) {
            Ok(minified) => {
                println!("{}", "Minified JSON:".cyan().bold());
                println!("{}", minified.green());
            }
            Err(e) => eprintln!("{} Failed to minify JSON: {}", "Error:".red().bold(), e),
        },
        Err(e) => eprintln!("{} Invalid JSON: {}", "Error:".red().bold(), e),
    }
}

/// Validate JSON
pub fn validate_json(input: &str) {
    match serde_json::from_str::<Value>(input) {
        Ok(_) => println!("{} JSON is valid", "✓".green().bold()),
        Err(e) => {
            eprintln!("{} Invalid JSON", "✗".red().bold());
            eprintln!("  Error: {}", e.to_string().yellow());
        }
    }
}

/// Format YAML
pub fn format_yaml(input: &str) {
    match serde_yaml::from_str::<Value>(input) {
        Ok(yaml) => match serde_yaml::to_string(&yaml) {
            Ok(formatted) => {
                println!("{}", "Formatted YAML:".cyan().bold());
                println!("{}", formatted.green());
            }
            Err(e) => eprintln!("{} Failed to format YAML: {}", "Error:".red().bold(), e),
        },
        Err(e) => eprintln!("{} Invalid YAML: {}", "Error:".red().bold(), e),
    }
}

/// Validate YAML
pub fn validate_yaml(input: &str) {
    match serde_yaml::from_str::<Value>(input) {
        Ok(_) => println!("{} YAML is valid", "✓".green().bold()),
        Err(e) => {
            eprintln!("{} Invalid YAML", "✗".red().bold());
            eprintln!("  Error: {}", e.to_string().yellow());
        }
    }
}

/// Convert JSON to YAML
pub fn json_to_yaml(input: &str) {
    match serde_json::from_str::<Value>(input) {
        Ok(json) => match serde_yaml::to_string(&json) {
            Ok(yaml) => {
                println!("{}", "Converted to YAML:".cyan().bold());
                println!("{}", yaml.green());
            }
            Err(e) => eprintln!("{} Failed to convert to YAML: {}", "Error:".red().bold(), e),
        },
        Err(e) => eprintln!("{} Invalid JSON input: {}", "Error:".red().bold(), e),
    }
}

/// Convert YAML to JSON
pub fn yaml_to_json(input: &str) {
    match serde_yaml::from_str::<Value>(input) {
        Ok(yaml) => match serde_json::to_string_pretty(&yaml) {
            Ok(json) => {
                println!("{}", "Converted to JSON:".cyan().bold());
                println!("{}", json.green());
            }
            Err(e) => eprintln!("{} Failed to convert to JSON: {}", "Error:".red().bold(), e),
        },
        Err(e) => eprintln!("{} Invalid YAML input: {}", "Error:".red().bold(), e),
    }
}

/// Get value from JSON path
pub fn json_query(input: &str, path: &str) {
    match serde_json::from_str::<Value>(input) {
        Ok(json) => {
            let parts: Vec<&str> = path.split('.').collect();
            let mut current = &json;

            for part in parts {
                match current.get(part) {
                    Some(value) => current = value,
                    None => {
                        eprintln!("{} Path not found: {}", "Error:".red().bold(), part);
                        return;
                    }
                }
            }

            match serde_json::to_string_pretty(current) {
                Ok(result) => {
                    println!("{}", "Query Result:".cyan().bold());
                    println!("{}", result.green());
                }
                Err(e) => eprintln!("{} Failed to format result: {}", "Error:".red().bold(), e),
            }
        }
        Err(e) => eprintln!("{} Invalid JSON: {}", "Error:".red().bold(), e),
    }
}
