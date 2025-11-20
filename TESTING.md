# Testing Guide for Command Line Helper

## Prerequisites

Before testing, ensure you have:
- ✅ Rust 1.70+ installed (`rustc --version`)
- ✅ Cargo installed (`cargo --version`)
- ✅ Git installed (for git module testing)

---

## Step 1: Fix Build Errors (Required First)

The application currently has compilation errors that need to be resolved before testing.

### Check Build Status
```powershell
# Navigate to project directory
cd d:\Projects\command-line-helper

# Check for errors
cargo check

# View detailed errors
cargo build 2>&1 | Out-File build-errors.txt
```

### Common Issues to Fix
1. **Missing imports in main.rs** - Need to import server_module
2. **Unused CLI arguments** - Need to wire up all the new feature flags
3. **Module registration** - Ensure all modules are properly registered

---

## Step 2: Build the Application

Once errors are fixed:

```powershell
# Clean previous builds
cargo clean

# Build in debug mode (faster, for testing)
cargo build

# Build release version (optimized)
cargo build --release
```

---

## Step 3: Basic Functionality Tests

### Test 1: Help Command
```powershell
# Debug build
.\target\debug\command_line_helper.exe --help

# Release build
.\target\release\command_line_helper.exe.exe --help
```

**Expected**: Should display comprehensive help text with all options

### Test 2: Version
```powershell
.\target\release\command_line_helper.exe --version
```

**Expected**: `command_line_helper 2.0.0`

---

## Step 4: Feature-by-Feature Testing

### Environment Variables Module

```powershell
# List all environment variables
.\target\release\command_line_helper.exe --env-list

# Get specific variable
.\target\release\command_line_helper.exe --env-get PATH

# Create test .env file
"TEST_VAR=hello`nANOTHER_VAR=world" | Out-File -Encoding UTF8 test.env

# Load from file
.\target\release\command_line_helper.exe --env-load test.env

# Export to file
.\target\release\command_line_helper.exe --env-export env-backup.txt

# Export with filter
.\target\release\command_line_helper.exe --env-export rust-vars.txt --env-filter "RUST"
```

### Hashing Module

```powershell
# Create test file
"Hello, World!" | Out-File test.txt

# Calculate SHA256 hash
.\target\release\command_line_helper.exe --hash-file test.txt --hash-algo sha256

# Calculate MD5 hash
.\target\release\command_line_helper.exe --hash-file test.txt --hash-algo md5

# Calculate all hashes
.\target\release\command_line_helper.exe --hash-all test.txt

# Hash a string
.\target\release\command_line_helper.exe --hash-string "Hello World" --hash-algo sha256

# Verify hash (use hash from previous command)
.\target\release\command_line_helper.exe --hash-file test.txt --hash-verify "YOUR_HASH_HERE" --hash-algo sha256
```

### Text Processing Module

```powershell
# Base64 encode
.\target\release\command_line_helper.exe --base64-encode "Hello World"

# Base64 decode
.\target\release\command_line_helper.exe --base64-decode "SGVsbG8gV29ybGQ="

# URL encode
.\target\release\command_line_helper.exe --url-encode "hello world & more"

# URL decode
.\target\release\command_line_helper.exe --url-decode "hello%20world"

# Case conversions
.\target\release\command_line_helper.exe --text-case upper --text "hello world"
.\target\release\command_line_helper.exe --text-case lower --text "HELLO WORLD"
.\target\release\command_line_helper.exe --text-case snake --text "Hello World"
.\target\release\command_line_helper.exe --text-case camel --text "hello world"
.\target\release\command_line_helper.exe --text-case kebab --text "Hello World"

# Text statistics
.\target\release\command_line_helper.exe --text-stats test.txt
```

### System Information Module

```powershell
# All system info
.\target\release\command_line_helper.exe --system-info

# Specific components
.\target\release\command_line_helper.exe --cpu-info
.\target\release\command_line_helper.exe --memory-info
.\target\release\command_line_helper.exe --disk-info
.\target\release\command_line_helper.exe --network-info
```

### Git Operations Module

```powershell
# Create test repository
mkdir test-repo
cd test-repo
git init

# Test git status
..\target\release\command_line_helper.exe --git-status

# Create a file and test add
"test content" | Out-File test-file.txt
..\target\release\command_line_helper.exe --git-add

# Test commit
..\target\release\command_line_helper.exe --git-commit "Initial commit"

# Test branch operations
..\target\release\command_line_helper.exe --git-branch feature-test
..\target\release\command_line_helper.exe --git-branches

# Test log
..\target\release\command_line_helper.exe --git-log 5

# Clean up
cd ..
Remove-Item -Recurse -Force test-repo
```

### Archive Module

```powershell
# Create test directory
mkdir test-archive
"file1" | Out-File test-archive\file1.txt
"file2" | Out-File test-archive\file2.txt

# Create ZIP archive
.\target\release\command_line_helper.exe --zip-create test.zip --archive-source test-archive

# List ZIP contents
.\target\release\command_line_helper.exe --zip-list test.zip

# Extract ZIP
mkdir extracted
.\target\release\command_line_helper.exe --zip-extract test.zip --output-path extracted

# Create TAR.GZ archive
.\target\release\command_line_helper.exe --tar-create test.tar.gz --archive-source test-archive

# List TAR.GZ contents
.\target\release\command_line_helper.exe --tar-list test.tar.gz

# Extract TAR.GZ
.\target\release\command_line_helper.exe --tar-extract test.tar.gz --output-path extracted-tar

# Clean up
Remove-Item -Recurse -Force test-archive, extracted, extracted-tar
Remove-Item test.zip, test.tar.gz
```

### Format Module

```powershell
# Test JSON formatting
$json = '{"name":"John","age":30,"city":"New York"}'
.\target\release\command_line_helper.exe --json-format $json

# Test JSON minification
$prettyJson = @'
{
  "name": "John",
  "age": 30
}
'@
.\target\release\command_line_helper.exe --json-minify $prettyJson

# Test JSON validation
.\target\release\command_line_helper.exe --json-validate '{"valid":"json"}'
.\target\release\command_line_helper.exe --json-validate '{invalid json}'

# Test JSON to YAML conversion
.\target\release\command_line_helper.exe --json-to-yaml '{"name":"John","age":30}'

# Test YAML to JSON conversion
$yaml = @'
name: John
age: 30
'@
.\target\release\command_line_helper.exe --yaml-to-json $yaml

# Test JSON query
.\target\release\command_line_helper.exe --json-query '{"user":{"name":"John","age":30}}' --query-path "user.name"
```

### Network Module

```powershell
# Check if port is open
.\target\release\command_line_helper.exe --port-check 80 --host google.com
.\target\release\command_line_helper.exe --port-check 443 --host github.com

# Ping host
.\target\release\command_line_helper.exe --ping google.com

# Get public IP
.\target\release\command_line_helper.exe --public-ip

# HTTP GET request
.\target\release\command_line_helper.exe --http-get "https://api.github.com"

# DNS lookup
.\target\release\command_line_helper.exe --dns-lookup google.com
```

### YouTube Module (if working)

```powershell
# Download a short public video (use a test video)
.\target\release\command_line_helper.exe --url "https://www.youtube.com/watch?v=dQw4w9WgXcQ" --output-path downloads
```

---

## Step 5: Web Dashboard Testing

### Start the Server
```powershell
# Start on default port (8080)
.\target\release\command_line_helper.exe --server-start

# Start on custom port
.\target\release\command_line_helper.exe --server-start --server-port 3000
```

### Test Dashboard
1. Open browser to `http://localhost:8080`
2. Verify dashboard loads with modern UI
3. Check system information displays correctly
4. Test quick action buttons
5. Try command terminal

### Test API Endpoints
```powershell
# Test system info API
Invoke-RestMethod -Uri "http://localhost:8080/api/system-info"

# Test environment variables API
Invoke-RestMethod -Uri "http://localhost:8080/api/env-vars"

# Test config API
Invoke-RestMethod -Uri "http://localhost:8080/api/config"
```

---

## Step 6: Configuration Testing

```powershell
# Show current configuration
.\target\release\command_line_helper.exe --config-show

# Configuration file location:
# Windows: C:\Users\USERNAME\AppData\Roaming\command-line-helper\config.toml
```

---

## Step 7: Installation Script Testing

### Test Windows Installation
```powershell
# Run installation script
.\install.ps1

# Verify installation
command_line_helper --version
clh --version

# Test from new terminal
# Open new PowerShell window and run:
clh --help
```

---

## Step 8: Automated Testing

### Run Unit Tests
```powershell
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_os_info

# Run tests for specific module
cargo test utilities::tests
```

### Run Integration Tests
```powershell
# If integration tests exist
cargo test --test '*'
```

---

## Step 9: Performance Testing

```powershell
# Measure execution time
Measure-Command { .\target\release\command_line_helper.exe --system-info }

# Test with large file
$largeFile = "large-test.txt"
1..10000 | ForEach-Object { "Line $_" } | Out-File $largeFile
Measure-Command { .\target\release\command_line_helper.exe --hash-file $largeFile }
```

---

## Step 10: Cross-Platform Testing (if applicable)

### On Linux/macOS
```bash
# Build
cargo build --release

# Run tests
./target/release/command_line_helper --help
./target/release/command_line_helper --system-info

# Test installation script
bash install.sh
```

---

## Common Issues & Solutions

### Issue: Build Fails
**Solution**: 
```powershell
cargo clean
cargo update
cargo build --release
```

### Issue: Module Not Found
**Solution**: Check that all modules are registered in `src/modules.rs`

### Issue: Permission Denied
**Solution**: Run PowerShell as Administrator

### Issue: PATH Not Updated
**Solution**: 
```powershell
# Restart terminal or manually refresh
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
```

---

## Test Checklist

- [ ] Application builds without errors
- [ ] Help command displays all options
- [ ] Environment variable operations work
- [ ] File hashing calculates correctly
- [ ] Text processing functions work
- [ ] System information displays
- [ ] Git operations function (in git repo)
- [ ] Archive creation/extraction works
- [ ] JSON/YAML formatting works
- [ ] Network utilities connect
- [ ] Web dashboard loads
- [ ] API endpoints respond
- [ ] Configuration loads/saves
- [ ] Installation script completes
- [ ] Executable in PATH
- [ ] Alias works (clh)

---

## Reporting Issues

If you find bugs during testing:

1. **Note the command** that failed
2. **Copy the error message**
3. **Check the logs** (if `--logs-out` was used)
4. **Try with `--verbose`** flag for more details
5. **Document** the steps to reproduce

---

## Next Steps After Testing

1. Fix any bugs found
2. Add more unit tests for edge cases
3. Create integration tests
4. Update documentation with any changes
5. Create release build for distribution

---

## Quick Test Script

Save this as `quick-test.ps1`:

```powershell
Write-Host "Running Quick Tests..." -ForegroundColor Cyan

# Test 1: Help
Write-Host "`n→ Test 1: Help Command" -ForegroundColor Yellow
.\target\release\command_line_helper.exe --help

# Test 2: System Info
Write-Host "`n→ Test 2: System Info" -ForegroundColor Yellow
.\target\release\command_line_helper.exe --system-info

# Test 3: Hash
Write-Host "`n→ Test 3: Hashing" -ForegroundColor Yellow
"test" | Out-File test.txt
.\target\release\command_line_helper.exe --hash-file test.txt

# Test 4: Base64
Write-Host "`n→ Test 4: Base64 Encoding" -ForegroundColor Yellow
.\target\release\command_line_helper.exe --base64-encode "Hello World"

Write-Host "`n✓ Quick tests complete!" -ForegroundColor Green
```

Run with: `.\quick-test.ps1`
