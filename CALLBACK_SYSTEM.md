# Callback System Implementation

## Overview
This Rust NLP server now includes a comprehensive callback system that automatically executes callbacks for every command processed. The callbacks are routed to appropriate handlers in a separate `callbacks.rs` file.

## Architecture

### Key Components

1. **CallbackHandler Enum** (`src/callbacks.rs`)
   - `System(SystemCommandHandler)` - Handles system commands like install, find_file, checkout, etc.
   - `Nlp(NlpCallbackHandler)` - Handles NLP tasks like sentiment, summarize, classify, etc.

2. **CallbackManager** (`src/callbacks.rs`)
   - Manages and routes callbacks to appropriate handlers
   - Executes callbacks automatically after each command processing
   - Tracks execution time and success/failure status

3. **Integration** (`src/nlp.rs`)
   - Modified `NlpProcessor` to include a `CallbackManager` instance
   - Added `execute_callbacks()` method that runs after every command
   - All commands now trigger their appropriate callbacks automatically

## Features

### Automatic Callback Execution
- Every command processed through the NLP server automatically triggers callbacks
- Callbacks are executed asynchronously after the main processing is complete
- Execution details are logged with timing information

### Command Support
**System Commands:**
- `install` - Package installation commands
- `find_file` - File search operations
- `find_content` - Content search operations
- `get_file_from` - File download/copy operations
- `show_tools` - Tool listing
- `open_app` - Application launching
- `open_file` - File opening
- `checkout` - Git checkout operations
- `diff` - Git diff operations
- `google_search` - Web search
- `ask_ai` - AI query handling

**NLP Commands:**
- `sentiment` - Sentiment analysis
- `summarize` - Text summarization
- `classify` - Text classification
- `extract_keywords` - Keyword extraction
- `translate` - Text translation
- `question_answer` - Question answering
- `natural_language` - Intent parsing

### Callback Results
Each callback returns a `CallbackResult` with:
- `success`: Boolean indicating if callback succeeded
- `message`: Human-readable status message
- `data`: Optional JSON data with callback-specific information
- `execution_time_ms`: Execution time in milliseconds

## Example Usage

### API Request
```bash
curl -X POST http://localhost:3000/process \
  -H "Content-Type: application/json" \
  -d '{"text": "rust", "task": "install"}'
```

### Server Response
```json
{
  "id": "02217c3a-600d-4f57-b448-62333da426ac",
  "input_text": "rust",
  "task": "install",
  "result": "{\"command\": \"install\", \"package\": \"rust\", \"suggested_commands\": [\"brew install rust\", \"npm install rust\", \"cargo install rust\", \"pip install rust\"]}",
  "confidence": 0.9,
  "processing_time_ms": 0
}
```

### Callback Execution (from logs)
```
INFO rustlm_server::callbacks: Executing callback system for command: install
INFO rustlm_server::callbacks: Processing install callback for: rust
INFO rustlm_server::callbacks: Install commands ready for execution: ["brew install rust", "npm install rust", "cargo install rust", "pip install rust"]
INFO rustlm_server::callbacks: Callback system completed successfully
INFO rustlm_server::nlp: Callback executed successfully in 0ms: Install callback processed for package: rust
```

## Natural Language Support

The system includes a new `natural_language` task that can parse intents from free-form text:

### Example
```bash
curl -X POST http://localhost:3000/process \
  -H "Content-Type: application/json" \
  -d '{"text": "I want to open file from SVN", "task": "natural_language"}'
```

### Response
```json
{
  "result": "{\"intent\": \"open_file_from_repo\", \"entities\": [(\"repository_type\", \"svn\")], \"confidence\": 0.80, \"next_action\": \"clarify_repository\", \"question\": \"Which repository would you like to open the file from? Please specify the repository name or path.\"}"
}
```

## Benefits

1. **Separation of Concerns**: Callbacks are handled in a separate module
2. **Extensibility**: Easy to add new callback handlers for different command types
3. **Logging**: Comprehensive logging of callback execution
4. **Error Handling**: Robust error handling with graceful failure
5. **Performance Tracking**: Execution time tracking for each callback
6. **Type Safety**: Strong typing with Rust's enum system

## Testing

The system includes comprehensive tests:
- Unit tests for individual callback handlers
- Integration tests for the callback manager
- End-to-end tests through the API

All tests pass successfully, ensuring the callback system works reliably.

## File Structure

```
src/
├── main.rs           # API server and routing
├── nlp.rs           # NLP processor with callback integration
└── callbacks.rs     # Callback system implementation
```

## Future Enhancements

1. **Custom Callbacks**: Allow registration of custom callback handlers
2. **Webhook Support**: Send callbacks to external HTTP endpoints
3. **Database Logging**: Store callback results in a database
4. **Conditional Callbacks**: Execute callbacks based on conditions
5. **Callback Chains**: Support for sequential callback execution
