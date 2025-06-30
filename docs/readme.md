# RustLM Server - Individual Callback Handler Executables

This documentation covers the individual executable callback handlers created for the RustLM server project. Each callback handler has been transformed into a standalone executable that can be run independently.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Architecture](#architecture)
- [Available Handlers](#available-handlers)
- [Usage Guide](#usage-guide)
- [Development](#development)
- [Troubleshooting](#troubleshooting)
- [API Reference](#api-reference)

## Overview

The RustLM server originally had callback handlers embedded within the main application. This project extends that by creating **18 individual executable files** - one for each callback handler. This allows for:

- **Microservice Architecture**: Deploy handlers as separate services
- **Process Isolation**: Run handlers in isolated processes
- **Selective Deployment**: Deploy only needed handlers
- **CI/CD Integration**: Use specific handlers in build pipelines
- **CLI Tools**: Create command-line utilities for specific tasks

### Key Features

✅ **Standalone Executables** - Each handler is completely independent  
✅ **Multiple Input Methods** - Command line, stdin, environment variables  
✅ **JSON Output** - Structured, parseable results  
✅ **Proper Exit Codes** - Standard success/failure indication  
✅ **Error Handling** - Robust error reporting and logging  
✅ **Shared Codebase** - All handlers use the same underlying logic  
✅ **Easy Integration** - Simple to use in scripts and other applications  

## Quick Start

### 1. Build All Handlers

```bash
# Build all 18 handlers at once
./build_handlers.sh
```

### 2. Test All Handlers

```bash
# Run comprehensive tests on all handlers
./test_handlers.sh
```

### 3. Use Individual Handlers

```bash
# System handlers
./target/release/install-handler nodejs
./target/release/find-file-handler package.json
./target/release/google-search-handler 'rust programming'

# NLP handlers
./target/release/sentiment-handler 'I love this product!'
echo 'Long text to summarize...' | ./target/release/summarize-handler
./target/release/classify-handler 'Technical documentation about Rust'

# With custom parsed results
PARSED_RESULT='{"sentiment": "positive", "score": 0.95}' \
  ./target/release/sentiment-handler 'This is amazing!'
```

## Architecture

### Project Structure

```
rustlm-server/
├── Cargo.toml                    # Multi-binary configuration
├── src/
│   ├── lib.rs                   # Shared library exports
│   ├── callbacks.rs             # Original callback implementations
│   ├── main.rs                  # Main server application
│   ├── nlp.rs                   # NLP utilities
│   ├── udp_broadcast.rs         # UDP functionality
│   └── bin/                     # Individual handler executables
│       ├── install_handler.rs
│       ├── find_file_handler.rs
│       ├── sentiment_handler.rs
│       └── ... (15 more handlers)
├── target/release/              # Compiled executables
├── docs/                        # Documentation
├── build_handlers.sh            # Build script
├── generate_handlers.sh         # Code generation script
└── test_handlers.sh            # Test script
```

### Handler Architecture

Each handler follows the same pattern:

1. **Parse Input** - From command line args or stdin
2. **Create Context** - Build `CommandContext` with input data
3. **Execute Handler** - Run the appropriate callback handler
4. **Output Results** - Return JSON formatted results
5. **Exit** - Proper exit code based on success/failure

```rust
// Common handler pattern
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let input_text = get_input()?;
    let parsed_result = get_parsed_result()?;
    
    let context = CommandContext {
        command: "handler_name".to_string(),
        task: "handler_name".to_string(),
        input_text,
        parsed_result,
        confidence: Some(0.9),
        timestamp: Utc::now(),
        session_id: None,
    };
    
    let handler = HandlerType;
    match handler.handle(&context).await {
        Ok(result) => {
            println!("{}", serde_json::to_string_pretty(&result)?);
            std::process::exit(if result.success { 0 } else { 1 });
        }
        Err(e) => {
            error!("Handler failed: {}", e);
            std::process::exit(1);
        }
    }
}
```

## Available Handlers

### System Command Handlers (11 handlers)

| Handler | Executable | Description |
|---------|------------|-------------|
| Install | `install-handler` | Software installation commands |
| Find File | `find-file-handler` | Search for files in filesystem |
| Find Content | `find-content-handler` | Search content within files |
| Get File From | `get-file-from-handler` | Download/retrieve files |
| Show Tools | `show-tools-handler` | Display available tools |
| Open App | `open-app-handler` | Launch applications |
| Open File | `open-file-handler` | Open files with apps |
| Checkout | `checkout-handler` | Git checkout operations |
| Diff | `diff-handler` | File and Git diff operations |
| Google Search | `google-search-handler` | Perform web searches |
| Ask AI | `ask-ai-handler` | AI query processing |

### NLP Handlers (7 handlers)

| Handler | Executable | Description |
|---------|------------|-------------|
| Sentiment | `sentiment-handler` | Analyze text sentiment |
| Summarize | `summarize-handler` | Summarize text content |
| Classify | `classify-handler` | Classify text into categories |
| Extract Keywords | `extract-keywords-handler` | Extract keywords from text |
| Translate | `translate-handler` | Translate text between languages |
| Question Answer | `question-answer-handler` | Answer questions based on context |
| Natural Language | `natural-language-handler` | General NLP processing |

## Usage Guide

### Input Methods

#### 1. Command Line Arguments

```bash
./target/release/sentiment-handler 'I love this product!'
./target/release/install-handler nodejs
./target/release/find-file-handler config.json
```

**Note**: Use single quotes (`'`) instead of double quotes (`"`) to avoid zsh parsing errors.

#### 2. Standard Input

```bash
echo 'Text to analyze' | ./target/release/sentiment-handler
cat large_document.txt | ./target/release/summarize-handler
curl -s https://api.example.com/data | ./target/release/classify-handler
```

#### 3. Environment Variables

```bash
# Custom parsed results
PARSED_RESULT='{"sentiment": "positive", "score": 0.95}' \
  ./target/release/sentiment-handler 'This is great!'

# Multiple environment variables
PARSED_RESULT='{"commands": ["brew install", "apt install"]}' \
RUST_LOG=debug \
  ./target/release/install-handler python
```

### Output Format

All handlers return JSON in this format:

```json
{
  "success": true,
  "message": "Handler completed successfully",
  "data": {
    "action": "specific_action",
    "result": "handler_specific_data",
    "additional_fields": "..."
  },
  "execution_time_ms": 42
}
```

### Exit Codes

- **0** - Success
- **1** - Error or failure

### Examples by Category

#### System Operations

```bash
# Install software
./target/release/install-handler rust
./target/release/install-handler python3

# File operations
./target/release/find-file-handler package.json
./target/release/find-content-handler 'TODO'
./target/release/open-file-handler document.pdf

# Git operations
./target/release/checkout-handler feature-branch
./target/release/diff-handler HEAD~1

# Web search
./target/release/google-search-handler 'rust async programming'
```

#### NLP Operations

```bash
# Sentiment analysis
./target/release/sentiment-handler 'I absolutely love this!'
./target/release/sentiment-handler 'This is terrible'

# Text processing
./target/release/summarize-handler 'Very long text that needs summarization...'
./target/release/classify-handler 'Technical documentation about machine learning'
./target/release/extract-keywords-handler 'Rust programming language systems development'

# Translation
./target/release/translate-handler 'Hello world'
./target/release/translate-handler 'Bonjour le monde'
```

#### Advanced Usage with Custom Data

```bash
# Sentiment with detailed analysis
PARSED_RESULT='{"sentiment": "positive", "score": 0.95, "emotions": ["joy", "satisfaction"]}' \
  ./target/release/sentiment-handler 'This is fantastic!'

# Install with specific package manager
PARSED_RESULT='{"package_manager": "brew", "suggested_commands": ["brew install nodejs"]}' \
  ./target/release/install-handler nodejs

# Classification with categories
PARSED_RESULT='{"categories": ["technical", "documentation"], "confidence": 0.9}' \
  ./target/release/classify-handler 'API documentation for REST services'
```

## Development

### Adding New Handlers

1. **Update Cargo.toml**:
```toml
[[bin]]
name = "new-handler"
path = "src/bin/new_handler.rs"
```

2. **Create Handler File**:
```bash
cp src/bin/sentiment_handler.rs src/bin/new_handler.rs
# Edit the file to use the appropriate handler type and command
```

3. **Update Build Script**:
```bash
# Add to build_handlers.sh binaries array
"new-handler"
```

4. **Test the Handler**:
```bash
cargo build --release --bin new-handler
./target/release/new-handler test-input
```

### Code Generation

The `generate_handlers.sh` script can automatically create new handlers:

```bash
# Edit the script to add new commands to the arrays
system_commands=("new_command")
nlp_commands=("new_nlp_command")

# Regenerate all handlers
./generate_handlers.sh
```

### Building and Testing

```bash
# Build specific handler
cargo build --release --bin sentiment-handler

# Build all handlers
./build_handlers.sh

# Test all handlers
./test_handlers.sh

# Debug specific handler
RUST_LOG=debug ./target/release/sentiment-handler 'test'
```

## Troubleshooting

### Common Issues

#### 1. zsh Parse Error

**Problem**: `zsh: parse error near \n`

**Solution**: Use single quotes instead of double quotes:
```bash
# ❌ Wrong
./target/release/sentiment-handler "I love this!"

# ✅ Correct
./target/release/sentiment-handler 'I love this!'
```

#### 2. Handler Not Found

**Problem**: `command not found: ./target/release/handler-name`

**Solution**: Build the handlers first:
```bash
./build_handlers.sh
```

#### 3. Permission Denied

**Problem**: `permission denied: ./target/release/handler-name`

**Solution**: Check file permissions:
```bash
ls -la target/release/
chmod +x target/release/handler-name
```

#### 4. JSON Parse Error

**Problem**: Handler fails with JSON parsing error

**Solution**: Check `PARSED_RESULT` format:
```bash
# ❌ Invalid JSON
PARSED_RESULT='{"invalid": json}' ./target/release/handler

# ✅ Valid JSON
PARSED_RESULT='{"valid": "json"}' ./target/release/handler
```

### Debugging

```bash
# Enable debug logging
RUST_LOG=debug ./target/release/sentiment-handler 'test'

# Trace execution
RUST_LOG=trace ./target/release/sentiment-handler 'test'

# Check handler output
./target/release/sentiment-handler 'test' | jq '.'
```

## API Reference

### CommandContext Structure

```rust
pub struct CommandContext {
    pub command: String,           // Handler command name
    pub task: String,             // Task description
    pub input_text: String,       // Input text to process
    pub parsed_result: String,    // JSON string with parsed data
    pub confidence: Option<f32>,  // Confidence score (0.0-1.0)
    pub timestamp: DateTime<Utc>, // Execution timestamp
    pub session_id: Option<String>, // Optional session identifier
}
```

### CallbackResult Structure

```rust
pub struct CallbackResult {
    pub success: bool,                    // Operation success status
    pub message: String,                  // Human-readable message
    pub data: Option<serde_json::Value>, // Handler-specific data
    pub execution_time_ms: u64,          // Execution time in milliseconds
}
```

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `PARSED_RESULT` | Custom JSON data for handler | `'{"sentiment": "positive"}'` |
| `RUST_LOG` | Logging level | `debug`, `info`, `warn`, `error` |

### Handler-Specific Data Formats

#### System Handlers

```json
{
  "action": "install_ready",
  "package": "nodejs",
  "suggested_commands": ["brew install nodejs"],
  "next_steps": "Commands are ready for execution"
}
```

#### NLP Handlers

```json
{
  "action": "sentiment_analyzed",
  "text": "I love this!",
  "analysis": {
    "sentiment": "positive",
    "score": 0.95
  },
  "confidence": 0.9
}
```

---

For more detailed information, see the individual documentation files in this folder:

- [Installation Guide](installation.md)
- [Usage Examples](examples.md)
- [Development Guide](development.md)
- [API Reference](api-reference.md)
- [Troubleshooting](troubleshooting.md)
