#!/bin/bash

# Command Line Helper Installation Script for Linux/macOS
# Run with: curl -fsSL https://raw.githubusercontent.com/CrimsonDevil333333/command-line-helper/main/install.sh | bash

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
INSTALL_DIR="$HOME/.local/bin"
REPO_URL="https://github.com/CrimsonDevil333333/command-line-helper.git"
BUILD_DIR="/tmp/command-line-helper-build"

echo -e "${CYAN}==================================================================${NC}"
echo -e "${CYAN}  Command Line Helper - Installation Script${NC}"
echo -e "${CYAN}==================================================================${NC}"
echo ""

# Check if Rust is installed
echo -e "${CYAN}→ Checking for Rust installation...${NC}"
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ Rust is not installed${NC}"
    echo ""
    echo -e "${YELLOW}Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓ Rust installed${NC}"
else
    echo -e "${GREEN}✓ Rust is installed${NC}"
fi

# Create installation directory
echo ""
echo -e "${CYAN}→ Creating installation directory...${NC}"
mkdir -p "$INSTALL_DIR"
echo -e "${GREEN}✓ Created: $INSTALL_DIR${NC}"

# Clone or update repository
echo ""
echo -e "${CYAN}→ Downloading Command Line Helper...${NC}"
if [ -d "$BUILD_DIR" ]; then
    rm -rf "$BUILD_DIR"
fi

git clone "$REPO_URL" "$BUILD_DIR"
cd "$BUILD_DIR"
echo -e "${GREEN}✓ Downloaded${NC}"

# Build the project
echo ""
echo -e "${CYAN}→ Building Command Line Helper...${NC}"
echo -e "${YELLOW}  This may take a few minutes...${NC}"

if cargo build --release; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi

# Install executable
echo ""
echo -e "${CYAN}→ Installing executable...${NC}"
cp target/release/command_line_helper "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/command_line_helper"

# Create short alias
ln -sf "$INSTALL_DIR/command_line_helper" "$INSTALL_DIR/clh"
echo -e "${GREEN}✓ Installed to: $INSTALL_DIR${NC}"

# Add to PATH if not already there
echo ""
echo -e "${CYAN}→ Configuring PATH...${NC}"

SHELL_RC=""
if [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
elif [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
fi

if [ -n "$SHELL_RC" ]; then
    if ! grep -q "$INSTALL_DIR" "$SHELL_RC" 2>/dev/null; then
        echo "" >> "$SHELL_RC"
        echo "# Command Line Helper" >> "$SHELL_RC"
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_RC"
        echo "alias clh='command_line_helper'" >> "$SHELL_RC"
        echo -e "${GREEN}✓ Added to PATH in $SHELL_RC${NC}"
        echo -e "${YELLOW}  Run: source $SHELL_RC${NC}"
    else
        echo -e "${GREEN}✓ Already in PATH${NC}"
    fi
fi

# Clean up
echo ""
echo -e "${CYAN}→ Cleaning up...${NC}"
cd ~
rm -rf "$BUILD_DIR"
echo -e "${GREEN}✓ Cleanup complete${NC}"

# Installation complete
echo ""
echo -e "${GREEN}==================================================================${NC}"
echo -e "${GREEN}  Installation Complete!${NC}"
echo -e "${GREEN}==================================================================${NC}"
echo ""
echo -e "${CYAN}Usage:${NC}"
echo -e "  command_line_helper --help"
echo -e "  clh --help"
echo ""
echo -e "${CYAN}Start web dashboard:${NC}"
echo -e "  command_line_helper --server-start"
echo -e "  Then open: http://localhost:8080"
echo ""
echo -e "${CYAN}Examples:${NC}"
echo -e "  clh --system-info"
echo -e "  clh --hash-file myfile.txt"
echo -e "  clh --env-list"
echo ""
echo -e "${CYAN}Reload your shell or run:${NC}"
echo -e "  source $SHELL_RC"
echo ""
echo -e "${CYAN}For more information, visit:${NC}"
echo -e "  https://github.com/CrimsonDevil333333/command-line-helper"
echo ""
