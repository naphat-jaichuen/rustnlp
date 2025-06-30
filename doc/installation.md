# Installation Guide

This guide covers how to set up and build the individual callback handler executables.

## Prerequisites

### System Requirements

- **Operating System**: macOS, Linux, or Windows
- **Rust**: Version 1.70 or later
- **Cargo**: Comes with Rust installation
- **Memory**: At least 2GB RAM for compilation
- **Disk Space**: ~500MB for source code and compiled binaries

### Required Tools

1. **Rust and Cargo**
   ```bash
   # Install via rustup (recommended)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   
   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Git** (for cloning and development)
   ```bash
   # macOS
   xcode-select --install
   
   # Ubuntu/Debian
   sudo apt update && sudo apt install git
   
   # Verify installation
   git --version
   ```

3. **jq** (optional, for testing and JSON parsing)
   ```bash
   # macOS
   brew install jq
   
   # Ubuntu/Debian
   sudo apt install jq
   
   # Verify installation
   jq --version
   ```

## Quick Installation

### Option 1: Build All Handlers (Recommended)

```bash
# Navigate to project directory
cd /path/to/rustlm-server

# Build all 18 handlers at once
./build_handlers.sh
```

This will:
- Compile all handlers in release mode
- Create executables in `target/release/`
- Display build status for each handler
- Show usage examples

### Option 2: Build Individual Handlers

```bash
# Build specific handlers
cargo build --release --bin sentiment-handler
cargo build --release --bin install-handler
cargo build --release --bin find-file-handler

# List available binaries
cargo build --release --bins
```

### Option 3: Development Build

```bash
# Build in debug mode (faster compilation, larger binaries)
cargo build --bin sentiment-handler
cargo build --bin install-handler

# Debug binaries are in target/debug/
./target/debug/sentiment-handler 'test'
```

## Verification

### 1. Check Built Executables

```bash
# List all built handlers
ls -la target/release/ | grep handler

# Check executable permissions
ls -la target/release/sentiment-handler
```

Expected output:
```
-rwxr-xr-x  1 user  staff  4042000 Jul  1 05:31 sentiment-handler
```

### 2. Test Individual Handlers

```bash
# Quick functionality test
./target/release/sentiment-handler 'This is a test'
./target/release/install-handler nodejs
./target/release/find-file-handler config.json
```

Expected output format:
```json
{
  "success": true,
  "message": "Handler completed successfully",
  "data": { ... },
  "execution_time_ms": 42
}
```

### 3. Run Comprehensive Tests

```bash
# Test all handlers
./test_handlers.sh
```

This will test:
- All system handlers
- All NLP handlers
- Custom parsed results
- Different input methods

## Installation Options

### Development Installation

For development and testing:

```bash
# Clone the repository
git clone <repository-url>
cd rustlm-server

# Install development dependencies
cargo fetch

# Build in debug mode
cargo build

# Run tests
cargo test
```

### Production Installation

For production deployment:

```bash
# Build optimized release binaries
cargo build --release

# Optional: Strip debug symbols for smaller binaries
strip target/release/*-handler

# Copy binaries to deployment location
cp target/release/*-handler /usr/local/bin/
```

### Docker Installation

Create a Dockerfile for containerized deployment:

```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/*-handler /usr/local/bin/

CMD ["sentiment-handler", "--help"]
```

Build and run:
```bash
docker build -t rustlm-handlers .
docker run rustlm-handlers sentiment-handler 'test input'
```

## Platform-Specific Notes

### macOS

- **M1/M2 Macs**: Native ARM64 compilation supported
- **Intel Macs**: x86_64 compilation
- **Cross-compilation**: Can target both architectures

```bash
# Build for specific target
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

### Linux

- **Dependencies**: Ensure `pkg-config` and `build-essential` are installed
- **Cross-compilation**: Can target multiple Linux distributions

```bash
# Install build dependencies (Ubuntu/Debian)
sudo apt update
sudo apt install build-essential pkg-config

# Build
cargo build --release
```

### Windows

- **MSVC**: Recommended toolchain
- **MinGW**: Alternative option

```powershell
# Using PowerShell
cargo build --release

# Executables will have .exe extension
.\target\release\sentiment-handler.exe "test input"
```

## Troubleshooting Installation

### Common Issues

#### 1. Compilation Errors

**Issue**: `error: linker 'cc' not found`
```bash
# macOS
xcode-select --install

# Linux
sudo apt install build-essential

# Windows
# Install Visual Studio Build Tools
```

**Issue**: `error: failed to run custom build command for 'openssl-sys'`
```bash
# macOS
brew install openssl
export OPENSSL_DIR=$(brew --prefix openssl)

# Linux
sudo apt install libssl-dev pkg-config
```

#### 2. Permission Issues

**Issue**: `permission denied: ./target/release/handler-name`
```bash
# Fix permissions
chmod +x target/release/*-handler

# Check current permissions
ls -la target/release/
```

#### 3. Missing Dependencies

**Issue**: Handler fails to run with missing library errors
```bash
# Check dynamic dependencies
ldd target/release/sentiment-handler  # Linux
otool -L target/release/sentiment-handler  # macOS

# Install missing system libraries
sudo apt install libc6-dev  # Linux example
```

#### 4. Out of Memory

**Issue**: Compilation fails with OOM error
```bash
# Reduce parallelism
cargo build --release -j 1

# Increase swap space (Linux)
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

### Build Performance

#### Optimize Compilation Time

```bash
# Use release profile with debug info for faster builds
cargo build --profile dev-release

# Use cached builds
export CARGO_INCREMENTAL=1

# Parallel builds (adjust based on CPU cores)
cargo build --release -j 8
```

#### Reduce Binary Size

```toml
# Add to Cargo.toml
[profile.release]
strip = true
lto = true
codegen-units = 1
panic = "abort"
```

## Post-Installation

### 1. Add to PATH

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$PATH:/path/to/rustlm-server/target/release"

# Reload shell
source ~/.bashrc  # or ~/.zshrc

# Test global access
sentiment-handler 'test from anywhere'
```

### 2. Create Symlinks

```bash
# Create convenient symlinks
sudo ln -s /path/to/rustlm-server/target/release/sentiment-handler /usr/local/bin/
sudo ln -s /path/to/rustlm-server/target/release/install-handler /usr/local/bin/

# Test
which sentiment-handler
```

### 3. Shell Completions

```bash
# Generate completion scripts (if supported)
sentiment-handler --generate-completion bash > ~/.bash_completion.d/sentiment-handler
```

## Maintenance

### Updating Handlers

```bash
# Pull latest changes
git pull origin main

# Rebuild all handlers
./build_handlers.sh

# Test after update
./test_handlers.sh
```

### Cleaning Build Artifacts

```bash
# Clean all build artifacts
cargo clean

# Clean only release builds
rm -rf target/release/

# Rebuild from scratch
./build_handlers.sh
```

### Monitoring Build Times

```bash
# Time the build process
time ./build_handlers.sh

# Profile compilation
cargo build --release --timings
```

## Next Steps

After successful installation:

1. Read the [Usage Examples](examples.md) guide
2. Review the [API Reference](api.md)
3. Check out [Development Guide](development.md) for customization
4. See [Troubleshooting](troubleshooting.md) for common issues

For support, refer to the project documentation or create an issue in the repository.
