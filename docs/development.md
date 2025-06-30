# Development Guide

This guide covers how to develop, extend, and maintain the individual callback handler executables.

## Table of Contents

- [Development Setup](#development-setup)
- [Adding New Handlers](#adding-new-handlers)
- [Modifying Existing Handlers](#modifying-existing-handlers)
- [Testing](#testing)
- [Code Organization](#code-organization)
- [Best Practices](#best-practices)
- [Contributing](#contributing)

## Development Setup

### Prerequisites

- **Rust**: 1.70 or later
- **Git**: For version control
- **Editor**: VS Code with rust-analyzer extension recommended
- **Tools**: `jq` for JSON processing, `cargo-watch` for development

### Setting Up Development Environment

```bash
# Clone the repository
git clone <repository-url>
cd rustlm-server

# Install development dependencies
cargo install cargo-watch cargo-expand cargo-audit

# Install VS Code extensions (if using VS Code)
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb

# Build all handlers in debug mode for faster iteration
cargo build

# Run tests to ensure everything works
cargo test
```

### Development Workflow

```bash
# Watch for changes and rebuild automatically
cargo watch -x 'build --bin sentiment-handler'

# Run specific handler during development
cargo run --bin sentiment-handler 'test input'

# Run with debug logging
RUST_LOG=debug cargo run --bin sentiment-handler 'test input'

# Format code
cargo fmt

# Check for issues
cargo clippy

# Run tests
cargo test
```

## Adding New Handlers

### Step 1: Update Cargo.toml

Add a new binary configuration:

```toml
[[bin]]
name = "new-handler"
path = "src/bin/new_handler.rs"
```

### Step 2: Create Handler File

Create the handler file at `src/bin/new_handler.rs`:

```rust
use anyhow::Result;
use chrono::Utc;
use rustlm_server::{SystemCommandHandler, CommandContext}; // or NlpCallbackHandler
use serde_json;
use std::env;
use std::io::{self, Read};
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting new handler");
    
    // Get input from command line arguments or stdin
    let input_text = get_input()?;
    let parsed_result = get_parsed_result()?;
    
    // Create command context
    let context = CommandContext {
        command: "new_command".to_string(),
        task: "new_command".to_string(),
        input_text,
        parsed_result,
        confidence: Some(0.9),
        timestamp: Utc::now(),
        session_id: None,
    };
    
    // Create handler and execute
    let handler = SystemCommandHandler; // or NlpCallbackHandler
    match handler.handle(&context).await {
        Ok(result) => {
            println!("{}", serde_json::to_string_pretty(&result)?);
            if result.success {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("Handler failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_input() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        Ok(args[1..].join(" "))
    } else {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer.trim().to_string())
    }
}

fn get_parsed_result() -> Result<String> {
    // You can get this from environment variable or another argument
    if let Ok(result) = env::var("PARSED_RESULT") {
        Ok(result)
    } else {
        // Default parsed result for new_command command
        Ok(r#"{"command": "new_command", "suggested_commands": ["echo 'Add specific commands for new command here'"]}"#.to_string())
    }
}
```

### Step 3: Add to Callback Handler

If adding a new command to existing handlers, update `src/callbacks.rs`:

#### For System Commands

Add to `SystemCommandHandler::handle()`:

```rust
match context.command.as_str() {
    // ... existing commands ...
    "new_command" => self.handle_new_command_callback(context).await,
    _ => Err(anyhow!("Unsupported command: {}", context.command)),
}
```

Add to `get_supported_commands()`:

```rust
vec![
    // ... existing commands ...
    "new_command".to_string(),
]
```

Implement the handler method:

```rust
async fn handle_new_command_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
    info!("Processing new command callback for: {}", context.input_text);
    
    let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
    
    Ok(CallbackResult {
        success: true,
        message: format!("New command callback processed for: {}", context.input_text),
        data: Some(serde_json::json!({
            "action": "new_command_ready",
            "input": context.input_text,
            "commands": parsed_data.get("suggested_commands"),
            "status": "new_command_prepared"
        })),
        execution_time_ms: 0,
    })
}
```

#### For NLP Commands

Similar process but in `NlpCallbackHandler`.

### Step 4: Update Build Scripts

Add to `build_handlers.sh`:

```bash
binaries=(
    # ... existing binaries ...
    "new-handler"
)
```

Add to `generate_handlers.sh` arrays:

```bash
system_commands=(
    # ... existing commands ...
    "new_command"
)
```

### Step 5: Add Tests

Create tests in the handler file or in `src/callbacks.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_command_callback() {
        let handler = SystemCommandHandler;
        let context = CommandContext {
            command: "new_command".to_string(),
            task: "new_command".to_string(),
            input_text: "test input".to_string(),
            parsed_result: r#"{"command": "new_command", "suggested_commands": ["echo test"]}"#.to_string(),
            confidence: Some(0.9),
            timestamp: chrono::Utc::now(),
            session_id: None,
        };

        let result = handler.handle(&context).await.unwrap();
        assert!(result.success);
        assert!(result.message.contains("New command callback processed"));
    }
}
```

### Step 6: Build and Test

```bash
# Build the new handler
cargo build --release --bin new-handler

# Test the handler
./target/release/new-handler 'test input'

# Add to test script
echo "Testing new handler..." >> test_handlers.sh
echo "./target/release/new-handler 'test input' | jq '.message'" >> test_handlers.sh
```

## Modifying Existing Handlers

### Changing Handler Behavior

1. **Modify the callback implementation** in `src/callbacks.rs`
2. **Update tests** to reflect the new behavior
3. **Rebuild the affected handlers**
4. **Test the changes**

Example - Adding validation to sentiment handler:

```rust
async fn handle_sentiment_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
    info!("Processing sentiment analysis callback");
    
    // Add input validation
    if context.input_text.trim().is_empty() {
        return Ok(CallbackResult {
            success: false,
            message: "Input text cannot be empty".to_string(),
            data: None,
            execution_time_ms: 0,
        });
    }
    
    if context.input_text.len() < 3 {
        return Ok(CallbackResult {
            success: false,
            message: "Input text too short for sentiment analysis".to_string(),
            data: None,
            execution_time_ms: 0,
        });
    }
    
    let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
    
    Ok(CallbackResult {
        success: true,
        message: "Sentiment analysis completed".to_string(),
        data: Some(serde_json::json!({
            "action": "sentiment_analyzed",
            "text": context.input_text,
            "analysis": parsed_data,
            "confidence": context.confidence,
            "word_count": context.input_text.split_whitespace().count()
        })),
        execution_time_ms: 0,
    })
}
```

### Adding Configuration Options

Add support for configuration via `PARSED_RESULT`:

```rust
async fn handle_sentiment_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
    info!("Processing sentiment analysis callback");
    
    let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
    
    // Extract configuration options
    let include_emotions = parsed_data.get("include_emotions")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    let confidence_threshold = parsed_data.get("confidence_threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.5);
    
    // Use configuration in processing
    let mut response = serde_json::json!({
        "action": "sentiment_analyzed",
        "text": context.input_text,
        "analysis": parsed_data,
        "confidence": context.confidence
    });
    
    if include_emotions {
        response["emotions"] = serde_json::json!(["joy", "satisfaction"]);
    }
    
    if context.confidence.unwrap_or(0.0) < confidence_threshold as f32 {
        response["warning"] = serde_json::json!("Low confidence result");
    }
    
    Ok(CallbackResult {
        success: true,
        message: "Sentiment analysis completed".to_string(),
        data: Some(response),
        execution_time_ms: 0,
    })
}
```

## Testing

### Unit Tests

Add tests to `src/callbacks.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sentiment_empty_input() {
        let handler = NlpCallbackHandler;
        let context = CommandContext {
            command: "sentiment".to_string(),
            task: "sentiment".to_string(),
            input_text: "".to_string(),
            parsed_result: r#"{"sentiment": "neutral"}"#.to_string(),
            confidence: Some(0.9),
            timestamp: chrono::Utc::now(),
            session_id: None,
        };

        let result = handler.handle(&context).await.unwrap();
        assert!(!result.success);
        assert!(result.message.contains("empty"));
    }

    #[tokio::test]
    async fn test_sentiment_with_config() {
        let handler = NlpCallbackHandler;
        let context = CommandContext {
            command: "sentiment".to_string(),
            task: "sentiment".to_string(),
            input_text: "I love this!".to_string(),
            parsed_result: r#"{"sentiment": "positive", "include_emotions": true}"#.to_string(),
            confidence: Some(0.9),
            timestamp: chrono::Utc::now(),
            session_id: None,
        };

        let result = handler.handle(&context).await.unwrap();
        assert!(result.success);
        assert!(result.data.is_some());
        
        let data = result.data.unwrap();
        assert!(data["emotions"].is_array());
    }
}
```

### Integration Tests

Create integration tests in `tests/` directory:

```rust
// tests/handler_integration.rs
use std::process::Command;
use serde_json::Value;

#[test]
fn test_sentiment_handler_integration() {
    let output = Command::new("./target/release/sentiment-handler")
        .arg("I love this product!")
        .output()
        .expect("Failed to execute handler");

    assert!(output.status.success());
    
    let json: Value = serde_json::from_slice(&output.stdout)
        .expect("Invalid JSON output");
    
    assert_eq!(json["success"], true);
    assert!(json["message"].as_str().unwrap().contains("Sentiment"));
    assert!(json["data"].is_object());
}

#[test]
fn test_handler_with_env_var() {
    let output = Command::new("./target/release/sentiment-handler")
        .arg("Test input")
        .env("PARSED_RESULT", r#"{"sentiment": "positive", "score": 0.95}"#)
        .output()
        .expect("Failed to execute handler");

    assert!(output.status.success());
    
    let json: Value = serde_json::from_slice(&output.stdout)
        .expect("Invalid JSON output");
    
    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["analysis"]["score"], 0.95);
}
```

### Performance Tests

Create performance benchmarks:

```rust
// benches/handler_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustlm_server::{CommandContext, NlpCallbackHandler};
use chrono::Utc;

async fn sentiment_benchmark() {
    let handler = NlpCallbackHandler;
    let context = CommandContext {
        command: "sentiment".to_string(),
        task: "sentiment".to_string(),
        input_text: "This is a test sentence for benchmarking purposes.".to_string(),
        parsed_result: r#"{"sentiment": "neutral"}"#.to_string(),
        confidence: Some(0.9),
        timestamp: Utc::now(),
        session_id: None,
    };

    let _result = handler.handle(&context).await.unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("sentiment_handler", |b| {
        b.to_async(&rt).iter(|| sentiment_benchmark())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

Run benchmarks:

```bash
cargo bench
```

## Code Organization

### Project Structure

```
src/
├── lib.rs              # Library exports
├── main.rs             # Main server application
├── callbacks.rs        # Callback implementations
├── nlp.rs              # NLP utilities
├── udp_broadcast.rs    # UDP functionality
└── bin/                # Individual handler executables
    ├── sentiment_handler.rs
    ├── install_handler.rs
    └── ...
```

### Adding New Modules

1. **Create new module** in `src/`
2. **Add to lib.rs** exports
3. **Update handlers** to use new functionality

Example - Adding a validation module:

```rust
// src/validation.rs
use anyhow::{Result, anyhow};

pub struct InputValidator;

impl InputValidator {
    pub fn validate_text_input(input: &str) -> Result<()> {
        if input.trim().is_empty() {
            return Err(anyhow!("Input cannot be empty"));
        }
        
        if input.len() > 10000 {
            return Err(anyhow!("Input too long (max 10000 characters)"));
        }
        
        // Check for valid UTF-8
        if !input.is_ascii() && input.chars().any(|c| c.is_control()) {
            return Err(anyhow!("Input contains invalid characters"));
        }
        
        Ok(())
    }
    
    pub fn sanitize_input(input: &str) -> String {
        input
            .chars()
            .filter(|c| !c.is_control() || c.is_whitespace())
            .collect::<String>()
            .trim()
            .to_string()
    }
}
```

Add to `lib.rs`:

```rust
pub mod validation;
pub use validation::InputValidator;
```

Use in handlers:

```rust
use rustlm_server::{InputValidator, ...};

// In handler implementation
let sanitized_input = InputValidator::sanitize_input(&context.input_text);
InputValidator::validate_text_input(&sanitized_input)?;
```

## Best Practices

### Error Handling

```rust
// Good - Specific error messages
if context.input_text.trim().is_empty() {
    return Ok(CallbackResult {
        success: false,
        message: "Input text cannot be empty for sentiment analysis".to_string(),
        data: None,
        execution_time_ms: 0,
    });
}

// Good - Wrap external errors
let parsed_data = serde_json::from_str(&context.parsed_result)
    .map_err(|e| anyhow!("Failed to parse PARSED_RESULT: {}", e))?;

// Good - Provide fallback values
let confidence_threshold = parsed_data.get("confidence_threshold")
    .and_then(|v| v.as_f64())
    .unwrap_or(0.5);
```

### Logging

```rust
use tracing::{info, warn, error, debug};

// Log important events
info!("Processing {} callback for input length: {}", 
      context.command, context.input_text.len());

// Log warnings for unusual conditions
if context.confidence.unwrap_or(1.0) < 0.5 {
    warn!("Low confidence input: {}", context.confidence.unwrap());
}

// Debug logging for development
debug!("Parsed result: {}", context.parsed_result);

// Error logging
error!("Failed to process {}: {}", context.command, error);
```

### JSON Response Structure

```rust
// Consistent response structure
Ok(CallbackResult {
    success: true,
    message: format!("{} completed successfully", operation_name),
    data: Some(serde_json::json!({
        "action": "operation_completed",
        "input": context.input_text,
        "result": actual_result,
        "metadata": {
            "version": "1.0",
            "timestamp": chrono::Utc::now(),
            "processing_time_ms": processing_time
        }
    })),
    execution_time_ms: total_time,
})
```

### Input Validation

```rust
// Validate input early
fn validate_input(context: &CommandContext) -> Result<()> {
    if context.input_text.trim().is_empty() {
        return Err(anyhow!("Input cannot be empty"));
    }
    
    if context.input_text.len() > 10000 {
        return Err(anyhow!("Input too long"));
    }
    
    Ok(())
}

// Use at start of handler
validate_input(context)?;
```

### Configuration Management

```rust
#[derive(Debug, Deserialize)]
struct HandlerConfig {
    #[serde(default = "default_confidence_threshold")]
    confidence_threshold: f64,
    
    #[serde(default)]
    include_metadata: bool,
    
    #[serde(default = "default_max_length")]
    max_length: usize,
}

fn default_confidence_threshold() -> f64 { 0.5 }
fn default_max_length() -> usize { 1000 }

// Parse configuration from PARSED_RESULT
let config: HandlerConfig = serde_json::from_str(&context.parsed_result)
    .unwrap_or_default();
```

## Contributing

### Development Workflow

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/new-handler`
3. **Make changes following the guidelines above**
4. **Add tests for new functionality**
5. **Ensure all tests pass**: `cargo test`
6. **Format code**: `cargo fmt`
7. **Check for issues**: `cargo clippy`
8. **Update documentation** if needed
9. **Submit a pull request**

### Code Review Checklist

- [ ] New functionality has tests
- [ ] Error handling is appropriate
- [ ] Logging is consistent
- [ ] Documentation is updated
- [ ] Performance impact is considered
- [ ] Backward compatibility is maintained
- [ ] Security implications are considered

### Release Process

1. **Update version numbers** in `Cargo.toml`
2. **Update CHANGELOG.md** with new features and fixes
3. **Build and test all handlers**: `./build_handlers.sh && ./test_handlers.sh`
4. **Tag the release**: `git tag v1.0.0`
5. **Build release binaries** for distribution
6. **Update documentation** with any API changes

### Debugging During Development

```bash
# Build with debug info
cargo build

# Run with detailed logging
RUST_LOG=debug cargo run --bin sentiment-handler 'test'

# Use debugger (VS Code with CodeLLDB)
# Set breakpoints and run in debug mode

# Profile memory usage
valgrind --tool=massif cargo run --bin sentiment-handler 'test'

# Check for memory leaks
valgrind --tool=memcheck cargo run --bin sentiment-handler 'test'
```

This development guide should help you effectively work with and extend the callback handler system. Remember to always test thoroughly and follow the established patterns when making changes.
