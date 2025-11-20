# Build Script for Command Line Helper
# Creates release executables for distribution

# Windows Build
Write-Host "Building for Windows..." -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Windows build complete" -ForegroundColor Green
    Write-Host "  Executable: target\release\command_line_helper.exe" -ForegroundColor Yellow
} else {
    Write-Host "✗ Windows build failed" -ForegroundColor Red
    exit 1
}

# Create distribution folder
$distDir = "dist\windows"
New-Item -ItemType Directory -Path $distDir -Force | Out-Null

# Copy executable and web files
Copy-Item "target\release\command_line_helper.exe" -Destination "$distDir\" -Force
Copy-Item "web" -Destination "$distDir\" -Recurse -Force
Copy-Item "README.md" -Destination "$distDir\" -Force
Copy-Item "install.ps1" -Destination "$distDir\" -Force

# Create README for distribution
@"
# Command Line Helper v2.0 - Windows Distribution

## Quick Start

1. Run install.ps1 as Administrator:
   ```powershell
   .\install.ps1
   ```

2. Or manually:
   - Copy command_line_helper.exe to a folder in your PATH
   - Or add this folder to your PATH environment variable

## Usage

```powershell
# Show help
.\command_line_helper.exe --help

# Start web dashboard
.\command_line_helper.exe --server-start

# System information
.\command_line_helper.exe --system-info
```

## Documentation

See README.md for full documentation.
"@ | Out-File -FilePath "$distDir\QUICKSTART.txt" -Encoding UTF8

Write-Host ""
Write-Host "✓ Distribution package created: $distDir" -ForegroundColor Green
Write-Host ""
Write-Host "To create a ZIP for distribution:" -ForegroundColor Cyan
Write-Host "  Compress-Archive -Path $distDir\* -DestinationPath command-line-helper-windows.zip" -ForegroundColor Yellow
