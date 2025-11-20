# Command Line Helper v2.0

<div align="center">

![Version](https://img.shields.io/badge/version-2.0.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
[![CI/CD](https://github.com/CrimsonDevil333333/command-line-helper/workflows/Rust%20CI/CD/badge.svg)](https://github.com/CrimsonDevil333333/command-line-helper/actions)

**A powerful, feature-rich command-line utility with modern web dashboard for developers**

[Features](#-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Web Dashboard](#-web-dashboard) • [CLI Usage](#-cli-usage) • [Contributing](#-contributing)

</div>

---

## ✨ Features

### 🎯 Core Capabilities
- **Language-Specific Operations**: Build, run, and test projects for Rust, Python, Java, .NET, Node.js, Go, C++
- **File Operations**: Advanced file search, copy, move with pattern matching and content search
- **Media Downloads**: Download videos from YouTube and 1000+ sites using yt-dlp with quality selection
- **Web Dashboard**: Modern, responsive UI with real-time system monitoring and command execution
- **Smart Tool Detection**: Automatic detection and installation guidance for missing dependencies

### 🆕 New in v2.0

#### 🌍 Environment Management
- List, get, and set environment variables
- Load from `.env` files
- Export variables to files with filtering
- Cross-platform support

#### 🔐 Cryptography & Hashing
- Calculate MD5, SHA256, SHA512 hashes for files and strings
- Verify file integrity with hash comparison
- Batch hash calculation for multiple files
- Secure hash algorithms

#### 📝 Text Processing
- **Encoding**: Base64 and URL encoding/decoding
- **Case Conversion**: upper, lower, title, camel, snake, kebab
- **Statistics**: Word count, character count, line count
- **Find & Replace**: Pattern-based text manipulation

#### 💻 System Information
- Real-time CPU usage and information
- Memory usage statistics
- Disk space monitoring
- Network interface details
- OS and kernel information

#### 📦 Git Operations
- Repository status and cloning
- Branch creation and management
- Staging and committing changes
- Commit history viewing
- Integrated with local git installation

#### 📁 Archive Management
- **ZIP**: Create, extract, and list ZIP archives
- **TAR.GZ**: Create, extract, and list TAR.GZ archives
- Compression support with progress indicators
- Recursive directory archiving

#### 🎨 Data Formatting
- **JSON**: Validate, format, minify, and query
- **YAML**: Validate and format
- **Conversion**: JSON ↔ YAML bidirectional conversion
- Pretty-printing with syntax highlighting

#### 🌐 Network Utilities
- Port availability checking
- Host ping with latency measurement
- HTTP GET/POST requests
- DNS lookups and resolution
- Public IP detection

#### 🛠️ Smart Tool Installer
- Automatic detection of missing tools (yt-dlp, git, etc.)
- OS-specific installation instructions
- Package manager integration:
  - **Windows**: winget, chocolatey, pip
  - **macOS**: homebrew, pip
  - **Linux**: apt, dnf, pacman, pip

---

## 📦 Installation

### Prerequisites
- **Rust** 1.70 or higher (for building from source)
- **Git** (optional, for cloning repository)

### Quick Install

#### Windows (PowerShell)
```powershell
# Clone repository
git clone https://github.com/CrimsonDevil333333/command-line-helper.git
cd command-line-helper

# Run installer
.\install.ps1
```

#### Linux/macOS (Bash)
```bash
# Clone repository
git clone https://github.com/CrimsonDevil333333/command-line-helper.git
cd command-line-helper

# Make installer executable and run
chmod +x install.sh
./install.sh
```

### Build from Source
```bash
# Clone repository
git clone https://github.com/CrimsonDevil333333/command-line-helper.git
cd command-line-helper

# Build release binary
cargo build --release

# Binary will be at: target/release/command_line_helper[.exe]
```

### Install yt-dlp (for YouTube downloads)
```bash
# Windows
winget install yt-dlp

# macOS
brew install yt-dlp

# Linux/Universal
pip install yt-dlp
```

---

## 🚀 Quick Start

### Start Web Dashboard
```bash
command_line_helper --server-start
```
Then open http://localhost:3000 in your browser.

### Download YouTube Video
```bash
# Best quality video
command_line_helper --url "https://youtu.be/VIDEO_ID" --quality best --output-path ./downloads

# Audio only (MP3)
command_line_helper --url "https://youtu.be/VIDEO_ID" --quality audio --output-path ./downloads

# Worst quality (bandwidth saving)
command_line_helper --url "https://youtu.be/VIDEO_ID" --quality worst --output-path ./downloads
```

### File Operations
```bash
# Search for files
command_line_helper --search "*.rs" --output-path ./src --limit 10

# Search content in files
command_line_helper --data-search "TODO" --search ./src --limit 20

# Copy files
command_line_helper --copy ./source/file.txt --output-path ./destination/ --name newfile.txt

# Move files
command_line_helper --move ./source/file.txt --output-path ./destination/
```

### Project Actions
```bash
# Run project (auto-detects language)
command_line_helper --action run

# Build project
command_line_helper --action build --language rust

# Run tests
command_line_helper --action test --language python

# Navigate to directory first
command_line_helper --goto ./my-project --action run
```

### Environment Variables
```bash
# List all variables
command_line_helper --env-list

# Get specific variable
command_line_helper --env-get PATH

# Set variable
command_line_helper --env-set "MY_VAR=value"

# Load from .env file
command_line_helper --env-load .env

# Export to file
command_line_helper --env-export env_backup.txt
```

### Hashing & Verification
```bash
# Hash a file
command_line_helper --hash-file ./document.pdf --hash-algo sha256

# Hash a string
command_line_helper --hash-string "Hello World" --hash-algo md5

# Verify file integrity
command_line_helper --hash-file ./download.zip --hash-verify "abc123..."

# Calculate all hashes
command_line_helper --hash-all ./important-file.bin
```

### Text Processing
```bash
# Base64 encode
command_line_helper --base64-encode "Hello World"

# Base64 decode
command_line_helper --base64-decode "SGVsbG8gV29ybGQ="

# Convert case
command_line_helper --text-case camel --text "hello world"

# Text statistics
command_line_helper --text-stats ./README.md
```

### Git Operations
```bash
# Check status
command_line_helper --git-status

# Clone repository
command_line_helper --git-clone "https://github.com/user/repo.git" --output-path ./repos

# Create branch
command_line_helper --git-branch feature/new-feature

# Stage all changes
command_line_helper --git-add

# Commit changes
command_line_helper --git-commit "feat: add new feature"

# View commit history
command_line_helper --git-log 10
```

### Archive Operations
```bash
# Create ZIP archive
command_line_helper --zip-create ./my-folder --output-path ./archive.zip

# Extract ZIP archive
command_line_helper --zip-extract ./archive.zip --output-path ./extracted

# Create TAR.GZ archive
command_line_helper --tar-create ./my-folder --output-path ./archive.tar.gz

# Extract TAR.GZ archive
command_line_helper --tar-extract ./archive.tar.gz --output-path ./extracted
```

### System Information
```bash
# All system info
command_line_helper --system-info

# CPU info only
command_line_helper --cpu-info

# Memory info only
command_line_helper --memory-info

# Disk info only
command_line_helper --disk-info

# Network info only
command_line_helper --network-info
```

### Network Utilities
```bash
# Get public IP
command_line_helper --public-ip

# Ping host
command_line_helper --ping google.com

# Check port
command_line_helper --port-check localhost --host 8080

# DNS lookup
command_line_helper --dns-lookup github.com
```

---

## 🌐 Web Dashboard

The web dashboard provides a modern, intuitive interface for all CLI features with real-time monitoring and visual feedback.

### Starting the Server
```bash
command_line_helper --server-start

# Custom port
command_line_helper --server-start --server-port 8080
```

### Dashboard Features

#### 📊 System Monitoring
- Real-time CPU and memory usage charts
- Live system statistics
- OS and hardware information
- Interactive Chart.js visualizations

#### 🎛️ Command Categories
- **Environment**: Manage environment variables
- **Hashing**: File and string hashing tools
- **Text Tools**: Encoding, case conversion, statistics
- **Git Ops**: Repository management
- **Archives**: ZIP and TAR.GZ operations
- **Network**: Connectivity and DNS tools
- **Formatters**: JSON/YAML processing
- **File Ops**: Copy, move, search operations
- **Media**: YouTube and video downloads
- **Project**: Build, run, test actions
- **Navigation**: Directory management

#### 🎨 Modern UI
- Dark theme with glassmorphism effects
- Responsive design (mobile-friendly)
- Real-time terminal output
- Progress indicators
- Color-coded status messages

### API Endpoints

```
GET  /api/system-info     - System information
GET  /api/env             - Environment variables
POST /api/execute         - Execute commands
GET  /api/config          - Configuration
POST /api/config          - Update configuration
```

---

## 🔧 Configuration

Configuration file location: `~/.config/command-line-helper/config.toml`

```toml
[general]
default_output_path = "."
verbose = false

[youtube]
default_quality = "best"
default_output_path = "./downloads"

[git]
default_branch = "main"

[server]
port = 3000
host = "127.0.0.1"
```

### Manage Configuration
```bash
# Show current config
command_line_helper --config-show

# Load config from file
command_line_helper --config-load ./my-config.toml
```

---

## 📚 CLI Usage

### General Options
```
-n, --name <NAME>                    Name for operations
-o, --output-path <PATH>             Output/destination path
-v, --verbose                        Enable verbose logging
-O, --logs-out                       Output logs to file
-h, --help                           Print help
-V, --version                        Print version
```

### Complete Command Reference
```bash
command_line_helper --help
```

For detailed documentation on each command, see [TESTING.md](./TESTING.md)

---

## 🏗️ Project Structure

```
command-line-helper/
├── src/
│   ├── main.rs                 # Entry point
│   ├── config.rs               # Configuration management
│   ├── installer.rs            # Tool installer
│   ├── utilities.rs            # Helper functions
│   └── modules/
│       ├── env_module.rs       # Environment variables
│       ├── hash_module.rs      # Hashing & crypto
│       ├── text_module.rs      # Text processing
│       ├── system_module.rs    # System information
│       ├── git_module.rs       # Git operations
│       ├── archive_module.rs   # Archive management
│       ├── format_module.rs    # Data formatting
│       ├── network_module.rs   # Network utilities
│       ├── youtube_module.rs   # Media downloads
│       ├── server_module.rs    # Web server
│       └── ...
├── web/
│   └── index.html              # Web dashboard
├── .github/
│   └── workflows/
│       └── rust.yml            # CI/CD pipeline
├── install.ps1                 # Windows installer
├── install.sh                  # Linux/macOS installer
├── build.ps1                   # Build script
├── Cargo.toml                  # Rust dependencies
└── README.md                   # This file
```

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'feat: add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Setup
```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/command-line-helper.git
cd command-line-helper

# Build and test
cargo build
cargo test
cargo clippy

# Run locally
cargo run -- --help
```

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Pass clippy lints (`cargo clippy`)
- Add tests for new features
- Update documentation

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **yt-dlp** - Reliable YouTube downloader
- **Rust Community** - Amazing ecosystem and tools
- **Contributors** - Thank you for your contributions!

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/CrimsonDevil333333/command-line-helper/issues)
- **Discussions**: [GitHub Discussions](https://github.com/CrimsonDevil333333/command-line-helper/discussions)
- **Documentation**: [Wiki](https://github.com/CrimsonDevil333333/command-line-helper/wiki)

---

<div align="center">

**Made with ❤️ by developers, for developers**

[⬆ Back to Top](#command-line-helper-v20)

</div>