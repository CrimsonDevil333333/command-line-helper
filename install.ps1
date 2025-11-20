# Command Line Helper Installation Script for Windows
# Run this script in PowerShell as Administrator

param(
    [string]$InstallPath = "$env:LOCALAPPDATA\Programs\CommandLineHelper",
    [switch]$AddToPath = $true,
    [switch]$CreateAlias = $true
)

Write-Host "==================================================================" -ForegroundColor Cyan
Write-Host "  Command Line Helper - Windows Installation Script" -ForegroundColor Cyan
Write-Host "==================================================================" -ForegroundColor Cyan
Write-Host ""

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin -and $AddToPath) {
    Write-Host "⚠ Warning: Not running as Administrator" -ForegroundColor Yellow
    Write-Host "  PATH modifications require Administrator privileges" -ForegroundColor Yellow
    Write-Host "  Re-run this script as Administrator or use -AddToPath:`$false" -ForegroundColor Yellow
    Write-Host ""
}

# Check if Rust is installed
Write-Host "→ Checking for Rust installation..." -ForegroundColor Cyan
$rustInstalled = Get-Command cargo -ErrorAction SilentlyContinue

if (-not $rustInstalled) {
    Write-Host "✗ Rust is not installed" -ForegroundColor Red
    Write-Host ""
    Write-Host "Please install Rust first:" -ForegroundColor Yellow
    Write-Host "  1. Visit: https://rustup.rs/" -ForegroundColor Yellow
    Write-Host "  2. Download and run rustup-init.exe" -ForegroundColor Yellow
    Write-Host "  3. Re-run this installation script" -ForegroundColor Yellow
    exit 1
}

Write-Host "✓ Rust is installed" -ForegroundColor Green

# Create installation directory
Write-Host ""
Write-Host "→ Creating installation directory..." -ForegroundColor Cyan
if (-not (Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    Write-Host "✓ Created: $InstallPath" -ForegroundColor Green
} else {
    Write-Host "✓ Directory exists: $InstallPath" -ForegroundColor Green
}

# Build the project
Write-Host ""
Write-Host "→ Building Command Line Helper..." -ForegroundColor Cyan
Write-Host "  This may take a few minutes..." -ForegroundColor Yellow

$buildOutput = cargo build --release 2>&1

if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Build successful" -ForegroundColor Green
} else {
    Write-Host "✗ Build failed" -ForegroundColor Red
    Write-Host $buildOutput
    exit 1
}

# Copy executable
Write-Host ""
Write-Host "→ Installing executable..." -ForegroundColor Cyan
$exePath = "target\release\command_line_helper.exe"

if (Test-Path $exePath) {
    Copy-Item $exePath -Destination "$InstallPath\command_line_helper.exe" -Force
    Copy-Item $exePath -Destination "$InstallPath\clh.exe" -Force  # Short alias
    Write-Host "✓ Installed to: $InstallPath" -ForegroundColor Green
} else {
    Write-Host "✗ Executable not found: $exePath" -ForegroundColor Red
    exit 1
}

# Add to PATH
if ($AddToPath) {
    Write-Host ""
    Write-Host "→ Adding to PATH..." -ForegroundColor Cyan
    
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    
    if ($currentPath -notlike "*$InstallPath*") {
        $newPath = "$currentPath;$InstallPath"
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        Write-Host "✓ Added to PATH" -ForegroundColor Green
        Write-Host "  You may need to restart your terminal" -ForegroundColor Yellow
    } else {
        Write-Host "✓ Already in PATH" -ForegroundColor Green
    }
}

# Create PowerShell profile alias
if ($CreateAlias) {
    Write-Host ""
    Write-Host "→ Creating PowerShell alias..." -ForegroundColor Cyan
    
    $profilePath = $PROFILE
    $aliasCommand = "Set-Alias -Name clh -Value command_line_helper"
    
    if (-not (Test-Path $profilePath)) {
        New-Item -ItemType File -Path $profilePath -Force | Out-Null
    }
    
    $profileContent = Get-Content $profilePath -ErrorAction SilentlyContinue
    
    if ($profileContent -notcontains $aliasCommand) {
        Add-Content -Path $profilePath -Value "`n# Command Line Helper alias"
        Add-Content -Path $profilePath -Value $aliasCommand
        Write-Host "✓ Alias 'clh' created" -ForegroundColor Green
    } else {
        Write-Host "✓ Alias already exists" -ForegroundColor Green
    }
}

# Installation complete
Write-Host ""
Write-Host "==================================================================" -ForegroundColor Green
Write-Host "  Installation Complete!" -ForegroundColor Green
Write-Host "==================================================================" -ForegroundColor Green
Write-Host ""
Write-Host "Usage:" -ForegroundColor Cyan
Write-Host "  command_line_helper --help" -ForegroundColor White
Write-Host "  clh --help" -ForegroundColor White
Write-Host ""
Write-Host "Start web dashboard:" -ForegroundColor Cyan
Write-Host "  command_line_helper --server-start" -ForegroundColor White
Write-Host "  Then open: http://localhost:8080" -ForegroundColor White
Write-Host ""
Write-Host "Examples:" -ForegroundColor Cyan
Write-Host "  clh --system-info" -ForegroundColor White
Write-Host "  clh --hash-file myfile.txt" -ForegroundColor White
Write-Host "  clh --env-list" -ForegroundColor White
Write-Host ""
Write-Host "For more information, visit:" -ForegroundColor Cyan
Write-Host "  https://github.com/CrimsonDevil333333/command-line-helper" -ForegroundColor White
Write-Host ""
