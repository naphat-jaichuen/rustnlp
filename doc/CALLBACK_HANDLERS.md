# Callback Handlers

This project provides individual executable files for each callback handler, allowing you to run specific callback operations independently.

## Building Handlers

To build all callback handlers:

```bash
./build_handlers.sh
```

This will create executable files in the `target/release/` directory.

## Available Handlers

### System Command Handlers

- **install-handler** - Handle software installation commands
- **find-file-handler** - Search for files in the filesystem
- **find-content-handler** - Search for content within files
- **get-file-from-handler** - Download or retrieve files from sources
- **show-tools-handler** - Display available tools and utilities
- **open-app-handler** - Launch applications
- **open-file-handler** - Open files with appropriate applications
- **checkout-handler** - Git checkout operations
- **diff-handler** - File and Git diff operations
- **google-search-handler** - Perform Google searches
- **ask-ai-handler** - AI query processing

### NLP (Natural Language Processing) Handlers

- **sentiment-handler** - Analyze text sentiment
- **summarize-handler** - Summarize text content
- **classify-handler** - Classify text into categories
- **extract-keywords-handler** - Extract keywords from text
- **translate-handler** - Translate text between languages
- **question-answer-handler** - Answer questions based on context
- **natural-language-handler** - General natural language processing

## Usage

### Command Line Arguments

Each handler can accept input as command line arguments:

```bash
./target/release/install-handler rust
./target/release/sentiment-handler 'I love this product!'
./target/release/find-file-handler config.json
```

### Standard Input

Handlers can also read from standard input:

```bash
echo 'I love this product!' | ./target/release/sentiment-handler
cat mytext.txt | ./target/release/summarize-handler
```

### Environment Variables

You can provide parsed results via environment variables:

```bash
PARSED_RESULT='{"command": "install", "package": "rust"}' ./target/release/install-handler rust
```

## Output Format

All handlers output JSON results in the following format:

```json
{
  "success": true,
  "message": "Handler completed successfully",
  "data": {
    "action": "specific_action",
    "result": "specific_result_data"
  },
  "execution_time_ms": 42
}
```

## Exit Codes

- **0** - Success
- **1** - Error or failure

## Examples

### Install Handler
```bash
# Install a package
./target/release/install-handler nodejs

# With custom parsed result
PARSED_RESULT='{"suggested_commands": ["brew install nodejs", "npm install -g nodejs"]}' \
  ./target/release/install-handler nodejs
```

### Sentiment Analysis
```bash
# Analyze sentiment
./target/release/sentiment-handler 'This is amazing!'

# From file
cat reviews.txt | ./target/release/sentiment-handler
```

### File Search
```bash
# Find a file
./target/release/find-file-handler package.json

# Find content
./target/release/find-content-handler 'TODO'
```

## Integration

These handlers can be integrated into:

- Shell scripts
- CI/CD pipelines  
- Other applications via process execution
- Microservice architectures
- Command-line tools

## Development

To add new handlers:

1. Add the binary configuration to `Cargo.toml`
2. Create the handler file in `src/bin/`
3. Update the build script
4. Test the handler

Each handler follows the same basic pattern:
- Parse input from arguments or stdin
- Create a `CommandContext`
- Execute the appropriate callback handler
- Output JSON result
- Exit with appropriate code
