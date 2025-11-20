use colored::Colorize;
use dialoguer::{Confirm, Select};
use std::process::Command;

pub struct ToolInstaller {
    pub tool_name: String,
    pub os: String,
}

impl ToolInstaller {
    pub fn new(tool_name: &str) -> Self {
        let os = std::env::consts::OS.to_string();
        Self {
            tool_name: tool_name.to_string(),
            os,
        }
    }

    /// Suggest installation with download links and auto-install option
    pub fn suggest_installation(&self) {
        println!("\n{}", "=".repeat(80).yellow());
        println!(
            "{} {} is not installed",
            "⚠".yellow().bold(),
            self.tool_name.cyan().bold()
        );
        println!("{}", "=".repeat(80).yellow());

        // Show download links
        self.show_download_links();

        // Offer auto-install if available
        if self.can_auto_install() {
            println!("\n{}", "Auto-Installation Available!".green().bold());

            if Confirm::new()
                .with_prompt(format!(
                    "Would you like to automatically install {}?",
                    self.tool_name
                ))
                .default(true)
                .interact()
                .unwrap_or(false)
            {
                self.auto_install();
            }
        } else {
            println!(
                "\n{} Please install {} manually using the links above",
                "ℹ".cyan(),
                self.tool_name
            );
        }
    }

    fn show_download_links(&self) {
        println!("\n{}", "Download Links:".cyan().bold());

        match self.tool_name.as_str() {
            "cargo" | "rust" => {
                println!("  {} https://rustup.rs/", "→".cyan());
                println!(
                    "  {} Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh",
                    "💡".yellow()
                );
            }
            "python" => match self.os.as_str() {
                "windows" => println!("  {} https://www.python.org/downloads/windows/", "→".cyan()),
                "macos" => {
                    println!("  {} https://www.python.org/downloads/macos/", "→".cyan());
                    println!("  {} Or use Homebrew: brew install python", "💡".yellow());
                }
                "linux" => {
                    println!("  {} Use your package manager:", "→".cyan());
                    println!("    Ubuntu/Debian: sudo apt install python3");
                    println!("    Fedora: sudo dnf install python3");
                    println!("    Arch: sudo pacman -S python");
                }
                _ => println!("  {} https://www.python.org/downloads/", "→".cyan()),
            },
            "node" | "npm" => {
                println!("  {} https://nodejs.org/", "→".cyan());
                match self.os.as_str() {
                    "windows" => println!("  {} Download the Windows installer", "💡".yellow()),
                    "macos" => println!("  {} Or use Homebrew: brew install node", "💡".yellow()),
                    "linux" => println!(
                        "  {} Or use nvm: https://github.com/nvm-sh/nvm",
                        "💡".yellow()
                    ),
                    _ => {}
                }
            }
            "java" | "mvn" => {
                println!("  {} https://adoptium.net/ (Recommended)", "→".cyan());
                println!(
                    "  {} https://maven.apache.org/download.cgi (Maven)",
                    "→".cyan()
                );
            }
            "dotnet" => {
                println!("  {} https://dotnet.microsoft.com/download", "→".cyan());
            }
            "git" => {
                println!("  {} https://git-scm.com/downloads", "→".cyan());
                match self.os.as_str() {
                    "windows" => println!("  {} Download Git for Windows", "💡".yellow()),
                    "macos" => println!("  {} Or use Homebrew: brew install git", "💡".yellow()),
                    "linux" => println!(
                        "  {} Use your package manager: sudo apt install git",
                        "💡".yellow()
                    ),
                    _ => {}
                }
            }
            _ => {
                println!(
                    "  {} Search for '{}' installation guide",
                    "→".cyan(),
                    self.tool_name
                );
            }
        }
    }

    fn can_auto_install(&self) -> bool {
        match self.os.as_str() {
            "windows" => self.can_auto_install_windows(),
            "macos" => self.can_auto_install_macos(),
            "linux" => self.can_auto_install_linux(),
            _ => false,
        }
    }

    fn can_auto_install_windows(&self) -> bool {
        // Check if winget or chocolatey is available
        which::which("winget").is_ok() || which::which("choco").is_ok()
    }

    fn can_auto_install_macos(&self) -> bool {
        // Check if Homebrew is available
        which::which("brew").is_ok()
    }

    fn can_auto_install_linux(&self) -> bool {
        // Check if apt, dnf, or pacman is available
        which::which("apt").is_ok() || which::which("dnf").is_ok() || which::which("pacman").is_ok()
    }

    fn auto_install(&self) {
        println!("\n{} Installing {}...", "→".cyan(), self.tool_name.green());

        let result = match self.os.as_str() {
            "windows" => self.install_windows(),
            "macos" => self.install_macos(),
            "linux" => self.install_linux(),
            _ => {
                eprintln!(
                    "{} Auto-installation not supported on this OS",
                    "✗".red().bold()
                );
                return;
            }
        };

        match result {
            Ok(_) => {
                println!(
                    "{} {} installed successfully!",
                    "✓".green().bold(),
                    self.tool_name
                );
                println!(
                    "{} You may need to restart your terminal for changes to take effect",
                    "ℹ".cyan()
                );
            }
            Err(e) => {
                eprintln!("{} Installation failed: {}", "✗".red().bold(), e);
                eprintln!("{} Please try manual installation", "ℹ".cyan());
            }
        }
    }

    fn install_windows(&self) -> Result<(), String> {
        let package = self.get_windows_package_name();

        // Try winget first
        if which::which("winget").is_ok() {
            let status = Command::new("winget")
                .args(&["install", "--id", &package, "-e"])
                .status()
                .map_err(|e| e.to_string())?;

            if status.success() {
                return Ok(());
            }
        }

        // Try chocolatey
        if which::which("choco").is_ok() {
            let status = Command::new("choco")
                .args(&["install", &package, "-y"])
                .status()
                .map_err(|e| e.to_string())?;

            if status.success() {
                return Ok(());
            }
        }

        Err("No package manager available (winget or chocolatey)".to_string())
    }

    fn install_macos(&self) -> Result<(), String> {
        let package = self.get_macos_package_name();

        let status = Command::new("brew")
            .args(&["install", &package])
            .status()
            .map_err(|e| e.to_string())?;

        if status.success() {
            Ok(())
        } else {
            Err("Homebrew installation failed".to_string())
        }
    }

    fn install_linux(&self) -> Result<(), String> {
        let package = self.get_linux_package_name();

        // Try apt (Debian/Ubuntu)
        if which::which("apt").is_ok() {
            let status = Command::new("sudo")
                .args(&["apt", "install", "-y", &package])
                .status()
                .map_err(|e| e.to_string())?;

            if status.success() {
                return Ok(());
            }
        }

        // Try dnf (Fedora)
        if which::which("dnf").is_ok() {
            let status = Command::new("sudo")
                .args(&["dnf", "install", "-y", &package])
                .status()
                .map_err(|e| e.to_string())?;

            if status.success() {
                return Ok(());
            }
        }

        // Try pacman (Arch)
        if which::which("pacman").is_ok() {
            let status = Command::new("sudo")
                .args(&["pacman", "-S", "--noconfirm", &package])
                .status()
                .map_err(|e| e.to_string())?;

            if status.success() {
                return Ok(());
            }
        }

        Err("No supported package manager found".to_string())
    }

    fn get_windows_package_name(&self) -> String {
        match self.tool_name.as_str() {
            "python" => "Python.Python.3.12".to_string(),
            "node" | "npm" => "OpenJS.NodeJS".to_string(),
            "git" => "Git.Git".to_string(),
            "cargo" | "rust" => "Rustlang.Rustup".to_string(),
            _ => self.tool_name.clone(),
        }
    }

    fn get_macos_package_name(&self) -> String {
        match self.tool_name.as_str() {
            "cargo" | "rust" => "rust".to_string(),
            "node" | "npm" => "node".to_string(),
            _ => self.tool_name.clone(),
        }
    }

    fn get_linux_package_name(&self) -> String {
        match self.tool_name.as_str() {
            "python" => "python3".to_string(),
            "cargo" | "rust" => "cargo".to_string(),
            "node" | "npm" => "nodejs".to_string(),
            "mvn" => "maven".to_string(),
            _ => self.tool_name.clone(),
        }
    }
}

/// Enhanced suggest_installation function that uses ToolInstaller
pub fn suggest_installation_enhanced(tool_name: &str) {
    let installer = ToolInstaller::new(tool_name);
    installer.suggest_installation();
}
