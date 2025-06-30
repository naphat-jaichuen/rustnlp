# Troubleshooting Guide

This guide helps resolve common issues when building and using the individual callback handler executables.

## Table of Contents

- [Build Issues](#build-issues)
- [Runtime Issues](#runtime-issues)
- [Shell Issues](#shell-issues)
- [Performance Issues](#performance-issues)
- [Integration Issues](#integration-issues)
- [Debugging Techniques](#debugging-techniques)

## Build Issues

### Compilation Errors

#### Issue: `error: linker 'cc' not found`

**Platforms**: macOS, Linux  
**Cause**: Missing C compiler toolchain

**Solutions**:

**macOS**:
```bash
# Install Xcode command line tools
xcode-select --install

# Verify installation
cc --version
```

**Linux (Ubuntu/Debian)**:
```bash
# Install build essentials
sudo apt update
sudo apt install build-essential

# Verify installation
gcc --version
```

**Linux (CentOS/RHEL)**:
```bash
# Install development tools
sudo yum groupinstall "Development Tools"
# or for newer versions
sudo dnf groupinstall "Development Tools"
```

#### Issue: `error: failed to run custom build command for 'openssl-sys'`

**Cause**: Missing OpenSSL development libraries

**Solutions**:

**macOS**:
```bash
# Install OpenSSL via Homebrew
brew install openssl

# Set environment variables
export OPENSSL_DIR=$(brew --prefix openssl)
export OPENSSL_LIB_DIR=$(brew --prefix openssl)/lib
export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include

# Rebuild
cargo clean && cargo build --release
```

**Linux (Ubuntu/Debian)**:
```bash
# Install OpenSSL development packages
sudo apt install libssl-dev pkg-config

# Rebuild
cargo clean && cargo build --release
```

**Linux (CentOS/RHEL)**:
```bash
# Install OpenSSL development packages
sudo yum install openssl-devel pkgconfig
# or
sudo dnf install openssl-devel pkgconfig
```

#### Issue: `error: could not compile` with memory errors

**Cause**: Insufficient memory during compilation

**Solutions**:

```bash
# Reduce parallel compilation jobs
cargo build --release -j 1

# Increase system swap (Linux)
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Use incremental compilation
export CARGO_INCREMENTAL=1
cargo build --release
```

#### Issue: `error: package collision` or dependency conflicts

**Cause**: Conflicting dependency versions

**Solutions**:

```bash
# Clean all build artifacts
cargo clean

# Update Cargo.lock
rm Cargo.lock
cargo update

# Rebuild from scratch
./build_handlers.sh
```

### Missing Dependencies

#### Issue: Handler builds but fails to link

**Cause**: Missing system libraries

**Diagnosis**:
```bash
# Check what libraries are required (Linux)
ldd target/release/sentiment-handler

# Check dynamic dependencies (macOS)
otool -L target/release/sentiment-handler
```

**Solutions**:

**Linux**:
```bash
# Install common missing libraries
sudo apt install libc6-dev libgcc-s1 libstdc++6

# For specific missing libraries, install the corresponding -dev package
sudo apt install lib<name>-dev
```

**macOS**:
```bash
# Update system
xcode-select --install

# If using Homebrew libraries, ensure they're linked
brew doctor
brew cleanup
```

## Runtime Issues

### Handler Execution Failures

#### Issue: `permission denied: ./target/release/handler-name`

**Cause**: Executable permissions not set

**Solutions**:
```bash
# Fix permissions for all handlers
chmod +x target/release/*-handler

# Verify permissions
ls -la target/release/ | grep handler

# Should show: -rwxr-xr-x
```

#### Issue: `command not found: ./target/release/handler-name`

**Cause**: Handler not built or wrong path

**Solutions**:
```bash
# Check if handler exists
ls -la target/release/

# Build missing handlers
./build_handlers.sh

# Use absolute path
/full/path/to/rustlm-server/target/release/sentiment-handler 'test'

# Add to PATH
export PATH="$PATH:$(pwd)/target/release"
```

#### Issue: Handler runs but produces no output

**Cause**: Input/output redirection issues

**Diagnosis**:
```bash
# Check if handler is reading from stdin when it shouldn't
echo 'test input' | ./target/release/sentiment-handler

# Check if handler is waiting for input
timeout 5s ./target/release/sentiment-handler 'test input'

# Check stderr output
./target/release/sentiment-handler 'test input' 2>&1
```

**Solutions**:
```bash
# Provide input via command line arguments
./target/release/sentiment-handler 'test input'

# Or via stdin with explicit EOF
echo 'test input' | ./target/release/sentiment-handler

# Enable debug logging
RUST_LOG=debug ./target/release/sentiment-handler 'test input'
```

### JSON Output Issues

#### Issue: Invalid JSON output

**Cause**: Handler crashing or producing non-JSON output

**Diagnosis**:
```bash
# Validate JSON output
./target/release/sentiment-handler 'test' | jq '.'

# Check for mixed output (JSON + logs)
./target/release/sentiment-handler 'test' 2>/dev/null | jq '.'

# Check stderr separately
./target/release/sentiment-handler 'test' 2>&1 >/dev/null
```

**Solutions**:
```bash
# Disable logging to stderr
RUST_LOG=off ./target/release/sentiment-handler 'test'

# Filter only JSON output
./target/release/sentiment-handler 'test' | grep '^{' | jq '.'

# Use error-safe parsing
output=$(./target/release/sentiment-handler 'test')
if echo "$output" | jq -e . >/dev/null 2>&1; then
  echo "Valid JSON"
else
  echo "Invalid JSON: $output"
fi
```

#### Issue: Handler returns `success: false` unexpectedly

**Cause**: Handler-specific validation or processing errors

**Diagnosis**:
```bash
# Check error message
./target/release/sentiment-handler 'test' | jq -r '.message'

# Enable debug logging
RUST_LOG=debug ./target/release/sentiment-handler 'test' 2>&1 | grep -v '^{'

# Test with minimal input
./target/release/sentiment-handler 'hello'
```

**Solutions**:
```bash
# Ensure input is not empty
if [[ -n "$input_text" ]]; then
  ./target/release/sentiment-handler "$input_text"
fi

# Validate input format
input_text=$(echo "$raw_input" | tr -d '\0' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
./target/release/sentiment-handler "$input_text"

# Check for special characters
input_text=$(echo "$raw_input" | iconv -t UTF-8 -f UTF-8 -c)
./target/release/sentiment-handler "$input_text"
```

## Shell Issues

### zsh Parse Errors

#### Issue: `zsh: parse error near \n`

**Cause**: Using double quotes with special characters

**Solutions**:
```bash
# ❌ Wrong - causes parse errors
./target/release/sentiment-handler "I love this! It's amazing."

# ✅ Correct - use single quotes
./target/release/sentiment-handler 'I love this! It'\''s amazing.'

# ✅ Alternative - escape special characters
./target/release/sentiment-handler "I love this! It's amazing."

# ✅ Use variables to avoid quoting issues
text="I love this! It's amazing."
./target/release/sentiment-handler "$text"
```

#### Issue: Environment variable conflicts

**Cause**: Shell interpreting environment variables incorrectly

**Solutions**:
```bash
# ❌ Wrong - shell expansion issues
PARSED_RESULT="{"key": "value"}" ./target/release/handler

# ✅ Correct - proper escaping
PARSED_RESULT='{"key": "value"}' ./target/release/handler

# ✅ Use here-doc for complex JSON
PARSED_RESULT=$(cat <<'EOF'
{
  "key": "value",
  "array": ["item1", "item2"]
}
EOF
) ./target/release/handler 'input'
```

### bash vs zsh Differences

#### Issue: Script works in bash but not zsh

**Solutions**:
```bash
# Force bash for script execution
#!/bin/bash

# Or ensure zsh compatibility
#!/bin/zsh
setopt BASH_REMATCH  # For regex compatibility
setopt NO_GLOB_SUBST # Prevent unwanted globbing
```

## Performance Issues

### Slow Handler Execution

#### Issue: Handlers taking too long to start

**Cause**: Dynamic linking overhead or large binary size

**Diagnosis**:
```bash
# Time handler startup
time ./target/release/sentiment-handler 'test'

# Check binary size
ls -lh target/release/sentiment-handler

# Check dynamic dependencies
ldd target/release/sentiment-handler  # Linux
otool -L target/release/sentiment-handler  # macOS
```

**Solutions**:
```bash
# Build with optimizations
export RUSTFLAGS="-C target-cpu=native"
cargo build --release

# Strip debug symbols
strip target/release/*-handler

# Use static linking (in Cargo.toml)
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
```

#### Issue: High memory usage during compilation

**Solutions**:
```bash
# Reduce parallel jobs
cargo build --release -j 1

# Use debug profile for faster compilation
cargo build --bin sentiment-handler

# Compile individual handlers
cargo build --release --bin sentiment-handler
cargo build --release --bin install-handler
```

### Handler Timeout Issues

#### Issue: Handlers hanging or timing out

**Diagnosis**:
```bash
# Set timeout for handler execution
timeout 30s ./target/release/sentiment-handler 'input'

# Check if handler is waiting for input
echo 'input' | timeout 5s ./target/release/sentiment-handler

# Monitor system resources
top -p $(pgrep sentiment-handler)
```

**Solutions**:
```bash
# Ensure proper input termination
echo 'input text' | ./target/release/sentiment-handler

# Use timeout wrapper
run_handler_with_timeout() {
  local handler="$1"
  local input="$2"
  local timeout="${3:-30}"
  
  timeout "${timeout}s" ./target/release/"$handler" "$input"
}
```

## Integration Issues

### Docker Issues

#### Issue: Handlers fail in Docker container

**Cause**: Missing dependencies or wrong architecture

**Diagnosis**:
```bash
# Check container architecture
docker run --rm your-image uname -a

# Check if handlers exist
docker run --rm your-image ls -la /usr/local/bin/

# Test handler in container
docker run --rm your-image sentiment-handler 'test'
```

**Solutions**:

**Dockerfile fixes**:
```dockerfile
# Ensure proper base image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy with proper permissions
COPY --from=builder /app/target/release/*-handler /usr/local/bin/
RUN chmod +x /usr/local/bin/*-handler
```

### CI/CD Issues

#### Issue: Handlers fail in CI environment

**Cause**: Missing dependencies or environment differences

**Solutions**:

**.github/workflows/test.yml**:
```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y build-essential libssl-dev pkg-config
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
      
      - name: Build handlers
        run: ./build_handlers.sh
      
      - name: Test handlers
        run: ./test_handlers.sh
```

### Python Integration Issues

#### Issue: `subprocess.CalledProcessError` when calling handlers

**Cause**: Handler exit codes or output format issues

**Solutions**:

```python
import subprocess
import json
import sys

def run_handler_safe(handler, input_text, parsed_result=None):
    """Safely run a callback handler with proper error handling."""
    cmd = [f'./target/release/{handler}', input_text]
    
    env = {}
    if parsed_result:
        env['PARSED_RESULT'] = json.dumps(parsed_result)
        env['PATH'] = os.environ.get('PATH', '')
    
    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            env=env,
            timeout=30,  # Add timeout
            check=False  # Don't raise on non-zero exit
        )
        
        # Parse JSON output
        try:
            output = json.loads(result.stdout)
        except json.JSONDecodeError:
            return {
                "success": False,
                "message": f"Invalid JSON output: {result.stdout}",
                "data": None,
                "execution_time_ms": 0
            }
        
        return output
        
    except subprocess.TimeoutExpired:
        return {
            "success": False,
            "message": "Handler execution timed out",
            "data": None,
            "execution_time_ms": 0
        }
    except Exception as e:
        return {
            "success": False,
            "message": f"Handler execution failed: {str(e)}",
            "data": None,
            "execution_time_ms": 0
        }
```

## Debugging Techniques

### Enable Verbose Logging

```bash
# Debug level logging
RUST_LOG=debug ./target/release/sentiment-handler 'test' 2>&1 | tee debug.log

# Trace level logging (very verbose)
RUST_LOG=trace ./target/release/sentiment-handler 'test' 2>&1 | tee trace.log

# Module-specific logging
RUST_LOG=rustlm_server::callbacks=debug ./target/release/sentiment-handler 'test'
```

### Trace Handler Execution

```bash
# Use strace to trace system calls (Linux)
strace -e trace=file ./target/release/sentiment-handler 'test'

# Use dtruss for system call tracing (macOS)
sudo dtruss -f ./target/release/sentiment-handler 'test'

# Use time to measure execution time
time ./target/release/sentiment-handler 'test'
```

### JSON Output Debugging

```bash
# Pretty-print JSON output
./target/release/sentiment-handler 'test' | jq '.'

# Extract specific fields
./target/release/sentiment-handler 'test' | jq '.success'
./target/release/sentiment-handler 'test' | jq '.message'
./target/release/sentiment-handler 'test' | jq '.data'

# Check for empty or null fields
./target/release/sentiment-handler 'test' | jq 'select(.data != null)'

# Validate JSON schema
./target/release/sentiment-handler 'test' | jq 'has("success") and has("message") and has("data") and has("execution_time_ms")'
```

### Network and File System Debugging

```bash
# Check if handlers are accessing network (should not be for most handlers)
netstat -tupln | grep $(pgrep sentiment-handler)

# Check file system access
lsof -p $(pgrep sentiment-handler)

# Monitor file descriptor usage
ls -la /proc/$(pgrep sentiment-handler)/fd/
```

### Memory and CPU Profiling

```bash
# Monitor memory usage
ps -o pid,ppid,cmd,%mem,%cpu --sort=-%mem | grep handler

# Use valgrind for memory debugging (Linux)
valgrind --tool=memcheck ./target/release/sentiment-handler 'test'

# Profile with perf (Linux)
perf record ./target/release/sentiment-handler 'test'
perf report
```

## Getting Help

### Collecting Debug Information

When reporting issues, include:

1. **System Information**:
```bash
# OS and architecture
uname -a

# Rust version
rustc --version
cargo --version

# Shell information
echo $SHELL
$SHELL --version
```

2. **Build Information**:
```bash
# Build output
./build_handlers.sh 2>&1 | tee build.log

# Handler information
ls -la target/release/ | grep handler
file target/release/sentiment-handler
```

3. **Runtime Information**:
```bash
# Handler output with debug logging
RUST_LOG=debug ./target/release/sentiment-handler 'test' 2>&1 | tee handler-debug.log

# Environment variables
env | grep -E "(RUST_|CARGO_|PATH)"
```

### Common Error Patterns

| Error Pattern | Likely Cause | Quick Fix |
|---------------|--------------|-----------|
| `parse error` | Shell quoting issues | Use single quotes |
| `permission denied` | File permissions | `chmod +x` |
| `command not found` | Path or build issues | Check PATH, rebuild |
| `Invalid JSON` | Handler crash or mixed output | Check stderr, enable logging |
| `success: false` | Input validation failure | Check input format |
| Timeout/hanging | Waiting for stdin | Provide explicit input |
| Memory errors | Insufficient resources | Reduce parallelism |
| Linking errors | Missing dependencies | Install dev packages |

For additional support, check the project repository issues or create a new issue with the debug information collected above.
