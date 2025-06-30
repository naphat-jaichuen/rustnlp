# RustLM Task Usage Guide

This guide explains when and how to use each task available in the RustLM server. Understanding these tasks will help you choose the right processing method for your specific use case.

## 📋 Quick Reference

| Category | Task | Purpose | Use Case |
|----------|------|---------|----------|
| **NLP** | `sentiment` | Emotional tone analysis | Customer feedback, social media monitoring |
| **NLP** | `summarize` | Text summarization | Article summaries, content overviews |
| **NLP** | `classify` | Text categorization | Content tagging, topic organization |
| **NLP** | `extract_keywords` | Key term extraction | SEO, concept highlighting |
| **NLP** | `translate` | Language translation | Multilingual content (placeholder) |
| **NLP** | `question_answer` | Q&A processing | Knowledge retrieval (placeholder) |
| **System** | `install` | Package installation | Environment setup, dependency management |
| **System** | `find_file` | File search | Filesystem navigation, file location |
| **System** | `find_content` | Content search | Code exploration, text search |
| **System** | `get_file_from` | File retrieval | Download automation, file copying |
| **System** | `show_tools` | Tool discovery | Development environment exploration |
| **System** | `open_app` | Application launcher | macOS app management |
| **System** | `open_file` | File opener | Quick file access, editing workflows |
| **Git** | `checkout` | Branch switching | Version control, development workflows |
| **Git** | `diff` | File comparison | Code review, change analysis |
| **Web** | `google_search` | Search automation | Information discovery, research |
| **AI** | `ask_ai` | AI-powered Q&A | Advanced question answering via Azure OpenAI |
| **General** | `natural_language` | NL command processing | Flexible command interpretation |

---

## 🧠 NLP Tasks

### 1. Sentiment Analysis (`sentiment`)

**Purpose**: Analyze the emotional tone of text and classify it as positive, negative, or neutral.

**When to Use**:
- ✅ Customer feedback analysis
- ✅ Social media sentiment monitoring
- ✅ Product review analysis
- ✅ Email tone assessment
- ✅ Content mood evaluation

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/sentiment \
  -H "Content-Type: application/json" \
  -d '{"text": "I absolutely love this new feature! It works perfectly."}'
```

**Expected Response**:
```json
{
  "task": "sentiment",
  "result": "{\"sentiment\": \"positive\", \"positive_score\": 2, \"negative_score\": 0}",
  "confidence": 0.9
}
```

**Best Practices**:
- Use for texts with clear emotional content
- Works best with customer reviews, social media posts
- Consider context when interpreting results

---

### 2. Text Summarization (`summarize`)

**Purpose**: Create concise summaries of longer text content.

**When to Use**:
- ✅ Article summarization
- ✅ Report executive summaries
- ✅ Meeting notes condensation
- ✅ Email thread summaries
- ✅ Research paper abstracts

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/summarize \
  -H "Content-Type: application/json" \
  -d '{"text": "Artificial intelligence has revolutionized many industries. Machine learning algorithms can now process vast amounts of data. Deep learning models have achieved human-level performance in many tasks. The future of AI looks promising with continued advancements."}'
```

**Best Practices**:
- Works best with structured text (multiple sentences)
- Ideal for content longer than 50 words
- Use for creating TL;DR versions of content

---

### 3. Text Classification (`classify`)

**Purpose**: Categorize text into predefined categories such as technology, business, sports, health, or entertainment.

**When to Use**:
- ✅ Content organization and tagging
- ✅ News article categorization
- ✅ Email filtering and routing
- ✅ Support ticket classification
- ✅ Research paper categorization

**Available Categories**:
- `technology` - Computer, software, AI, programming
- `business` - Finance, market, investment, profit
- `sports` - Games, teams, competitions, athletes
- `health` - Medicine, treatment, hospitals, wellness
- `entertainment` - Movies, music, shows, celebrities

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/classify \
  -H "Content-Type: application/json" \
  -d '{"text": "The new machine learning algorithm shows 95% accuracy in predicting market trends using deep neural networks."}'
```

**Best Practices**:
- Ensure text contains relevant keywords for accurate classification
- Works best with domain-specific content
- Use for automated content sorting

---

### 4. Keyword Extraction (`extract_keywords`)

**Purpose**: Identify and extract the most important keywords and phrases from text.

**When to Use**:
- ✅ SEO optimization
- ✅ Content indexing
- ✅ Topic identification
- ✅ Research paper key terms
- ✅ Document summarization support

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/extract_keywords \
  -H "Content-Type: application/json" \
  -d '{"text": "Cloud computing platforms like AWS and Azure provide scalable infrastructure for modern applications using containerization technologies."}'
```

**Best Practices**:
- Use with content-rich text (>20 words)
- Effective for technical documentation
- Helpful for creating tag clouds and metadata

---

### 5. Translation (`translate`) *[Placeholder]*

**Purpose**: Language translation functionality (currently a placeholder).

**When to Use**:
- 🚧 **Note**: This is currently a mock implementation
- Future use: Multilingual content creation
- Future use: International communication
- Future use: Content localization

**Recommended Action**: Replace with actual translation service (Google Translate API, Azure Translator, etc.)

---

### 6. Question Answering (`question_answer`) *[Placeholder]*

**Purpose**: Answer questions based on provided context (currently a placeholder).

**When to Use**:
- 🚧 **Note**: This is currently a mock implementation
- Future use: Knowledge base queries
- Future use: Document-based Q&A
- Future use: Educational content assistance

**Recommended Action**: Integrate with actual Q&A models or services

---

## 🛠️ System Command Tasks

### 7. Package Installation (`install`)

**Purpose**: Generate installation commands for various package managers and tools.

**When to Use**:
- ✅ Environment setup automation
- ✅ Dependency management
- ✅ Development workflow streamlining
- ✅ CI/CD pipeline configuration
- ✅ Documentation generation

**Supported Package Managers**:
- `brew` (Homebrew for macOS)
- `npm` (Node.js packages)
- `cargo` (Rust packages)
- `pip` (Python packages)

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/install \
  -H "Content-Type: application/json" \
  -d '{"text": "nodejs"}'
```

**Best Practices**:
- Use exact package names for best results
- Helpful for creating setup scripts
- Great for documentation automation

---

### 8. Find Files (`find_file`)

**Purpose**: Generate commands to locate files by name in the filesystem.

**When to Use**:
- ✅ Filesystem navigation
- ✅ Build script automation
- ✅ File management workflows
- ✅ Development environment setup
- ✅ Missing file location

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/find_file \
  -H "Content-Type: application/json" \
  -d '{"text": "config.json"}'
```

**Generated Commands**:
- `find` command variations
- `locate` command usage
- Platform-specific search tools

---

### 9. Find Content (`find_content`)

**Purpose**: Generate commands to search for specific content within files.

**When to Use**:
- ✅ Code exploration and debugging
- ✅ Configuration file analysis
- ✅ Log file investigation
- ✅ Research and documentation
- ✅ Security auditing (finding sensitive data)

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/find_content \
  -H "Content-Type: application/json" \
  -d '{"text": "TODO"}'
```

**Generated Commands**:
- `grep` variations for text search
- `ripgrep` (rg) for fast searches
- `ag` (Silver Searcher) commands

---

### 10. Get File From Source (`get_file_from`)

**Purpose**: Generate commands to download or copy files from various sources.

**When to Use**:
- ✅ Build automation
- ✅ Asset downloading
- ✅ Configuration file retrieval
- ✅ CI/CD pipeline file management
- ✅ Development environment setup

**Supported Sources**:
- HTTP/HTTPS URLs
- FTP servers
- Local file paths
- Cloud storage services

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/get_file_from \
  -H "Content-Type: application/json" \
  -d '{"text": "https://example.com/config.yaml"}'
```

---

### 11. Show Tools (`show_tools`)

**Purpose**: Display categorized lists of development tools and utilities.

**Available Categories**:
- `development` - IDEs, editors, compilers
- `system` - System utilities and monitoring tools
- `file_management` - File operation tools
- `network` - Network analysis and testing tools
- `text_editors` - Text and code editors

**When to Use**:
- ✅ Discovering new development tools
- ✅ Environment setup planning
- ✅ Tool recommendation systems
- ✅ Educational purposes
- ✅ Workflow optimization

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/show_tools \
  -H "Content-Type: application/json" \
  -d '{"text": "development"}'
```

---

### 12. Open Applications (`open_app`)

**Purpose**: Generate commands to open macOS applications.

**When to Use**:
- ✅ Workflow automation on macOS
- ✅ Development environment setup
- ✅ Script-based app launching
- ✅ Productivity automation
- ✅ System administration

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/open_app \
  -H "Content-Type: application/json" \
  -d '{"text": "Visual Studio Code"}'
```

**Generated Commands**:
- `open -a` commands for macOS
- Application bundle path handling
- Alternative launching methods

---

### 13. Open Files (`open_file`)

**Purpose**: Generate commands to open files with appropriate applications.

**When to Use**:
- ✅ Quick file access in workflows
- ✅ Development environment integration
- ✅ File management automation
- ✅ Cross-platform file handling
- ✅ Script-based file operations

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/open_file \
  -H "Content-Type: application/json" \
  -d '{"text": "/path/to/document.pdf"}'
```

---

## 🔀 Git Version Control Tasks

### 14. Git Checkout (`checkout`)

**Purpose**: Generate Git checkout commands for branches, commits, and tags.

**When to Use**:
- ✅ Branch switching automation
- ✅ Version control workflows
- ✅ CI/CD pipeline branch management
- ✅ Development environment setup
- ✅ Release management

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/checkout \
  -H "Content-Type: application/json" \
  -d '{"text": "feature/new-ui"}'
```

**Generated Commands**:
- Branch checkout: `git checkout branch-name`
- Commit checkout: `git checkout commit-hash`
- Tag checkout: `git checkout tag-name`
- New branch creation: `git checkout -b new-branch`

---

### 15. Git Diff (`diff`)

**Purpose**: Generate Git diff commands for comparing files, commits, and branches.

**When to Use**:
- ✅ Code review automation
- ✅ Change analysis
- ✅ Release note generation
- ✅ Debugging and troubleshooting
- ✅ Merge conflict resolution

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/diff \
  -H "Content-Type: application/json" \
  -d '{"text": "HEAD~1 HEAD"}'
```

**Generated Commands**:
- File diff: `git diff file-path`
- Commit diff: `git diff commit1 commit2`
- Branch diff: `git diff branch1..branch2`
- Staged changes: `git diff --cached`

---

## 🌐 Web and Search Tasks

### 16. Google Search (`google_search`)

**Purpose**: Generate Google search URLs and commands to open them.

**When to Use**:
- ✅ Research automation
- ✅ Information discovery workflows
- ✅ Documentation assistance
- ✅ Troubleshooting and debugging
- ✅ Learning and education

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/google_search \
  -H "Content-Type: application/json" \
  -d '{"text": "rust programming best practices"}'
```

**Generated Output**:
- Google search URL
- Platform-specific open commands
- Alternative search methods
- Browser automation scripts

---

## 🤖 AI-Powered Tasks

### 17. Ask AI (`ask_ai`)

**Purpose**: Send questions to Azure OpenAI for AI-powered responses.

**Prerequisites**:
Set these environment variables:
```bash
export AZURE_OPENAI_ENDPOINT="https://your-resource.openai.azure.com"
export AZURE_OPENAI_API_KEY="your-api-key"
export AZURE_OPENAI_DEPLOYMENT="gpt-35-turbo"
```

**When to Use**:
- ✅ Complex question answering
- ✅ Code explanation and generation
- ✅ Creative writing assistance
- ✅ Problem-solving guidance
- ✅ Educational support

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/ask_ai \
  -H "Content-Type: application/json" \
  -d '{"text": "Explain the benefits of using Rust for system programming"}'
```

**Response Types**:
- **With Azure Setup**: AI-generated answer
- **Without Setup**: Configuration instructions and curl examples

---

## 🎯 General Purpose Tasks

### 18. Natural Language (`natural_language`)

**Purpose**: Handle general natural language commands and provide intelligent responses.

**When to Use**:
- ✅ Flexible command interpretation
- ✅ Conversational interfaces
- ✅ Multi-intent processing
- ✅ Fallback for unclear requests
- ✅ General assistance workflows

**Example Usage**:
```bash
curl -X POST http://localhost:3000/process/natural_language \
  -H "Content-Type: application/json" \
  -d '{"text": "Help me find all the configuration files in my project"}'
```

---

## 📊 Choosing the Right Task

### Decision Flow

1. **Text Analysis Needed?**
   - Emotion/sentiment → `sentiment`
   - Categorization → `classify`
   - Summary → `summarize`
   - Key terms → `extract_keywords`

2. **System Operations?**
   - Software installation → `install`
   - File operations → `find_file`, `find_content`, `get_file_from`, `open_file`
   - App management → `open_app`
   - Tool discovery → `show_tools`

3. **Version Control?**
   - Branch operations → `checkout`
   - Change analysis → `diff`

4. **Research/Information?**
   - Web search → `google_search`
   - AI assistance → `ask_ai`

5. **Unclear Intent?**
   - General processing → `natural_language`

### Performance Considerations

- **Fast Tasks** (< 50ms): `sentiment`, `classify`, `extract_keywords`
- **Medium Tasks** (50-200ms): `summarize`, system commands
- **Slow Tasks** (> 200ms): `ask_ai` (depends on Azure OpenAI response time)

### Accuracy Notes

- **High Accuracy**: System commands, file operations, Git commands
- **Medium Accuracy**: NLP tasks (rule-based implementation)
- **Variable Accuracy**: `ask_ai` (depends on Azure OpenAI model)

---

## 🔧 Usage Tips

### API Call Examples

**Using curl**:
```bash
# General endpoint
curl -X POST http://localhost:3000/process \
  -H "Content-Type: application/json" \
  -d '{"text": "your text here", "task": "task_name"}'

# Task-specific endpoint
curl -X POST http://localhost:3000/process/task_name \
  -H "Content-Type: application/json" \
  -d '{"text": "your text here"}'
```

**Response Format**:
```json
{
  "id": "request-uuid",
  "input_text": "your text here",
  "task": "task_name",
  "result": "processed result",
  "confidence": 0.85,
  "processing_time_ms": 120
}
```

### Error Handling

Common error scenarios:
- Empty text input
- Unsupported task name
- Invalid JSON format
- Azure OpenAI configuration missing (for `ask_ai`)

### Best Practices

1. **Choose the most specific task** for your use case
2. **Provide clear, relevant input text** for better results
3. **Use task-specific endpoints** for cleaner code
4. **Handle confidence scores** appropriately in your application
5. **Implement proper error handling** for failed requests
6. **Consider response time** when choosing tasks for real-time applications

---

## 📈 Future Enhancements

Tasks marked as placeholders can be enhanced with:

- **Translation**: Google Translate API, Azure Translator
- **Question Answering**: Transformer models, knowledge bases
- **Improved NLP**: Machine learning models, neural networks
- **Additional System Commands**: More platform support, cloud operations

For production use, consider replacing rule-based implementations with proper ML models for better accuracy and performance.
