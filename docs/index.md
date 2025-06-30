# RustLM Server - Individual Callback Handlers Documentation

Welcome to the comprehensive documentation for the RustLM Server individual callback handler executables. This documentation covers everything you need to know about building, using, and extending the standalone callback handlers.

## 📚 Documentation Overview

This documentation is organized into several guides, each focusing on different aspects of the callback handler system:

### 🚀 Getting Started

- **[Main README](../README.md)** - Complete overview and quick start guide
- **[Installation Guide](installation.md)** - Detailed setup and build instructions
- **[Usage Examples](examples.md)** - Comprehensive examples and use cases

### 🔧 Technical Reference

- **[API Reference](api-reference.md)** - Detailed API documentation and specifications
- **[Troubleshooting](troubleshooting.md)** - Common issues and solutions
- **[Development Guide](development.md)** - How to extend and modify handlers

## 🎯 What Are Individual Callback Handlers?

The RustLM server originally had callback handlers embedded within the main application. This project extends that by creating **18 individual executable files** - one for each callback handler. This allows for:

- **Microservice Architecture**: Deploy handlers as separate services
- **Process Isolation**: Run handlers in isolated processes  
- **Selective Deployment**: Deploy only needed handlers
- **CI/CD Integration**: Use specific handlers in build pipelines
- **CLI Tools**: Create command-line utilities for specific tasks

## 📋 Available Handlers

### System Command Handlers (11 executables)
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

### NLP Handlers (7 executables)
- `sentiment-handler` - Text sentiment analysis
- `summarize-handler` - Text summarization
- `classify-handler` - Text classification
- `extract-keywords-handler` - Keyword extraction
- `translate-handler` - Text translation
- `question-answer-handler` - Q&A processing
- `natural-language-handler` - General NLP

## ⚡ Quick Start

```bash
# 1. Build all handlers
./build_handlers.sh

# 2. Test all handlers
./test_handlers.sh

# 3. Use individual handlers
./target/release/sentiment-handler 'I love this product!'
./target/release/install-handler nodejs
echo 'Long text...' | ./target/release/summarize-handler
```

## 📖 Documentation Structure

### For New Users

1. **Start with [Main README](../README.md)** for an overview
2. **Follow [Installation Guide](installation.md)** to set up
3. **Explore [Usage Examples](examples.md)** for practical examples
4. **Reference [Troubleshooting](troubleshooting.md)** if you encounter issues

### For Developers

1. **Read [Development Guide](development.md)** for contribution guidelines
2. **Check [API Reference](api-reference.md)** for technical specifications
3. **Use [Troubleshooting](troubleshooting.md)** for debugging

### For System Integrators

1. **Review [API Reference](api-reference.md)** for integration specifications
2. **Study [Usage Examples](examples.md)** for integration patterns
3. **Consult [Troubleshooting](troubleshooting.md)** for deployment issues

## 🔑 Key Features

✅ **Standalone Executables** - Each handler is completely independent  
✅ **Multiple Input Methods** - Command line, stdin, environment variables  
✅ **JSON Output** - Structured, parseable results  
✅ **Proper Exit Codes** - Standard success/failure indication  
✅ **Error Handling** - Robust error reporting and logging  
✅ **Shared Codebase** - All handlers use the same underlying logic  
✅ **Easy Integration** - Simple to use in scripts and other applications  

## 📊 Common Usage Patterns

### Command Line Usage
```bash
# Basic usage
./target/release/sentiment-handler 'I love this!'

# With environment variables
PARSED_RESULT='{"sentiment": "positive"}' ./target/release/sentiment-handler 'Great!'

# Pipeline usage
echo 'Text to analyze' | ./target/release/sentiment-handler
```

### Integration Examples
```bash
# Shell script integration
result=$(./target/release/sentiment-handler 'input text')
sentiment=$(echo "$result" | jq -r '.data.analysis.sentiment')

# Python integration
import subprocess
result = subprocess.run(['./target/release/sentiment-handler', 'input'], 
                       capture_output=True, text=True)
```

## 🛠️ Development and Contribution

The project is designed to be easily extensible. See the [Development Guide](development.md) for:

- Adding new handlers
- Modifying existing handlers  
- Testing procedures
- Code organization
- Best practices
- Contributing guidelines

## 🔍 Finding Information

### By Task

| Task | Documentation |
|------|---------------|
| **Setting up the project** | [Installation Guide](installation.md) |
| **Learning basic usage** | [Main README](../README.md) + [Usage Examples](examples.md) |
| **Integrating with other systems** | [API Reference](api-reference.md) + [Usage Examples](examples.md) |
| **Troubleshooting problems** | [Troubleshooting](troubleshooting.md) |
| **Extending functionality** | [Development Guide](development.md) |
| **Understanding the API** | [API Reference](api-reference.md) |

### By Role

| Role | Recommended Reading Order |
|------|---------------------------|
| **End User** | README → Installation → Examples → Troubleshooting |
| **Developer** | README → Installation → Development → API → Troubleshooting |
| **System Administrator** | README → Installation → API → Examples → Troubleshooting |
| **Integrator** | README → API → Examples → Development → Troubleshooting |

## 🆘 Getting Help

1. **Check [Troubleshooting](troubleshooting.md)** for common issues
2. **Review [Usage Examples](examples.md)** for similar use cases
3. **Consult [API Reference](api-reference.md)** for technical details
4. **Search the project repository** for existing issues
5. **Create a new issue** with debug information from [Troubleshooting](troubleshooting.md)

## 📋 Documentation Standards

This documentation follows these principles:

- **Comprehensive**: Covers all aspects of the system
- **Practical**: Includes working examples and real-world scenarios
- **Accessible**: Written for different skill levels and use cases
- **Maintainable**: Organized for easy updates and additions
- **Searchable**: Cross-referenced and well-indexed

## 🔄 Keeping Documentation Up to Date

The documentation is maintained alongside the code. When contributing:

1. **Update relevant documentation** when adding features
2. **Add examples** for new functionality
3. **Update API documentation** for interface changes
4. **Test all examples** to ensure they work
5. **Cross-reference** new content with existing docs

## 📝 Documentation Feedback

If you find errors, missing information, or have suggestions for improvement:

1. **Check if the issue exists** in the project repository
2. **Create a documentation issue** with specific details
3. **Suggest improvements** with concrete examples
4. **Contribute fixes** via pull requests

---

This documentation represents the collective effort to make the RustLM callback handlers accessible, usable, and extensible for everyone. Whether you're a newcomer or an experienced developer, you should find the information you need to successfully work with the system.

Happy coding! 🚀
