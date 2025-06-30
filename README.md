# RustLM Server

A Rust-based REST API server with Natural Language Processing capabilities.

## Features

- **REST API Endpoints**: Clean HTTP API for text processing
- **Multiple NLP Tasks**: Sentiment analysis, text summarization, classification, keyword extraction, translation, and question answering
- **Async Processing**: Built with Tokio for high-performance async operations
- **JSON API**: Full JSON request/response support
- **CORS Support**: Cross-origin resource sharing enabled
- **Logging**: Structured logging with tracing
- **Error Handling**: Comprehensive error responses

## Quick Start

### Prerequisites

- Rust 1.70+ installed
- Cargo package manager

### Installation

```bash
# Clone or navigate to the project directory
cd rustlm-server

# Build the project
cargo build --release

# Run the server
cargo run
```

The server will start on `http://localhost:3000`

## API Endpoints

### Health Check
```bash
GET /health
```

Response:
```json
{
  "status": "healthy",
  "service": "rustlm-server",
  "version": "0.1.0"
}
```

### Process Text (General)
```bash
POST /process
Content-Type: application/json

{
  "text": "Your text to process",
  "task": "sentiment"
}
```

### Process Text (Task-Specific)
```bash
POST /process/sentiment
Content-Type: application/json

{
  "text": "I love this amazing product!"
}
```

### List Available Models/Tasks
```bash
GET /models
```

Response:
```json
{
  "available_tasks": [
    "sentiment",
    "summarize", 
    "classify",
    "extract_keywords",
    "translate",
    "question_answer"
  ],
  "description": "List of available NLP processing tasks"
}
```

## Available Tasks

### 🧠 NLP Tasks

#### 1. Sentiment Analysis
Analyzes the emotional tone of text.

**Task**: `sentiment`

**Example**:
```bash
curl -X POST http://localhost:3000/process/sentiment \
  -H "Content-Type: application/json" \
  -d '{"text": "I love this amazing product!"}'
```

**Response**:
```json
{
  "id": "uuid-here",
  "input_text": "I love this amazing product!",
  "task": "sentiment",
  "result": "{\"sentiment\": \"positive\", \"positive_score\": 2, \"negative_score\": 0}",
  "confidence": 0.9,
  "processing_time_ms": 15
}
```

#### 2. Text Summarization
Creates a concise summary of longer text.

**Task**: `summarize`

#### 3. Text Classification
Categorizes text into predefined categories.

**Task**: `classify`
**Categories**: technology, business, sports, health, entertainment, general

#### 4. Keyword Extraction
Extracts the most important keywords from text.

**Task**: `extract_keywords`

#### 5. Translation (Mock)
Placeholder for translation functionality.

**Task**: `translate`

#### 6. Question Answering (Mock)
Placeholder for question answering functionality.

**Task**: `question_answer`

### 🛠️ System Command Tasks

#### 7. Install Packages
Provides installation commands for various package managers.

**Task**: `install`

**Example**:
```bash
curl -X POST http://localhost:3000/process/install \
  -H "Content-Type: application/json" \
  -d '{"text": "nodejs"}'
```

**Response**:
```json
{
  "command": "install",
  "package": "nodejs",
  "suggested_commands": [
    "brew install nodejs",
    "npm install nodejs",
    "cargo install nodejs",
    "pip install nodejs"
  ]
}
```

#### 8. Find Files
Provides commands to find files by name.

**Task**: `find_file`

**Example**:
```bash
curl -X POST http://localhost:3000/process/find_file \
  -H "Content-Type: application/json" \
  -d '{"text": "main.rs"}'
```

#### 9. Find Content
Provides commands to search for content within files.

**Task**: `find_content`

**Example**:
```bash
curl -X POST http://localhost:3000/process/find_content \
  -H "Content-Type: application/json" \
  -d '{"text": "TODO"}'
```

#### 10. Get File From Source
Provides commands to download or copy files.

**Task**: `get_file_from`

**Example**:
```bash
curl -X POST http://localhost:3000/process/get_file_from \
  -H "Content-Type: application/json" \
  -d '{"text": "https://example.com/file.txt"}'
```

#### 11. Show Tools
Lists available development tools by category.

**Task**: `show_tools`

**Categories**: development, system, file_management, network, text_editors

**Example**:
```bash
curl -X POST http://localhost:3000/process/show_tools \
  -H "Content-Type: application/json" \
  -d '{"text": "development"}'
```

#### 12. Open Applications
Provides commands to open macOS applications.

**Task**: `open_app`

**Example**:
```bash
curl -X POST http://localhost:3000/process/open_app \
  -H "Content-Type: application/json" \
  -d '{"text": "Visual Studio Code"}'
```

#### 13. Open Files
Provides commands to open files with various applications.

**Task**: `open_file`

**Example**:
```bash
curl -X POST http://localhost:3000/process/open_file \
  -H "Content-Type: application/json" \
  -d '{"text": "/path/to/file.txt"}'
```

#### 14. Git Checkout
Provides git checkout commands for branches and commits.

**Task**: `checkout`

**Example**:
```bash
curl -X POST http://localhost:3000/process/checkout \
  -H "Content-Type: application/json" \
  -d '{"text": "feature-branch"}'
```

#### 15. Git Diff
Provides git diff commands for comparing files and commits.

**Task**: `diff`

**Example**:
```bash
curl -X POST http://localhost:3000/process/diff \
  -H "Content-Type: application/json" \
  -d '{"text": "HEAD~1 HEAD"}'
```

## Response Format

All successful responses follow this format:

```json
{
  "id": "request-uuid",
  "input_text": "original input text",
  "task": "task_name",
  "result": "processed result",
  "confidence": 0.85,
  "processing_time_ms": 120
}
```

Error responses:

```json
{
  "error": "error_type",
  "message": "Detailed error message"
}
```

## Development

### Running in Development Mode
```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Run clippy for linting
cargo clippy
```

### Project Structure
```
src/
├── main.rs           # Server setup and API routes
├── nlp.rs           # NLP processing logic
└── lib.rs           # Library exports (if needed)

Cargo.toml           # Dependencies and project config
README.md           # This file
```

## Configuration

The server runs on `0.0.0.0:3000` by default. You can modify this in `src/main.rs`.

Environment variables:
- `RUST_LOG`: Set logging level (e.g., `debug`, `info`, `warn`, `error`)

## Extending the NLP Processor

To add new NLP tasks:

1. Add the task name to `available_tasks` in `NlpProcessor::new()`
2. Add a match case in `NlpProcessor::process()`
3. Implement the processing function

Example:
```rust
async fn new_task(&self, text: &str) -> Result<(String, Option<f32>)> {
    // Your processing logic here
    Ok(("result".to_string(), Some(0.8)))
}
```

## Production Considerations

This implementation uses simple rule-based NLP for demonstration. For production use:

1. **Replace with ML Models**: Integrate actual transformer models using candle-transformers
2. **Add Authentication**: Implement API key authentication
3. **Rate Limiting**: Add request rate limiting
4. **Caching**: Cache frequently processed texts
5. **Database**: Store processing history and results
6. **Monitoring**: Add metrics and health monitoring
7. **Docker**: Containerize for easy deployment

## License

MIT License
