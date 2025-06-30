# API Reference

This document provides detailed API reference for the individual callback handler executables.

## Table of Contents

- [Command Line Interface](#command-line-interface)
- [Input/Output Specification](#inputoutput-specification)
- [Data Structures](#data-structures)
- [Handler-Specific APIs](#handler-specific-apis)
- [Environment Variables](#environment-variables)
- [Exit Codes](#exit-codes)
- [Error Handling](#error-handling)

## Command Line Interface

### General Syntax

```bash
./target/release/handler-name [OPTIONS] [INPUT_TEXT]
```

### Arguments

- `INPUT_TEXT` (optional): Text to process. If not provided, input is read from stdin.

### Options

Currently, handlers don't accept command-line options but this may be extended in future versions.

### Input Methods

#### 1. Command Line Arguments
```bash
./target/release/sentiment-handler 'I love this product!'
```

#### 2. Standard Input
```bash
echo 'Text to analyze' | ./target/release/sentiment-handler
cat file.txt | ./target/release/summarize-handler
```

#### 3. Environment Variables
```bash
PARSED_RESULT='{"custom": "data"}' ./target/release/handler-name 'input'
```

## Input/Output Specification

### Input

Handlers accept text input through:
1. Command line arguments (joined with spaces)
2. Standard input (read until EOF)
3. Environment variables for configuration

### Output

All handlers output JSON to stdout with the following structure:

```json
{
  "success": boolean,
  "message": string,
  "data": object | null,
  "execution_time_ms": number
}
```

## Data Structures

### CommandContext

The internal context structure passed to handlers:

```rust
pub struct CommandContext {
    pub command: String,           // Handler command name
    pub task: String,             // Task description (same as command)
    pub input_text: String,       // Input text to process
    pub parsed_result: String,    // JSON string with parsed data
    pub confidence: Option<f32>,  // Confidence score (0.0-1.0)
    pub timestamp: DateTime<Utc>, // Execution timestamp
    pub session_id: Option<String>, // Optional session identifier
}
```

### CallbackResult

The result structure returned by handlers:

```rust
pub struct CallbackResult {
    pub success: bool,                    // Operation success status
    pub message: String,                  // Human-readable message
    pub data: Option<serde_json::Value>, // Handler-specific data
    pub execution_time_ms: u64,          // Execution time in milliseconds
}
```

## Handler-Specific APIs

### System Command Handlers

#### Install Handler

**Command**: `install-handler`

**Purpose**: Process software installation requests

**Input**: Package name or installation request

**Output Data Structure**:
```json
{
  "action": "install_ready",
  "package": "string",
  "suggested_commands": ["string"],
  "next_steps": "string"
}
```

**Example**:
```bash
./target/release/install-handler nodejs
```

**Custom Parsed Result**:
```json
{
  "package_manager": "brew",
  "suggested_commands": ["brew install nodejs", "npm install -g nodejs"],
  "platform": "macos"
}
```

#### Find File Handler

**Command**: `find-file-handler`

**Purpose**: Search for files in the filesystem

**Input**: Filename or file pattern

**Output Data Structure**:
```json
{
  "action": "file_search_ready",
  "filename": "string",
  "search_commands": ["string"],
  "status": "search_prepared"
}
```

**Custom Parsed Result**:
```json
{
  "search_paths": ["/usr/local", "/opt"],
  "file_types": [".json", ".toml"],
  "case_sensitive": true,
  "max_depth": 5
}
```

#### Find Content Handler

**Command**: `find-content-handler`

**Purpose**: Search for content within files

**Input**: Search term or pattern

**Output Data Structure**:
```json
{
  "action": "content_search_ready",
  "search_term": "string",
  "search_commands": ["string"],
  "status": "search_prepared"
}
```

**Custom Parsed Result**:
```json
{
  "pattern": "regex_pattern",
  "regex": true,
  "case_sensitive": false,
  "file_extensions": [".rs", ".py", ".js"]
}
```

#### Open App Handler

**Command**: `open-app-handler`

**Purpose**: Launch applications

**Input**: Application name

**Output Data Structure**:
```json
{
  "action": "app_launch_ready",
  "app_name": "string",
  "launch_commands": ["string"],
  "status": "app_launch_prepared"
}
```

#### Google Search Handler

**Command**: `google-search-handler`

**Purpose**: Perform web searches

**Input**: Search query

**Output Data Structure**:
```json
{
  "action": "search_ready",
  "query": "string",
  "search_url": "string",
  "search_commands": ["string"],
  "status": "search_prepared"
}
```

#### Git Handlers

##### Checkout Handler

**Command**: `checkout-handler`

**Input**: Branch name or commit hash

**Output Data Structure**:
```json
{
  "action": "checkout_ready",
  "target": "string",
  "checkout_commands": ["string"],
  "status": "checkout_prepared"
}
```

##### Diff Handler

**Command**: `diff-handler`

**Input**: Diff target (e.g., "HEAD~1", "main..feature")

**Output Data Structure**:
```json
{
  "action": "diff_ready",
  "target": "string",
  "diff_commands": ["string"],
  "status": "diff_prepared"
}
```

### NLP Handlers

#### Sentiment Handler

**Command**: `sentiment-handler`

**Purpose**: Analyze text sentiment

**Input**: Text to analyze

**Output Data Structure**:
```json
{
  "action": "sentiment_analyzed",
  "text": "string",
  "analysis": {
    "sentiment": "positive|negative|neutral",
    "score": 0.95,
    "emotions": ["joy", "satisfaction"],
    "keywords": ["love", "amazing"]
  },
  "confidence": 0.9
}
```

**Custom Parsed Result**:
```json
{
  "sentiment": "positive",
  "score": 0.95,
  "emotions": ["joy", "satisfaction"],
  "confidence": "high",
  "keywords": ["love", "amazing"],
  "detailed_analysis": {
    "positive_score": 0.95,
    "negative_score": 0.05,
    "neutral_score": 0.0
  }
}
```

#### Summarize Handler

**Command**: `summarize-handler`

**Purpose**: Summarize text content

**Input**: Text to summarize

**Output Data Structure**:
```json
{
  "action": "text_summarized",
  "original_text": "string",
  "summary": "string",
  "confidence": 0.9
}
```

**Custom Parsed Result**:
```json
{
  "max_length": 100,
  "style": "bullet_points",
  "key_points": ["point1", "point2"],
  "summary_ratio": 0.3
}
```

#### Classify Handler

**Command**: `classify-handler`

**Purpose**: Classify text into categories

**Input**: Text to classify

**Output Data Structure**:
```json
{
  "action": "text_classified",
  "text": "string",
  "classification": {
    "category": "technical",
    "subcategory": "documentation",
    "confidence": 0.85,
    "categories": {
      "technical": 0.85,
      "business": 0.10,
      "personal": 0.05
    }
  },
  "confidence": 0.85
}
```

#### Extract Keywords Handler

**Command**: `extract-keywords-handler`

**Purpose**: Extract keywords from text

**Input**: Text to analyze

**Output Data Structure**:
```json
{
  "action": "keywords_extracted",
  "text": "string",
  "keywords": ["keyword1", "keyword2", "keyword3"],
  "confidence": 0.9
}
```

**Custom Parsed Result**:
```json
{
  "min_length": 4,
  "max_keywords": 10,
  "exclude_common": true,
  "keyword_scores": {
    "rust": 0.95,
    "programming": 0.88,
    "language": 0.75
  }
}
```

#### Translate Handler

**Command**: `translate-handler`

**Purpose**: Translate text between languages

**Input**: Text to translate

**Output Data Structure**:
```json
{
  "action": "text_translated",
  "original_text": "string",
  "translation": {
    "translated_text": "string",
    "source_language": "english",
    "target_language": "spanish",
    "confidence": 0.92
  },
  "confidence": 0.92
}
```

#### Question Answer Handler

**Command**: `question-answer-handler`

**Purpose**: Answer questions based on context

**Input**: Question text

**Output Data Structure**:
```json
{
  "action": "question_answered",
  "question": "string",
  "answer": {
    "answer_text": "string",
    "confidence": 0.88,
    "sources": ["source1", "source2"],
    "context_used": "string"
  },
  "confidence": 0.88
}
```

#### Natural Language Handler

**Command**: `natural-language-handler`

**Purpose**: General natural language processing

**Input**: Text to process

**Output Data Structure**:
```json
{
  "action": "natural_language_processed",
  "input": "string",
  "parsed_intent": "string",
  "confidence": 0.9,
  "next_steps": "string"
}
```

## Environment Variables

### PARSED_RESULT

**Type**: JSON string  
**Purpose**: Provide custom parsed data to handlers  
**Format**: Valid JSON string

**Example**:
```bash
PARSED_RESULT='{"sentiment": "positive", "score": 0.95}' \
  ./target/release/sentiment-handler 'text'
```

**Handler-specific formats**:

#### System Handlers
```json
{
  "package_manager": "brew",
  "suggested_commands": ["command1", "command2"],
  "platform": "macos",
  "options": ["--flag1", "--flag2"]
}
```

#### NLP Handlers
```json
{
  "max_length": 100,
  "style": "bullet_points",
  "target_language": "spanish",
  "categories": ["cat1", "cat2"],
  "confidence_threshold": 0.8
}
```

### RUST_LOG

**Type**: String  
**Purpose**: Control logging verbosity  
**Values**: `error`, `warn`, `info`, `debug`, `trace`

**Example**:
```bash
RUST_LOG=debug ./target/release/sentiment-handler 'text'
```

## Exit Codes

| Code | Meaning | Description |
|------|---------|-------------|
| 0 | Success | Handler completed successfully |
| 1 | Error | Handler failed due to error |

## Error Handling

### Error Response Format

When a handler fails, it outputs JSON with `success: false`:

```json
{
  "success": false,
  "message": "Error description",
  "data": null,
  "execution_time_ms": 0
}
```

### Common Error Types

#### Input Errors

**Invalid Input**:
```json
{
  "success": false,
  "message": "Invalid input: empty text provided",
  "data": null,
  "execution_time_ms": 0
}
```

#### Parsing Errors

**JSON Parse Error**:
```json
{
  "success": false,
  "message": "Failed to parse PARSED_RESULT: invalid JSON",
  "data": null,
  "execution_time_ms": 0
}
```

#### Handler Errors

**Handler-specific Error**:
```json
{
  "success": false,
  "message": "Sentiment analysis failed: insufficient text length",
  "data": null,
  "execution_time_ms": 5
}
```

### Error Recovery

1. **Check Exit Code**: Always check the exit code after running a handler
2. **Parse JSON Response**: Even on failure, handlers return valid JSON
3. **Log Error Messages**: Use the `message` field for debugging
4. **Retry Logic**: Implement appropriate retry mechanisms for transient errors

### Debugging

#### Enable Debug Logging
```bash
RUST_LOG=debug ./target/release/handler-name 'input' 2>&1 | tee debug.log
```

#### Validate JSON Output
```bash
./target/release/handler-name 'input' | jq '.' || echo "Invalid JSON output"
```

#### Check Handler Status
```bash
if ./target/release/handler-name 'input' | jq -e '.success' > /dev/null; then
  echo "Handler succeeded"
else
  echo "Handler failed"
fi
```

## Best Practices

### Input Validation

- Always use single quotes for command line arguments to avoid shell parsing issues
- Validate JSON in `PARSED_RESULT` before setting the environment variable
- Handle empty or whitespace-only input appropriately

### Error Handling

- Check exit codes in scripts
- Parse and handle JSON error responses
- Implement appropriate retry logic for transient failures

### Performance

- Use appropriate handlers for the task (don't use heavy NLP handlers for simple tasks)
- Consider caching results for repeated operations
- Monitor execution times through the `execution_time_ms` field

### Security

- Sanitize input when using handlers in web applications
- Be cautious with user-provided `PARSED_RESULT` data
- Validate and limit input size for public-facing services

## Integration Patterns

### Synchronous Processing

```bash
result=$(./target/release/sentiment-handler 'input text')
success=$(echo "$result" | jq -r '.success')
if [[ "$success" == "true" ]]; then
  # Process successful result
  sentiment=$(echo "$result" | jq -r '.data.analysis.sentiment')
fi
```

### Pipeline Processing

```bash
echo 'input text' | \
  ./target/release/summarize-handler | \
  jq -r '.data.summary' | \
  ./target/release/sentiment-handler
```

### Error-Safe Processing

```bash
process_text() {
  local input="$1"
  local result
  
  if ! result=$(./target/release/sentiment-handler "$input" 2>&1); then
    echo "Handler execution failed"
    return 1
  fi
  
  if ! echo "$result" | jq -e '.success' > /dev/null; then
    echo "Handler reported failure: $(echo "$result" | jq -r '.message')"
    return 1
  fi
  
  echo "$result"
}
```
