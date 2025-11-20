use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub colors: ColorConfig,
    pub paths: PathConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub verbose: bool,
    pub log_to_file: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorConfig {
    pub enabled: bool,
    pub theme: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathConfig {
    pub default_output: String,
    pub download_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: GeneralConfig {
                verbose: false,
                log_to_file: false,
            },
            colors: ColorConfig {
                enabled: true,
                theme: "default".to_string(),
            },
            paths: PathConfig {
                default_output: ".".to_string(),
                download_path: ".".to_string(),
            },
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Load configuration from a specific file
    pub fn load_from_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;

        Ok(())
    }

    /// Get the configuration file path
    pub fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir().ok_or("Could not determine config directory")?;

        Ok(config_dir.join("command-line-helper").join("config.toml"))
    }

    /// Display current configuration
    pub fn display(&self) {
        use colored::Colorize;

        println!("{}", "Current Configuration".cyan().bold());
        println!("{}", "=".repeat(80).cyan());

        println!("\n{}", "General:".yellow().bold());
        println!(
            "  Verbose:      {}",
            if self.general.verbose {
                "enabled".green()
            } else {
                "disabled".red()
            }
        );
        println!(
            "  Log to file:  {}",
            if self.general.log_to_file {
                "enabled".green()
            } else {
                "disabled".red()
            }
        );

        println!("\n{}", "Colors:".yellow().bold());
        println!(
            "  Enabled:      {}",
            if self.colors.enabled {
                "yes".green()
            } else {
                "no".red()
            }
        );
        println!("  Theme:        {}", self.colors.theme.green());

        println!("\n{}", "Paths:".yellow().bold());
        println!("  Default output: {}", self.paths.default_output.green());
        println!("  Download path:  {}", self.paths.download_path.green());

        if let Ok(config_path) = Self::config_path() {
            println!(
                "\n{} {}",
                "Config file:".cyan().bold(),
                config_path.display().to_string().yellow()
            );
        }
    }
}
