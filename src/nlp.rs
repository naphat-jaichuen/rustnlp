use anyhow::{anyhow, Result};
use std::collections::HashMap;
use tracing::info;

/// NLP Processor that handles various text processing tasks
pub struct NlpProcessor {
    available_tasks: Vec<String>,
}

impl NlpProcessor {
    /// Initialize the NLP processor
    pub async fn new() -> Result<Self> {
        info!("Initializing NLP processor...");
        
        // For now, we'll implement basic NLP tasks without heavy model dependencies
        // In a production environment, you would load actual ML models here
        let available_tasks = vec![
            "sentiment".to_string(),
            "summarize".to_string(),
            "classify".to_string(),
            "extract_keywords".to_string(),
            "translate".to_string(),
            "question_answer".to_string(),
            // System command tasks
            "install".to_string(),
            "find_file".to_string(),
            "find_content".to_string(),
            "get_file_from".to_string(),
            "show_tools".to_string(),
            "open_app".to_string(),
            "open_file".to_string(),
            "checkout".to_string(),
            "diff".to_string(),
            "google_search".to_string(),
        ];

        info!("Available NLP tasks: {:?}", available_tasks);

        Ok(Self { available_tasks })
    }

    /// Process text with the specified task
    pub async fn process(&self, text: &str, task: &str) -> Result<(String, Option<f32>)> {
        if text.trim().is_empty() {
            return Err(anyhow!("Input text cannot be empty"));
        }

        match task.to_lowercase().as_str() {
            "sentiment" => self.analyze_sentiment(text).await,
            "summarize" => self.summarize_text(text).await,
            "classify" => self.classify_text(text).await,
            "extract_keywords" => self.extract_keywords(text).await,
            "translate" => self.translate_text(text).await,
            "question_answer" => self.answer_question(text).await,
            // System command tasks
            "install" => self.handle_install(text).await,
            "find_file" => self.handle_find_file(text).await,
            "find_content" => self.handle_find_content(text).await,
            "get_file_from" => self.handle_get_file_from(text).await,
            "show_tools" => self.handle_show_tools(text).await,
            "open_app" => self.handle_open_app(text).await,
            "open_file" => self.handle_open_file(text).await,
            "checkout" => self.handle_checkout(text).await,
            "diff" => self.handle_diff(text).await,
            "google_search" => self.handle_google_search(text).await,
            _ => Err(anyhow!("Unsupported task: {}", task)),
        }
    }

    /// List available processing tasks
    pub fn list_available_tasks(&self) -> Vec<String> {
        self.available_tasks.clone()
    }

    /// Analyze sentiment of the text
    async fn analyze_sentiment(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Analyzing sentiment for text of length: {}", text.len());

        // Simple rule-based sentiment analysis for demonstration
        // In production, you would use a proper ML model
        let positive_words = ["good", "great", "excellent", "amazing", "wonderful", "fantastic", "love", "happy", "awesome"];
        let negative_words = ["bad", "terrible", "awful", "hate", "sad", "angry", "disappointed", "horrible"];

        let text_lower = text.to_lowercase();
        let mut positive_count = 0;
        let mut negative_count = 0;

        for word in positive_words.iter() {
            positive_count += text_lower.matches(word).count();
        }

        for word in negative_words.iter() {
            negative_count += text_lower.matches(word).count();
        }

        let (sentiment, confidence) = if positive_count > negative_count {
            ("positive", 0.7 + (positive_count as f32 * 0.1).min(0.3))
        } else if negative_count > positive_count {
            ("negative", 0.7 + (negative_count as f32 * 0.1).min(0.3))
        } else {
            ("neutral", 0.5)
        };

        let result = format!(
            "{{\"sentiment\": \"{}\", \"positive_score\": {}, \"negative_score\": {}}}",
            sentiment, positive_count, negative_count
        );

        Ok((result, Some(confidence)))
    }

    /// Summarize the text
    async fn summarize_text(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Summarizing text of length: {}", text.len());

        // Simple extractive summarization - take first and last sentences
        let sentences: Vec<&str> = text
            .split(&['.', '!', '?'])
            .filter(|s| !s.trim().is_empty())
            .collect();

        let summary = if sentences.len() <= 2 {
            text.to_string()
        } else {
            format!("{}. {}.", sentences[0].trim(), sentences[sentences.len() - 1].trim())
        };

        let confidence = if sentences.len() > 1 { 0.6 } else { 0.3 };

        Ok((summary, Some(confidence)))
    }

    /// Classify the text
    async fn classify_text(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Classifying text of length: {}", text.len());

        // Simple keyword-based classification
        let text_lower = text.to_lowercase();
        
        let categories = vec![
            ("technology", vec!["computer", "software", "ai", "machine learning", "programming", "tech"]),
            ("business", vec!["money", "profit", "company", "market", "investment", "finance"]),
            ("sports", vec!["game", "play", "team", "sport", "competition", "athlete"]),
            ("health", vec!["doctor", "medicine", "health", "hospital", "treatment", "disease"]),
            ("entertainment", vec!["movie", "music", "show", "entertainment", "actor", "film"]),
        ];

        let mut best_category = "general";
        let mut best_score = 0;

        for (category, keywords) in categories {
            let score = keywords.iter()
                .map(|keyword| text_lower.matches(keyword).count())
                .sum::<usize>();
            
            if score > best_score {
                best_score = score;
                best_category = category;
            }
        }

        let confidence = if best_score > 0 {
            (best_score as f32 * 0.2).min(0.9)
        } else {
            0.1
        };

        let result = format!(
            "{{\"category\": \"{}\", \"confidence\": {:.2}, \"keyword_matches\": {}}}",
            best_category, confidence, best_score
        );

        Ok((result, Some(confidence)))
    }

    /// Extract keywords from the text
    async fn extract_keywords(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Extracting keywords from text of length: {}", text.len());

        // Simple keyword extraction based on word frequency
        let stop_words = vec![
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by",
            "is", "are", "was", "were", "be", "been", "have", "has", "had", "do", "does", "did",
            "will", "would", "could", "should", "may", "might", "can", "this", "that", "these", "those"
        ];

        let text_lower = text.to_lowercase();
        let words: Vec<&str> = text_lower
            .split_whitespace()
            .filter(|word| {
                word.len() > 3 && !stop_words.contains(word)
            })
            .collect();

        let mut word_count: HashMap<&str, usize> = HashMap::new();
        for word in &words {
            *word_count.entry(word).or_insert(0) += 1;
        }

        let mut sorted_words: Vec<_> = word_count.iter().collect();
        sorted_words.sort_by(|a, b| b.1.cmp(a.1));

        let keywords: Vec<String> = sorted_words
            .iter()
            .take(5)
            .map(|(word, count)| format!("{} ({})", word, count))
            .collect();

        let result = format!("{{\"keywords\": {:?}}}", keywords);
        Ok((result, Some(0.8)))
    }

    /// Translate text (mock implementation)
    async fn translate_text(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Translating text of length: {}", text.len());

        // Mock translation - in reality you'd use a translation service or model
        let result = format!(
            "{{\"original\": \"{}\", \"translated\": \"[Translation not implemented - would translate to target language]\", \"source_language\": \"auto-detected\", \"target_language\": \"en\"}}",
            text.chars().take(100).collect::<String>()
        );

        Ok((result, Some(0.1))) // Low confidence for mock
    }

    /// Answer questions (mock implementation)
    async fn answer_question(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Processing question of length: {}", text.len());

        // Simple question detection and mock answering
        let text_lower = text.to_lowercase();
        
        let answer = if text_lower.contains("what") {
            "This appears to be a 'what' question. A proper QA model would analyze the context and provide a specific answer."
        } else if text_lower.contains("how") {
            "This appears to be a 'how' question. A proper QA model would provide step-by-step instructions or explanations."
        } else if text_lower.contains("why") {
            "This appears to be a 'why' question. A proper QA model would provide reasoning and explanations."
        } else if text_lower.contains("when") {
            "This appears to be a 'when' question. A proper QA model would provide temporal information."
        } else if text_lower.contains("where") {
            "This appears to be a 'where' question. A proper QA model would provide location-based information."
        } else {
            "I can detect this is a question, but would need a proper QA model to provide a meaningful answer."
        };

        let result = format!(
            "{{\"question\": \"{}\", \"answer\": \"{}\", \"type\": \"mock_response\"}}",
            text.chars().take(100).collect::<String>(),
            answer
        );

        Ok((result, Some(0.3))) // Low confidence for mock
    }

    // === System Command Handlers ===

    /// Handle install command
    async fn handle_install(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Processing install command: {}", text);
        
        let package = text.trim();
        if package.is_empty() {
            return Ok((format!("{{\"command\": \"install\", \"error\": \"Package name required\", \"usage\": \"install <package_name>\"}}"), Some(0.9)));
        }
        
        let result = format!(
            "{{\"command\": \"install\", \"package\": \"{}\", \"suggested_commands\": [\"brew install {}\", \"npm install {}\", \"cargo install {}\", \"pip install {}\"]}}",
            package, package, package, package, package
        );
        
        Ok((result, Some(0.9)))
    }

    /// Handle find file command
    async fn handle_find_file(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Processing find file command: {}", text);
        
        let filename = text.trim();
        if filename.is_empty() {
            return Ok((format!("{{\"command\": \"find_file\", \"error\": \"Filename required\", \"usage\": \"find_file <filename>\"}}"), Some(0.9)));
        }
        
        let result = format!(
            "{{\"command\": \"find_file\", \"filename\": \"{}\", \"suggested_commands\": [\"find . -name '{}'\", \"find . -iname '{}'\", \"locate {}\", \"fd {}\"]}}",
            filename, filename, filename, filename, filename
        );
        
        Ok((result, Some(0.9)))
    }

    /// Handle find content command
    async fn handle_find_content(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Processing find content command: {}", text);
        
        let search_term = text.trim();
        if search_term.is_empty() {
            return Ok((format!("{{\"command\": \"find_content\", \"error\": \"Search term required\", \"usage\": \"find_content <search_term>\"}}"), Some(0.9)));
        }
        
        let result = format!(
            "{{\"command\": \"find_content\", \"search_term\": \"{}\", \"suggested_commands\": [\"grep -r '{}' .\", \"rg '{}'\", \"ag '{}'\", \"find . -type f -exec grep -l '{}' {{}} \\;\"]}}",
            search_term, search_term, search_term, search_term, search_term
        );
        
        Ok((result, Some(0.9)))
    }

    /// Handle get file from command
    async fn handle_get_file_from(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Processing get file from command: {}", text);
        
        let source = text.trim();
        if source.is_empty() {
            return Ok((format!("{{\"command\": \"get_file_from\", \"error\": \"Source required\", \"usage\": \"get_file_from <url_or_path>\"}}"), Some(0.9)));
        }
        
        let suggested_commands = if source.starts_with("http") {
            vec![
                format!("curl -O {}", source),
                format!("wget {}", source),
                format!("curl -L {} -o filename", source),
            ]
        } else {
            vec![
                format!("cp {} .", source),
                format!("rsync -av {} .", source),
                format!("scp {} .", source),
            ]
        };
        
        let result = format!(
            "{{\"command\": \"get_file_from\", \"source\": \"{}\", \"suggested_commands\": {:?}}}",
            source, suggested_commands
        );
        
        Ok((result, Some(0.9)))
    }

    /// Handle show tools command
    async fn handle_show_tools(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Processing show tools command: {}", text);
        
        let category = text.trim().to_lowercase();
        
        let tools = if category.is_empty() || category == "all" {
            serde_json::json!({
                "development": ["git", "cargo", "npm", "yarn", "docker", "kubectl"],
                "system": ["brew", "apt", "yum", "systemctl", "ps", "top", "htop"],
                "file_management": ["ls", "find", "grep", "sed", "awk", "rsync", "tar"],
                "network": ["curl", "wget", "ssh", "scp", "ping", "netstat"],
                "text_editors": ["vim", "nano", "code", "emacs", "sublime"]
            })
        } else {
            match category.as_str() {
                "dev" | "development" => serde_json::json!(["git", "cargo", "npm", "yarn", "docker", "kubectl"]),
                "system" => serde_json::json!(["brew", "apt", "yum", "systemctl", "ps", "top", "htop"]),
                "file" | "files" => serde_json::json!(["ls", "find", "grep", "sed", "awk", "rsync", "tar"]),
                "network" => serde_json::json!(["curl", "wget", "ssh", "scp", "ping", "netstat"]),
                "editor" | "editors" => serde_json::json!(["vim", "nano", "code", "emacs", "sublime"]),
                _ => serde_json::json!({"error": "Unknown category", "available_categories": ["development", "system", "file_management", "network", "text_editors"]})
            }
        };
        
        let result = format!(
            "{{\"command\": \"show_tools\", \"category\": \"{}\", \"tools\": {}}}",
            if category.is_empty() { "all" } else { &category },
            tools
        );
        
        Ok((result, Some(0.9)))
    }

    /// Handle open app command
    async fn handle_open_app(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Processing open app command: {}", text);
        
        let app_name = text.trim();
        if app_name.is_empty() {
            return Ok((format!("{{\"command\": \"open_app\", \"error\": \"App name required\", \"usage\": \"open_app <app_name>\"}}"), Some(0.9)));
        }
        
        let result = format!(
            "{{\"command\": \"open_app\", \"app_name\": \"{}\", \"suggested_commands\": [\"open -a '{}'\", \"open /Applications/{}.app\", \"osascript -e 'tell application \\\"{}\\\" to activate'\"]}}",
            app_name, app_name, app_name, app_name
        );
        
        Ok((result, Some(0.9)))
    }

    /// Handle open file command
    async fn handle_open_file(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Processing open file command: {}", text);
        
        let file_path = text.trim();
        if file_path.is_empty() {
            return Ok((format!("{{\"command\": \"open_file\", \"error\": \"File path required\", \"usage\": \"open_file <file_path>\"}}"), Some(0.9)));
        }
        
        let result = format!(
            "{{\"command\": \"open_file\", \"file_path\": \"{}\", \"suggested_commands\": [\"open '{}'\", \"code '{}'\", \"vim '{}'\", \"cat '{}'\"]}}",
            file_path, file_path, file_path, file_path, file_path
        );
        
        Ok((result, Some(0.9)))
    }

    /// Handle checkout command (Git)
    async fn handle_checkout(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Processing checkout command: {}", text);
        
        let branch_or_commit = text.trim();
        if branch_or_commit.is_empty() {
            return Ok((format!("{{\"command\": \"checkout\", \"error\": \"Branch or commit required\", \"usage\": \"checkout <branch_or_commit>\"}}"), Some(0.9)));
        }
        
        let result = format!(
            "{{\"command\": \"checkout\", \"target\": \"{}\", \"suggested_commands\": [\"git checkout {}\", \"git checkout -b {}\", \"git switch {}\", \"git switch -c {}\"]}}",
            branch_or_commit, branch_or_commit, branch_or_commit, branch_or_commit, branch_or_commit
        );
        
        Ok((result, Some(0.9)))
    }

    /// Handle diff command
    async fn handle_diff(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Processing diff command: {}", text);
        
        let files_or_commits = text.trim();
        
        let suggested_commands = if files_or_commits.is_empty() {
            vec![
                "git diff".to_string(),
                "git diff --staged".to_string(),
                "git diff HEAD~1".to_string(),
                "git status".to_string(),
            ]
        } else if files_or_commits.contains(' ') {
            // Likely two files or commits
            let parts: Vec<&str> = files_or_commits.split_whitespace().collect();
            if parts.len() >= 2 {
                vec![
                    format!("diff {} {}", parts[0], parts[1]),
                    format!("git diff {} {}", parts[0], parts[1]),
                    format!("code --diff {} {}", parts[0], parts[1]),
                ]
            } else {
                vec![format!("git diff {}", files_or_commits)]
            }
        } else {
            vec![
                format!("git diff {}", files_or_commits),
                format!("git diff HEAD {}", files_or_commits),
                format!("git show {}", files_or_commits),
            ]
        };
        
        let result = format!(
            "{{\"command\": \"diff\", \"target\": \"{}\", \"suggested_commands\": {:?}}}",
            files_or_commits, suggested_commands
        );
        
        Ok((result, Some(0.9)))
    }

    /// Handle Google search command
    async fn handle_google_search(&self, text: &str) -> Result<(String, Option<f32>)> {
        info!("Processing Google search command: {}", text);
        
        let query = text.trim();
        if query.is_empty() {
            return Ok((format!("{{\"command\": \"google_search\", \"error\": \"Search query required\", \"usage\": \"google_search <search_query>\"}}"), Some(0.9)));
        }
        
        // URL encode the search query
        let encoded_query = query.replace(" ", "+").replace("&", "%26").replace("?", "%3F");
        let google_url = format!("https://www.google.com/search?q={}", encoded_query);
        
        let suggested_commands = vec![
            format!("open '{}'", google_url),
            format!("curl -s '{}' | grep -i title", google_url),
            format!("python3 -m webbrowser '{}'", google_url),
            format!("osascript -e \"open location \\\"{}\\\"\"", google_url),
        ];
        
        let result = format!(
            "{{\"command\": \"google_search\", \"query\": \"{}\", \"google_url\": \"{}\", \"suggested_commands\": {:?}}}",
            query, google_url, suggested_commands
        );
        
        Ok((result, Some(0.9)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nlp_processor_initialization() {
        let processor = NlpProcessor::new().await.unwrap();
        let tasks = processor.list_available_tasks();
        
        assert_eq!(tasks.len(), 16);
        assert!(tasks.contains(&"sentiment".to_string()));
        assert!(tasks.contains(&"summarize".to_string()));
        assert!(tasks.contains(&"classify".to_string()));
        assert!(tasks.contains(&"extract_keywords".to_string()));
        assert!(tasks.contains(&"translate".to_string()));
        assert!(tasks.contains(&"question_answer".to_string()));
    }

    #[tokio::test]
    async fn test_sentiment_analysis_positive() {
        let processor = NlpProcessor::new().await.unwrap();
        let (result, confidence) = processor
            .analyze_sentiment("I love this amazing wonderful product!")
            .await
            .unwrap();
        
        assert!(result.contains("positive"));
        assert!(confidence.is_some());
        assert!(confidence.unwrap() > 0.7);
    }

    #[tokio::test]
    async fn test_sentiment_analysis_negative() {
        let processor = NlpProcessor::new().await.unwrap();
        let (result, confidence) = processor
            .analyze_sentiment("This is terrible awful bad horrible")
            .await
            .unwrap();
        
        assert!(result.contains("negative"));
        assert!(confidence.is_some());
        assert!(confidence.unwrap() > 0.7);
    }

    #[tokio::test]
    async fn test_sentiment_analysis_neutral() {
        let processor = NlpProcessor::new().await.unwrap();
        let (result, confidence) = processor
            .analyze_sentiment("This is a simple statement.")
            .await
            .unwrap();
        
        assert!(result.contains("neutral"));
        assert!(confidence.is_some());
        assert_eq!(confidence.unwrap(), 0.5);
    }

    #[tokio::test]
    async fn test_text_summarization() {
        let processor = NlpProcessor::new().await.unwrap();
        let text = "This is the first sentence. Here is some middle content that should be ignored. This is the final sentence.";
        let (result, confidence) = processor
            .summarize_text(text)
            .await
            .unwrap();
        
        assert!(result.contains("This is the first sentence"));
        assert!(result.contains("This is the final sentence"));
        assert!(confidence.is_some());
        assert_eq!(confidence.unwrap(), 0.6);
    }

    #[tokio::test]
    async fn test_text_classification() {
        let processor = NlpProcessor::new().await.unwrap();
        
        // Test technology classification
        let (result, confidence) = processor
            .classify_text("I love programming in Rust and machine learning with AI")
            .await
            .unwrap();
        
        assert!(result.contains("technology"));
        assert!(confidence.is_some());
        assert!(confidence.unwrap() > 0.2);
    }

    #[tokio::test]
    async fn test_keyword_extraction() {
        let processor = NlpProcessor::new().await.unwrap();
        let text = "programming programming programming artificial intelligence technology development";
        let (result, confidence) = processor
            .extract_keywords(text)
            .await
            .unwrap();
        
        assert!(result.contains("programming"));
        assert!(result.contains("keywords"));
        assert!(confidence.is_some());
        assert_eq!(confidence.unwrap(), 0.8);
    }

    #[tokio::test]
    async fn test_question_answering() {
        let processor = NlpProcessor::new().await.unwrap();
        
        // Test "what" question
        let (result, confidence) = processor
            .answer_question("What is machine learning?")
            .await
            .unwrap();
        
        assert!(result.contains("what"));
        assert!(confidence.is_some());
        assert_eq!(confidence.unwrap(), 0.3);
        
        // Test "how" question
        let (result, _) = processor
            .answer_question("How does this work?")
            .await
            .unwrap();
        
        assert!(result.contains("how"));
    }

    #[tokio::test]
    async fn test_translation_mock() {
        let processor = NlpProcessor::new().await.unwrap();
        let (result, confidence) = processor
            .translate_text("Hello world")
            .await
            .unwrap();
        
        assert!(result.contains("Translation not implemented"));
        assert!(confidence.is_some());
        assert_eq!(confidence.unwrap(), 0.1);
    }

    #[tokio::test]
    async fn test_process_empty_text() {
        let processor = NlpProcessor::new().await.unwrap();
        let result = processor.process("", "sentiment").await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[tokio::test]
    async fn test_process_unsupported_task() {
        let processor = NlpProcessor::new().await.unwrap();
        let result = processor.process("test text", "unsupported_task").await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unsupported task"));
    }

    #[tokio::test]
    async fn test_process_valid_tasks() {
        let processor = NlpProcessor::new().await.unwrap();
        let tasks = processor.list_available_tasks();
        
        for task in tasks {
            let result = processor.process("test text for processing", &task).await;
            assert!(result.is_ok(), "Task {} should work", task);
        }
    }

    // === System Command Tests ===

    #[tokio::test]
    async fn test_install_command() {
        let processor = NlpProcessor::new().await.unwrap();
        let (result, confidence) = processor
            .handle_install("rust")
            .await
            .unwrap();
        
        assert!(result.contains("install"));
        assert!(result.contains("rust"));
        assert!(result.contains("brew install rust"));
        assert!(confidence.is_some());
        assert_eq!(confidence.unwrap(), 0.9);
    }

    #[tokio::test]
    async fn test_find_file_command() {
        let processor = NlpProcessor::new().await.unwrap();
        let (result, confidence) = processor
            .handle_find_file("main.rs")
            .await
            .unwrap();
        
        assert!(result.contains("find_file"));
        assert!(result.contains("main.rs"));
        assert!(result.contains("find . -name"));
        assert!(confidence.is_some());
        assert_eq!(confidence.unwrap(), 0.9);
    }

    #[tokio::test]
    async fn test_find_content_command() {
        let processor = NlpProcessor::new().await.unwrap();
        let (result, confidence) = processor
            .handle_find_content("TODO")
            .await
            .unwrap();
        
        assert!(result.contains("find_content"));
        assert!(result.contains("TODO"));
        assert!(result.contains("grep -r"));
        assert!(confidence.is_some());
        assert_eq!(confidence.unwrap(), 0.9);
    }

    #[tokio::test]
    async fn test_show_tools_command() {
        let processor = NlpProcessor::new().await.unwrap();
        let (result, confidence) = processor
            .handle_show_tools("development")
            .await
            .unwrap();
        
        assert!(result.contains("show_tools"));
        assert!(result.contains("git"));
        assert!(result.contains("cargo"));
        assert!(confidence.is_some());
        assert_eq!(confidence.unwrap(), 0.9);
    }

    #[tokio::test]
    async fn test_checkout_command() {
        let processor = NlpProcessor::new().await.unwrap();
        let (result, confidence) = processor
            .handle_checkout("main")
            .await
            .unwrap();
        
        assert!(result.contains("checkout"));
        assert!(result.contains("main"));
        assert!(result.contains("git checkout"));
        assert!(confidence.is_some());
        assert_eq!(confidence.unwrap(), 0.9);
    }

    #[tokio::test]
    async fn test_diff_command() {
        let processor = NlpProcessor::new().await.unwrap();
        let (result, confidence) = processor
            .handle_diff("")
            .await
            .unwrap();
        
        assert!(result.contains("diff"));
        assert!(result.contains("git diff"));
        assert!(confidence.is_some());
        assert_eq!(confidence.unwrap(), 0.9);
    }

    #[tokio::test]
    async fn test_google_search_command() {
        let processor = NlpProcessor::new().await.unwrap();
        let (result, confidence) = processor
            .handle_google_search("rust programming language")
            .await
            .unwrap();
        
        assert!(result.contains("google_search"));
        assert!(result.contains("rust programming language"));
        assert!(result.contains("https://www.google.com/search?q=rust+programming+language"));
        assert!(result.contains("open"));
        assert!(confidence.is_some());
        assert_eq!(confidence.unwrap(), 0.9);
    }
}
