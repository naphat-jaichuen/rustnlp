# Usage Examples

This guide provides comprehensive examples for using the individual callback handler executables in various scenarios.

## Table of Contents

- [Basic Usage](#basic-usage)
- [System Command Handlers](#system-command-handlers)
- [NLP Handlers](#nlp-handlers)
- [Advanced Usage](#advanced-usage)
- [Integration Examples](#integration-examples)
- [Real-World Scenarios](#real-world-scenarios)

## Basic Usage

### Command Line Arguments

```bash
# Basic syntax
./target/release/handler-name 'input text'

# Examples
./target/release/sentiment-handler 'I love this product!'
./target/release/install-handler nodejs
./target/release/find-file-handler package.json
```

### Standard Input

```bash
# Pipe text to handler
echo 'Text to process' | ./target/release/sentiment-handler

# Process files
cat document.txt | ./target/release/summarize-handler

# Chain commands
curl -s https://api.example.com/data | ./target/release/classify-handler
```

### Environment Variables

```bash
# Custom parsed results
PARSED_RESULT='{"sentiment": "positive", "score": 0.95}' \
  ./target/release/sentiment-handler 'This is amazing!'

# Multiple variables
PARSED_RESULT='{"custom": "data"}' \
RUST_LOG=debug \
  ./target/release/handler-name 'input'
```

## System Command Handlers

### Install Handler

**Purpose**: Handle software installation commands

```bash
# Basic package installation
./target/release/install-handler rust
./target/release/install-handler nodejs
./target/release/install-handler python3

# With custom package manager suggestions
PARSED_RESULT='{"package_manager": "brew", "suggested_commands": ["brew install nodejs", "npm install -g nodejs"]}' \
  ./target/release/install-handler nodejs

# Multiple packages
./target/release/install-handler 'docker kubernetes kubectl'
```

**Output Example**:
```json
{
  "success": true,
  "message": "Install callback processed for package: nodejs",
  "data": {
    "action": "install_ready",
    "package": "nodejs",
    "suggested_commands": [
      "brew install {{package}}",
      "apt-get install {{package}}",
      "cargo install {{package}}"
    ],
    "next_steps": "Commands are ready for execution"
  },
  "execution_time_ms": 5
}
```

### Find File Handler

**Purpose**: Search for files in the filesystem

```bash
# Find specific files
./target/release/find-file-handler package.json
./target/release/find-file-handler Cargo.toml
./target/release/find-file-handler '*.rs'

# With custom search paths
PARSED_RESULT='{"search_paths": ["/usr/local", "/opt"], "file_types": [".json", ".toml"]}' \
  ./target/release/find-file-handler config

# Find by pattern
./target/release/find-file-handler 'test_*.py'
```

**Output Example**:
```json
{
  "success": true,
  "message": "Find file callback processed for: package.json",
  "data": {
    "action": "file_search_ready",
    "filename": "package.json",
    "search_commands": ["echo 'Add specific commands for find file here'"],
    "status": "search_prepared"
  },
  "execution_time_ms": 3
}
```

### Find Content Handler

**Purpose**: Search for content within files

```bash
# Search for specific text
./target/release/find-content-handler 'TODO'
./target/release/find-content-handler 'function main'
./target/release/find-content-handler 'import'

# Case-sensitive search
./target/release/find-content-handler 'Error'

# Search with regex pattern
PARSED_RESULT='{"pattern": "fn\\s+\\w+", "regex": true}' \
  ./target/release/find-content-handler 'function patterns'
```

### Google Search Handler

**Purpose**: Perform web searches

```bash
# Simple search
./target/release/google-search-handler 'rust programming'
./target/release/google-search-handler 'docker tutorial'

# Technical searches
./target/release/google-search-handler 'kubernetes deployment yaml'
./target/release/google-search-handler 'react hooks useState'

# With custom search parameters
PARSED_RESULT='{"site": "stackoverflow.com", "time_range": "past_year"}' \
  ./target/release/google-search-handler 'rust async await'
```

### Open App Handler

**Purpose**: Launch applications

```bash
# Launch applications
./target/release/open-app-handler vscode
./target/release/open-app-handler firefox
./target/release/open-app-handler terminal

# With specific arguments
PARSED_RESULT='{"args": ["--new-window", "project.code-workspace"]}' \
  ./target/release/open-app-handler vscode
```

### Git Operations

```bash
# Checkout branches
./target/release/checkout-handler main
./target/release/checkout-handler feature-branch
./target/release/checkout-handler 'release/v1.0'

# View differences
./target/release/diff-handler HEAD~1
./target/release/diff-handler 'main..feature'

# With custom git options
PARSED_RESULT='{"options": ["--stat", "--name-only"]}' \
  ./target/release/diff-handler HEAD~3
```

## NLP Handlers

### Sentiment Handler

**Purpose**: Analyze text sentiment

```bash
# Positive sentiment
./target/release/sentiment-handler 'I absolutely love this product!'
./target/release/sentiment-handler 'This is fantastic and amazing!'

# Negative sentiment
./target/release/sentiment-handler 'This is terrible and disappointing'
./target/release/sentiment-handler 'I hate waiting in long lines'

# Neutral sentiment
./target/release/sentiment-handler 'The weather today is cloudy'
./target/release/sentiment-handler 'The meeting is scheduled for 3 PM'

# Batch sentiment analysis
echo 'Review 1: Great product, highly recommended!' | ./target/release/sentiment-handler
echo 'Review 2: Poor quality, would not buy again' | ./target/release/sentiment-handler

# With detailed analysis
PARSED_RESULT='{"sentiment": "positive", "score": 0.95, "emotions": ["joy", "satisfaction"], "keywords": ["love", "amazing"]}' \
  ./target/release/sentiment-handler 'This is absolutely incredible!'
```

**Output Example**:
```json
{
  "success": true,
  "message": "Sentiment analysis completed",
  "data": {
    "action": "sentiment_analyzed",
    "text": "I love this product!",
    "analysis": {
      "sentiment": "positive",
      "score": 0.95,
      "emotions": ["joy", "satisfaction"]
    },
    "confidence": 0.9
  },
  "execution_time_ms": 8
}
```

### Summarize Handler

**Purpose**: Summarize text content

```bash
# Summarize articles
cat article.txt | ./target/release/summarize-handler

# Summarize direct text
./target/release/summarize-handler 'This is a very long document with lots of details about various topics including technology, business processes, and user experience design. The document covers multiple aspects of software development...'

# Summarize with length constraints
PARSED_RESULT='{"max_length": 100, "style": "bullet_points"}' \
  ./target/release/summarize-handler 'Long technical documentation...'

# Summarize from URL (with preprocessing)
curl -s https://example.com/article | ./target/release/summarize-handler
```

### Classify Handler

**Purpose**: Classify text into categories

```bash
# Technical classification
./target/release/classify-handler 'This is a technical documentation about REST API development'
./target/release/classify-handler 'User manual for setting up the development environment'

# Content classification
./target/release/classify-handler 'Breaking news: Market reaches all-time high'
./target/release/classify-handler 'Recipe for chocolate chip cookies with detailed instructions'

# With predefined categories
PARSED_RESULT='{"categories": ["technical", "business", "personal", "news"], "confidence_threshold": 0.8}' \
  ./target/release/classify-handler 'Quarterly financial report shows strong growth'
```

### Extract Keywords Handler

**Purpose**: Extract keywords from text

```bash
# Extract from technical text
./target/release/extract-keywords-handler 'Rust programming language systems development memory safety concurrency'

# Extract from business text
./target/release/extract-keywords-handler 'Strategic business planning market analysis competitive advantage growth metrics'

# Extract with filtering
PARSED_RESULT='{"min_length": 4, "max_keywords": 10, "exclude_common": true}' \
  ./target/release/extract-keywords-handler 'Long text with many technical terms...'

# Extract from document
cat technical_paper.txt | ./target/release/extract-keywords-handler
```

### Translate Handler

**Purpose**: Translate text between languages

```bash
# Basic translation
./target/release/translate-handler 'Hello world'
./target/release/translate-handler 'Bonjour le monde'
./target/release/translate-handler 'Hola mundo'

# With target language
PARSED_RESULT='{"target_language": "spanish", "source_language": "english"}' \
  ./target/release/translate-handler 'Hello, how are you today?'

# Translate documents
cat document.txt | ./target/release/translate-handler
```

### Question Answer Handler

**Purpose**: Answer questions based on context

```bash
# Simple Q&A
./target/release/question-answer-handler 'What is the capital of France?'
./target/release/question-answer-handler 'How do you implement async functions in Rust?'

# With context
PARSED_RESULT='{"context": "Rust is a systems programming language focused on safety and performance", "domain": "programming"}' \
  ./target/release/question-answer-handler 'What are the main benefits of Rust?'
```

## Advanced Usage

### Chaining Handlers

```bash
# Sentiment analysis of summary
cat long_review.txt | \
  ./target/release/summarize-handler | \
  jq -r '.data.summary' | \
  ./target/release/sentiment-handler

# Classify then extract keywords
./target/release/classify-handler 'Technical documentation...' | \
  jq -r '.data.classification' | \
  ./target/release/extract-keywords-handler
```

### Conditional Execution

```bash
# Only proceed if sentiment is positive
if ./target/release/sentiment-handler 'User feedback' | jq -r '.data.analysis.sentiment' | grep -q 'positive'; then
  echo "Processing positive feedback..."
  # Additional processing
fi

# Error handling
if ! ./target/release/install-handler nodejs > /dev/null 2>&1; then
  echo "Installation handler failed"
  exit 1
fi
```

### JSON Processing

```bash
# Extract specific fields
./target/release/sentiment-handler 'Great product!' | jq '.data.analysis.score'

# Format output
./target/release/install-handler nodejs | jq '.data.suggested_commands[]'

# Combine results
{
  "sentiment": $(./target/release/sentiment-handler 'Input text' | jq '.data.analysis'),
  "keywords": $(./target/release/extract-keywords-handler 'Input text' | jq '.data.keywords')
} | jq '.'
```

## Integration Examples

### Shell Scripts

```bash
#!/bin/bash
# analyze_feedback.sh

input_file="$1"
if [[ ! -f "$input_file" ]]; then
  echo "Usage: $0 <feedback_file>"
  exit 1
fi

echo "Analyzing feedback from: $input_file"

# Process each line
while IFS= read -r line; do
  if [[ -n "$line" ]]; then
    echo "Processing: $line"
    
    # Analyze sentiment
    sentiment=$(./target/release/sentiment-handler "$line" | jq -r '.data.analysis.sentiment')
    score=$(./target/release/sentiment-handler "$line" | jq -r '.data.analysis.score // 0')
    
    echo "  Sentiment: $sentiment (Score: $score)"
    
    # Extract keywords if positive
    if [[ "$sentiment" == "positive" ]]; then
      keywords=$(./target/release/extract-keywords-handler "$line" | jq -r '.data.keywords[]?' | tr '\n' ', ')
      echo "  Keywords: $keywords"
    fi
    
    echo "---"
  fi
done < "$input_file"
```

### Python Integration

```python
#!/usr/bin/env python3
import subprocess
import json
import sys

def run_handler(handler, input_text, parsed_result=None):
    """Run a callback handler and return the result."""
    cmd = [f'./target/release/{handler}', input_text]
    
    env = {}
    if parsed_result:
        env['PARSED_RESULT'] = json.dumps(parsed_result)
    
    try:
        result = subprocess.run(
            cmd, 
            capture_output=True, 
            text=True, 
            env=env,
            check=True
        )
        return json.loads(result.stdout)
    except subprocess.CalledProcessError as e:
        print(f"Handler failed: {e.stderr}", file=sys.stderr)
        return None

# Usage examples
def analyze_text(text):
    """Comprehensive text analysis."""
    results = {}
    
    # Sentiment analysis
    sentiment_result = run_handler('sentiment-handler', text)
    if sentiment_result:
        results['sentiment'] = sentiment_result['data']['analysis']
    
    # Keyword extraction
    keywords_result = run_handler('extract-keywords-handler', text)
    if keywords_result:
        results['keywords'] = keywords_result['data']['keywords']
    
    # Classification
    classify_result = run_handler('classify-handler', text)
    if classify_result:
        results['classification'] = classify_result['data']['classification']
    
    return results

# Example usage
if __name__ == '__main__':
    text = 'This is an amazing product with great features!'
    analysis = analyze_text(text)
    print(json.dumps(analysis, indent=2))
```

### Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  sentiment-service:
    build: .
    command: ["sentiment-handler"]
    volumes:
      - ./data:/data
    environment:
      - RUST_LOG=info

  install-service:
    build: .
    command: ["install-handler"]
    volumes:
      - ./scripts:/scripts

  nlp-pipeline:
    build: .
    command: ["bash", "-c", "cat /data/input.txt | sentiment-handler | tee /data/sentiment.json"]
    volumes:
      - ./data:/data
    depends_on:
      - sentiment-service
```

### CI/CD Pipeline

```yaml
# .github/workflows/analysis.yml
name: Text Analysis Pipeline

on:
  pull_request:
    paths: ['docs/**', '*.md']

jobs:
  analyze-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build handlers
        run: ./build_handlers.sh
      
      - name: Analyze documentation
        run: |
          for file in docs/*.md; do
            echo "Analyzing: $file"
            
            # Check sentiment of documentation
            sentiment=$(cat "$file" | ./target/release/sentiment-handler | jq -r '.data.analysis.sentiment')
            
            # Extract keywords
            keywords=$(cat "$file" | ./target/release/extract-keywords-handler | jq -r '.data.keywords[]' | head -5)
            
            echo "Sentiment: $sentiment"
            echo "Top keywords: $keywords"
          done
```

## Real-World Scenarios

### Customer Feedback Analysis

```bash
#!/bin/bash
# Process customer feedback CSV

csv_file="feedback.csv"
output_dir="analysis_results"
mkdir -p "$output_dir"

# Skip header and process each feedback row
tail -n +2 "$csv_file" | while IFS=',' read -r id customer_name feedback_text rating; do
  echo "Processing feedback ID: $id"
  
  # Analyze sentiment
  sentiment_result=$(./target/release/sentiment-handler "$feedback_text")
  sentiment=$(echo "$sentiment_result" | jq -r '.data.analysis.sentiment')
  confidence=$(echo "$sentiment_result" | jq -r '.data.confidence')
  
  # Extract key themes
  keywords=$(./target/release/extract-keywords-handler "$feedback_text" | jq -r '.data.keywords[]' | head -3 | tr '\n' ', ')
  
  # Classify feedback type
  category=$(./target/release/classify-handler "$feedback_text" | jq -r '.data.classification.category // "general"')
  
  # Generate summary if negative
  if [[ "$sentiment" == "negative" ]]; then
    summary=$(./target/release/summarize-handler "$feedback_text" | jq -r '.data.summary')
    echo "$id,$customer_name,$sentiment,$confidence,$keywords,$category,$summary" >> "$output_dir/negative_feedback.csv"
  fi
  
  # Log all results
  echo "$id,$customer_name,$sentiment,$confidence,$keywords,$category" >> "$output_dir/all_feedback_analysis.csv"
done

echo "Analysis complete. Results in $output_dir/"
```

### Content Moderation Pipeline

```bash
#!/bin/bash
# Content moderation using multiple handlers

moderate_content() {
  local content="$1"
  local content_id="$2"
  
  # Classify content type
  classification=$(./target/release/classify-handler "$content" | jq -r '.data.classification.category')
  
  # Analyze sentiment
  sentiment=$(./target/release/sentiment-handler "$content" | jq -r '.data.analysis.sentiment')
  score=$(./target/release/sentiment-handler "$content" | jq -r '.data.analysis.score')
  
  # Extract keywords for flagging
  keywords=$(./target/release/extract-keywords-handler "$content" | jq -r '.data.keywords[]')
  
  # Decision logic
  if [[ "$sentiment" == "negative" ]] && (( $(echo "$score < 0.3" | bc -l) )); then
    echo "FLAGGED: $content_id - Highly negative content (score: $score)"
    echo "Keywords: $keywords"
    return 1
  elif [[ "$classification" == "spam" ]]; then
    echo "FLAGGED: $content_id - Classified as spam"
    return 1
  else
    echo "APPROVED: $content_id - Content passed moderation"
    return 0
  fi
}

# Process content queue
while IFS= read -r line; do
  content_id=$(echo "$line" | cut -d'|' -f1)
  content_text=$(echo "$line" | cut -d'|' -f2-)
  
  moderate_content "$content_text" "$content_id"
done < content_queue.txt
```

### Development Workflow Automation

```bash
#!/bin/bash
# Analyze commit messages and documentation

analyze_project() {
  echo "Analyzing project documentation and commit messages..."
  
  # Analyze README sentiment
  if [[ -f "README.md" ]]; then
    readme_sentiment=$(cat README.md | ./target/release/sentiment-handler | jq -r '.data.analysis.sentiment')
    echo "README sentiment: $readme_sentiment"
  fi
  
  # Extract keywords from recent commits
  git log --oneline -10 --pretty=format:"%s" | while read -r commit_msg; do
    keywords=$(./target/release/extract-keywords-handler "$commit_msg" | jq -r '.data.keywords[]?' | head -3)
    if [[ -n "$keywords" ]]; then
      echo "Commit keywords: $keywords"
    fi
  done
  
  # Analyze documentation files
  find docs/ -name "*.md" 2>/dev/null | while read -r doc_file; do
    echo "Analyzing: $doc_file"
    
    # Check if documentation needs updating
    summary=$(cat "$doc_file" | ./target/release/summarize-handler | jq -r '.data.summary')
    echo "Summary: $summary"
    
    # Extract main topics
    topics=$(cat "$doc_file" | ./target/release/extract-keywords-handler | jq -r '.data.keywords[]' | head -5)
    echo "Main topics: $topics"
    echo "---"
  done
}

analyze_project
```

These examples demonstrate the flexibility and power of the individual callback handler executables in various real-world scenarios, from simple text analysis to complex automation pipelines.
