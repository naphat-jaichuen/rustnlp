# RustLM Server

A comprehensive Rust-based platform providing both REST API server functionality and individual executable callback handlers for Natural Language Processing and system automation tasks.

## 🚀 Two Ways to Use RustLM

### 1. REST API Server Mode
Traditional server deployment with HTTP endpoints for text processing and system automation.

### 2. Individual Handler Executables ⭐ NEW!
Standalone executable files for each callback handler, enabling microservice architecture, CLI tools, and process isolation.

## Features

- **REST API Endpoints**: Clean HTTP API for text processing
- **Multiple NLP Tasks**: Sentiment analysis, text summarization, classification, keyword extraction, translation, and question answering
- **Async Processing**: Built with Tokio for high-performance async operations
- **JSON API**: Full JSON request/response support
- **CORS Support**: Cross-origin resource sharing enabled
- **Logging**: Structured logging with tracing
- **Error Handling**: Comprehensive error responses
- **UDP Broadcast Discovery**: Automatic server discovery with shared key validation

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

The server will start on `http://localhost:3000` and automatically begin broadcasting its availability via UDP.

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

#### 16. Google Search
Generates Google search URLs and commands to open them.

**Task**: `google_search`

**Example**:
```bash
curl -X POST http://localhost:3000/process/google_search \
  -H "Content-Type: application/json" \
  -d '{"text": "rust programming language tutorial"}'
```

**Response**:
```json
{
  "command": "google_search",
  "query": "rust programming language tutorial",
  "google_url": "https://www.google.com/search?q=rust+programming+language+tutorial",
  "suggested_commands": [
    "open 'https://www.google.com/search?q=rust+programming+language+tutorial'",
    "curl -s 'https://www.google.com/search?q=rust+programming+language+tutorial' | grep -i title",
    "python3 -m webbrowser 'https://www.google.com/search?q=rust+programming+language+tutorial'",
    "osascript -e \"open location \"https://www.google.com/search?q=rust+programming+language+tutorial\"\""
  ]
}
```

#### 17. Ask AI (Azure OpenAI)
Sends questions to Azure OpenAI and returns AI-generated responses.

**Task**: `ask_ai`

**Setup Required**:
Set these environment variables before using:
```bash
export AZURE_OPENAI_ENDPOINT="https://your-resource.openai.azure.com"
export AZURE_OPENAI_API_KEY="your-api-key"
export AZURE_OPENAI_DEPLOYMENT="gpt-35-turbo"
```

**Example**:
```bash
curl -X POST http://localhost:3000/process/ask_ai \
  -H "Content-Type: application/json" \
  -d '{"text": "What is the best way to learn Rust programming?"}'
```

**Response (Setup Required)**:
```json
{
  "command": "ask_ai",
  "question": "What is the best way to learn Rust programming?",
  "status": "setup_required",
  "message": "To use Azure OpenAI, set these environment variables...",
  "curl_example": "curl -X POST 'https://your-resource.openai.azure.com'/openai/deployments/gpt-35-turbo/chat/completions?api-version=2024-02-15-preview -H 'Content-Type: application/json' -H 'api-key: YOUR_API_KEY' -d '{\"messages\": [{\"role\": \"user\", \"content\": \"What is the best way to learn Rust programming?\"}], \"max_tokens\": 1000}'"
}
```

**Response (With Azure Setup)**:
```json
{
  "command": "ask_ai",
  "question": "What is the best way to learn Rust programming?",
  "answer": "The best way to learn Rust programming is to start with the official Rust Book...",
  "source": "azure_openai"
}
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

## UDP Server Discovery

The server automatically broadcasts its availability via UDP on port 8888. This allows client applications to discover running servers on the local network.

### Server Broadcast
The server sends a JSON message every 30 seconds:
```json
{
  "service": "rustlm-service",
  "ip": "192.168.1.100",
  "port": 3000,
  "key": "SECRETKEY123"
}
```

### Client Discovery

**Passive Discovery** (listens for periodic announcements):
```bash
# Run the client discovery example
cargo run --example client_discovery
```

**Active Discovery** (sends discovery requests):
```bash
# Send discovery request and wait for responses
cargo run --example client_request_discovery
```

Both clients will validate the shared key and only accept servers with the correct key.

### Announcement Modes

You can configure how the server announces itself in `src/main.rs`:

**1. Periodic Mode (Default)**: Announces every N seconds continuously
```rust
let announcement_mode = udp_broadcast::AnnouncementMode::Periodic(30); // Every 30 seconds
```

**2. On-Request Mode**: Only responds when clients send discovery requests (no periodic announcements)
```rust
let announcement_mode = udp_broadcast::AnnouncementMode::OnRequest;
```

**3. Limited Mode**: Announces N times then stops
```rust
let announcement_mode = udp_broadcast::AnnouncementMode::Limited(10, 5); // 5 announcements every 10 seconds
```

### Key Configuration
The shared key is defined in `src/main.rs`. Both server and client must use the same key:
```rust
let shared_key = "SECRETKEY123"; // Change this to your own key
```

## ⭐ Individual Handler Executables

In addition to the REST API server, RustLM now provides **18 individual executable files** - one for each callback handler. This enables microservice architecture, CLI tools, and process isolation.

### 🚀 Quick Start with Individual Handlers

#### 1. Build All Handlers
```bash
# Build all 18 handlers at once
./build_handlers.sh
```

#### 2. Test All Handlers
```bash
# Run comprehensive tests on all handlers
./test_handlers.sh
```

#### 3. Use Individual Handlers
```bash
# System handlers
./target/release/install-handler nodejs
./target/release/find-file-handler package.json
./target/release/google-search-handler 'rust programming'

# NLP handlers
./target/release/sentiment-handler 'I love this product!'
echo 'Long text to summarize...' | ./target/release/summarize-handler
./target/release/classify-handler 'Technical documentation about Rust'

# With custom configuration
PARSED_RESULT='{"sentiment": "positive", "score": 0.95}' \
  ./target/release/sentiment-handler 'This is amazing!'
```

### 📁 Available Individual Handlers

#### System Command Handlers (11 executables)
- `install-handler` - Software installation commands
- `find-file-handler` - File system search
- `find-content-handler` - Content search within files
- `get-file-from-handler` - File download/retrieval
- `show-tools-handler` - Tool listing
- `open-app-handler` - Application launching
- `open-file-handler` - File opening
- `checkout-handler` - Git checkout operations
- `diff-handler` - File/Git diff operations
- `google-search-handler` - Web search
- `ask-ai-handler` - AI query processing

#### NLP Handlers (7 executables)
- `sentiment-handler` - Text sentiment analysis
- `summarize-handler` - Text summarization
- `classify-handler` - Text classification
- `extract-keywords-handler` - Keyword extraction
- `translate-handler` - Text translation
- `question-answer-handler` - Q&A processing
- `natural-language-handler` - General NLP

### 🔌 Input Methods

#### 1. Command Line Arguments
```bash
./target/release/sentiment-handler 'I love this product!'
./target/release/install-handler nodejs
```

**Note**: Use single quotes (`'`) instead of double quotes (`"`) to avoid zsh parsing errors.

#### 2. Standard Input
```bash
echo 'Text to analyze' | ./target/release/sentiment-handler
cat document.txt | ./target/release/summarize-handler
```

#### 3. Environment Variables
```bash
# Custom parsed results
PARSED_RESULT='{"sentiment": "positive", "score": 0.95}' \
  ./target/release/sentiment-handler 'This is great!'

# Debug logging
RUST_LOG=debug ./target/release/sentiment-handler 'test'
```

### 📊 Output Format

All handlers return JSON with this structure:
```json
{
  "success": true,
  "message": "Handler completed successfully",
  "data": {
    "action": "specific_action",
    "result": "handler_specific_data"
  },
  "execution_time_ms": 42
}
```

### 🏢 Architecture Benefits

- **Microservice Ready**: Each handler as independent service
- **Process Isolation**: Separate memory spaces and fault isolation
- **Selective Deployment**: Deploy only needed functionality
- **CLI Tools**: Use handlers in shell scripts and automation
- **Integration Friendly**: Easy to embed in existing systems

### 📈 Use Cases

- **Shell Scripts**: Automate text processing workflows
- **CI/CD Pipelines**: Integrate specific handlers in build processes
- **Microservices**: Deploy handlers as separate containerized services
- **Development Tools**: Create custom CLI utilities
- **System Administration**: Automate system tasks

### 📆 Complete Documentation

For comprehensive documentation on individual handlers:

- **[Documentation Hub](docs/index.md)** - Complete navigation and overview
- **[Installation Guide](docs/installation.md)** - Detailed setup instructions
- **[Usage Examples](docs/examples.md)** - Real-world usage scenarios
- **[API Reference](docs/api-reference.md)** - Complete technical specifications
- **[Troubleshooting](docs/troubleshooting.md)** - Common issues and solutions
- **[Development Guide](docs/development.md)** - How to extend and modify

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
