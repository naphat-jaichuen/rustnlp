use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};

/// Callback result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackResult {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub execution_time_ms: u64,
}

/// Command context passed to callbacks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandContext {
    pub command: String,
    pub task: String,
    pub input_text: String,
    pub parsed_result: String,
    pub confidence: Option<f32>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub session_id: Option<String>,
}

/// Callback handler enum that can handle different types of callbacks
#[derive(Debug)]
pub enum CallbackHandler {
    System(SystemCommandHandler),
    Nlp(NlpCallbackHandler),
}

impl CallbackHandler {
    pub async fn handle(&self, context: &CommandContext) -> Result<CallbackResult> {
        match self {
            CallbackHandler::System(handler) => handler.handle(context).await,
            CallbackHandler::Nlp(handler) => handler.handle(context).await,
        }
    }

    pub fn get_supported_commands(&self) -> Vec<String> {
        match self {
            CallbackHandler::System(handler) => handler.get_supported_commands(),
            CallbackHandler::Nlp(handler) => handler.get_supported_commands(),
        }
    }

    pub fn get_handler_name(&self) -> String {
        match self {
            CallbackHandler::System(handler) => handler.get_handler_name(),
            CallbackHandler::Nlp(handler) => handler.get_handler_name(),
        }
    }
}

/// Default callback handlers for system commands
#[derive(Debug)]
pub struct SystemCommandHandler;


impl SystemCommandHandler {
    pub async fn handle(&self, context: &CommandContext) -> Result<CallbackResult> {
        let start_time = std::time::Instant::now();
        info!("Executing system command callback for: {}", context.command);

        match context.command.as_str() {
            "install" => self.handle_install_callback(context).await,
            "find_file" => self.handle_find_file_callback(context).await,
            "find_content" => self.handle_find_content_callback(context).await,
            "get_file_from" => self.handle_get_file_from_callback(context).await,
            "show_tools" => self.handle_show_tools_callback(context).await,
            "open_app" => self.handle_open_app_callback(context).await,
            "open_file" => self.handle_open_file_callback(context).await,
            "checkout" => self.handle_checkout_callback(context).await,
            "diff" => self.handle_diff_callback(context).await,
            "google_search" => self.handle_google_search_callback(context).await,
            "ask_ai" => self.handle_ask_ai_callback(context).await,
            _ => Err(anyhow!("Unsupported command: {}", context.command)),
        }.map(|mut result| {
            result.execution_time_ms = start_time.elapsed().as_millis() as u64;
            result
        })
    }

    pub fn get_supported_commands(&self) -> Vec<String> {
        vec![
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
            "ask_ai".to_string(),
        ]
    }

    pub fn get_handler_name(&self) -> String {
        "SystemCommandHandler".to_string()
    }
    async fn handle_install_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing install callback for: {}", context.input_text);
        
        // Parse the result to get suggested commands
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        if let Some(commands) = parsed_data.get("suggested_commands").and_then(|c| c.as_array()) {
            // Here you could actually execute the commands or log them
            let command_list: Vec<String> = commands.iter()
                .filter_map(|c| c.as_str())
                .map(|s| s.to_string())
                .collect();
            
            info!("Install commands ready for execution: {:?}", command_list);
            
            Ok(CallbackResult {
                success: true,
                message: format!("Install callback processed for package: {}", context.input_text),
                data: Some(serde_json::json!({
                    "action": "install_ready",
                    "package": context.input_text,
                    "suggested_commands": command_list,
                    "next_steps": "Commands are ready for execution"
                })),
                execution_time_ms: 0, // Will be set by caller
            })
        } else {
            warn!("Could not parse install commands from result");
            Ok(CallbackResult {
                success: false,
                message: "Failed to parse install commands".to_string(),
                data: None,
                execution_time_ms: 0,
            })
        }
    }

    async fn handle_find_file_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing find file callback for: {}", context.input_text);
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: format!("Find file callback processed for: {}", context.input_text),
            data: Some(serde_json::json!({
                "action": "file_search_ready",
                "filename": context.input_text,
                "search_commands": parsed_data.get("suggested_commands"),
                "status": "search_prepared"
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_find_content_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing find content callback for: {}", context.input_text);
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: format!("Find content callback processed for: {}", context.input_text),
            data: Some(serde_json::json!({
                "action": "content_search_ready",
                "search_term": context.input_text,
                "search_commands": parsed_data.get("suggested_commands"),
                "status": "search_prepared"
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_get_file_from_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing get file from callback for: {}", context.input_text);
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: format!("Get file from callback processed for: {}", context.input_text),
            data: Some(serde_json::json!({
                "action": "file_download_ready",
                "source": context.input_text,
                "download_commands": parsed_data.get("suggested_commands"),
                "status": "download_prepared"
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_show_tools_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing show tools callback for: {}", context.input_text);
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: format!("Show tools callback processed for category: {}", context.input_text),
            data: Some(serde_json::json!({
                "action": "tools_listed",
                "category": context.input_text,
                "tools": parsed_data.get("tools"),
                "status": "tools_displayed"
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_open_app_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing open app callback for: {}", context.input_text);
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: format!("Open app callback processed for: {}", context.input_text),
            data: Some(serde_json::json!({
                "action": "app_launch_ready",
                "app_name": context.input_text,
                "launch_commands": parsed_data.get("suggested_commands"),
                "status": "app_launch_prepared"
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_open_file_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing open file callback for: {}", context.input_text);
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: format!("Open file callback processed for: {}", context.input_text),
            data: Some(serde_json::json!({
                "action": "file_open_ready",
                "file_path": context.input_text,
                "open_commands": parsed_data.get("suggested_commands"),
                "status": "file_open_prepared"
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_checkout_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing checkout callback for: {}", context.input_text);
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: format!("Checkout callback processed for: {}", context.input_text),
            data: Some(serde_json::json!({
                "action": "checkout_ready",
                "target": context.input_text,
                "checkout_commands": parsed_data.get("suggested_commands"),
                "status": "checkout_prepared"
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_diff_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing diff callback for: {}", context.input_text);
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: format!("Diff callback processed for: {}", context.input_text),
            data: Some(serde_json::json!({
                "action": "diff_ready",
                "target": context.input_text,
                "diff_commands": parsed_data.get("suggested_commands"),
                "status": "diff_prepared"
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_google_search_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing Google search callback for: {}", context.input_text);
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: format!("Google search callback processed for: {}", context.input_text),
            data: Some(serde_json::json!({
                "action": "search_ready",
                "query": context.input_text,
                "search_url": parsed_data.get("google_url"),
                "search_commands": parsed_data.get("suggested_commands"),
                "status": "search_prepared"
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_ask_ai_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing Ask AI callback for: {}", context.input_text);
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: format!("Ask AI callback processed for: {}", context.input_text),
            data: Some(serde_json::json!({
                "action": "ai_query_processed",
                "question": context.input_text,
                "ai_response": parsed_data.get("answer"),
                "status": parsed_data.get("status").unwrap_or(&serde_json::Value::String("completed".to_string())),
            })),
            execution_time_ms: 0,
        })
    }
}

/// Natural Language Processing callback handler
#[derive(Debug)]
pub struct NlpCallbackHandler;


impl NlpCallbackHandler {
    pub async fn handle(&self, context: &CommandContext) -> Result<CallbackResult> {
        let start_time = std::time::Instant::now();
        info!("Executing NLP callback for: {}", context.command);

        let result = match context.command.as_str() {
            "sentiment" => self.handle_sentiment_callback(context).await,
            "summarize" => self.handle_summarize_callback(context).await,
            "classify" => self.handle_classify_callback(context).await,
            "extract_keywords" => self.handle_keywords_callback(context).await,
            "translate" => self.handle_translate_callback(context).await,
            "question_answer" => self.handle_qa_callback(context).await,
            "natural_language" => self.handle_natural_language_callback(context).await,
            _ => Err(anyhow!("Unsupported NLP command: {}", context.command)),
        };

        result.map(|mut r| {
            r.execution_time_ms = start_time.elapsed().as_millis() as u64;
            r
        })
    }

    pub fn get_supported_commands(&self) -> Vec<String> {
        vec![
            "sentiment".to_string(),
            "summarize".to_string(),
            "classify".to_string(),
            "extract_keywords".to_string(),
            "translate".to_string(),
            "question_answer".to_string(),
            "natural_language".to_string(),
        ]
    }

    pub fn get_handler_name(&self) -> String {
        "NlpCallbackHandler".to_string()
    }

    async fn handle_sentiment_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing sentiment analysis callback");
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: "Sentiment analysis completed".to_string(),
            data: Some(serde_json::json!({
                "action": "sentiment_analyzed",
                "text": context.input_text,
                "analysis": parsed_data,
                "confidence": context.confidence
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_summarize_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing text summarization callback");
        
        Ok(CallbackResult {
            success: true,
            message: "Text summarization completed".to_string(),
            data: Some(serde_json::json!({
                "action": "text_summarized",
                "original_text": context.input_text,
                "summary": context.parsed_result,
                "confidence": context.confidence
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_classify_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing text classification callback");
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: "Text classification completed".to_string(),
            data: Some(serde_json::json!({
                "action": "text_classified",
                "text": context.input_text,
                "classification": parsed_data,
                "confidence": context.confidence
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_keywords_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing keyword extraction callback");
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: "Keyword extraction completed".to_string(),
            data: Some(serde_json::json!({
                "action": "keywords_extracted",
                "text": context.input_text,
                "keywords": parsed_data,
                "confidence": context.confidence
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_translate_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing translation callback");
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: "Translation completed".to_string(),
            data: Some(serde_json::json!({
                "action": "text_translated",
                "original_text": context.input_text,
                "translation": parsed_data,
                "confidence": context.confidence
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_qa_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing question answering callback");
        
        let parsed_data: serde_json::Value = serde_json::from_str(&context.parsed_result)?;
        
        Ok(CallbackResult {
            success: true,
            message: "Question answering completed".to_string(),
            data: Some(serde_json::json!({
                "action": "question_answered",
                "question": context.input_text,
                "answer": parsed_data,
                "confidence": context.confidence
            })),
            execution_time_ms: 0,
        })
    }

    async fn handle_natural_language_callback(&self, context: &CommandContext) -> Result<CallbackResult> {
        info!("Processing natural language callback");
        
        Ok(CallbackResult {
            success: true,
            message: "Natural language processing completed".to_string(),
            data: Some(serde_json::json!({
                "action": "natural_language_processed",
                "input": context.input_text,
                "parsed_intent": context.parsed_result,
                "confidence": context.confidence,
                "next_steps": "Intent parsed and ready for execution"
            })),
            execution_time_ms: 0,
        })
    }
}

/// Main callback manager that routes commands to appropriate handlers
pub struct CallbackManager {
    handlers: HashMap<String, CallbackHandler>,
}

impl CallbackManager {
    /// Create a new callback manager with default handlers
    pub fn new() -> Self {
        let mut manager = Self {
            handlers: HashMap::new(),
        };
        
        // Register default handlers
        manager.register_handler("system", CallbackHandler::System(SystemCommandHandler));
        manager.register_handler("nlp", CallbackHandler::Nlp(NlpCallbackHandler));
        
        manager
    }

    /// Register a new callback handler
    pub fn register_handler(&mut self, name: &str, handler: CallbackHandler) {
        info!("Registering callback handler: {}", name);
        self.handlers.insert(name.to_string(), handler);
    }

    /// Execute callbacks for a command
    pub async fn execute_callback(&self, context: &CommandContext) -> Result<Vec<CallbackResult>> {
        let mut results = Vec::new();
        
        for (name, handler) in &self.handlers {
            if handler.get_supported_commands().contains(&context.command) {
                info!("Executing callback {} for command: {}", name, context.command);
                
                match handler.handle(context).await {
                    Ok(result) => {
                        info!("Callback {} completed successfully", name);
                        results.push(result);
                    }
                    Err(e) => {
                        warn!("Callback {} failed: {}", name, e);
                        results.push(CallbackResult {
                            success: false,
                            message: format!("Callback {} failed: {}", name, e),
                            data: None,
                            execution_time_ms: 0,
                        });
                    }
                }
            }
        }
        
        if results.is_empty() {
            info!("No callbacks found for command: {}", context.command);
            results.push(CallbackResult {
                success: true,
                message: format!("No callbacks registered for command: {}", context.command),
                data: None,
                execution_time_ms: 0,
            });
        }
        
        Ok(results)
    }

    /// Get all supported commands across all handlers
    pub fn get_all_supported_commands(&self) -> Vec<String> {
        let mut all_commands = Vec::new();
        
        for handler in self.handlers.values() {
            all_commands.extend(handler.get_supported_commands());
        }
        
        all_commands.sort();
        all_commands.dedup();
        all_commands
    }

    /// Get handler information
    pub fn get_handler_info(&self) -> Vec<serde_json::Value> {
        self.handlers
            .iter()
            .map(|(name, handler)| {
                serde_json::json!({
                    "handler_name": name,
                    "handler_type": handler.get_handler_name(),
                    "supported_commands": handler.get_supported_commands()
                })
            })
            .collect()
    }
}

impl Default for CallbackManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_callback_manager_creation() {
        let manager = CallbackManager::new();
        let commands = manager.get_all_supported_commands();
        
        assert!(!commands.is_empty());
        assert!(commands.contains(&"sentiment".to_string()));
        assert!(commands.contains(&"install".to_string()));
    }

    #[tokio::test]
    async fn test_system_command_callback() {
        let handler = SystemCommandHandler;
        let context = CommandContext {
            command: "install".to_string(),
            task: "install".to_string(),
            input_text: "rust".to_string(),
            parsed_result: r#"{"command": "install", "package": "rust", "suggested_commands": ["brew install rust", "cargo install rust"]}"#.to_string(),
            confidence: Some(0.9),
            timestamp: chrono::Utc::now(),
            session_id: None,
        };

        let result = handler.handle(&context).await.unwrap();
        assert!(result.success);
        assert!(result.message.contains("Install callback processed"));
    }

    #[tokio::test]
    async fn test_nlp_callback() {
        let handler = NlpCallbackHandler;
        let context = CommandContext {
            command: "sentiment".to_string(),
            task: "sentiment".to_string(),
            input_text: "I love this!".to_string(),
            parsed_result: r#"{"sentiment": "positive", "positive_score": 1, "negative_score": 0}"#.to_string(),
            confidence: Some(0.8),
            timestamp: chrono::Utc::now(),
            session_id: None,
        };

        let result = handler.handle(&context).await.unwrap();
        assert!(result.success);
        assert!(result.message.contains("Sentiment analysis completed"));
    }

    #[tokio::test]
    async fn test_callback_execution() {
        let manager = CallbackManager::new();
        let context = CommandContext {
            command: "sentiment".to_string(),
            task: "sentiment".to_string(),
            input_text: "Great product!".to_string(),
            parsed_result: r#"{"sentiment": "positive"}"#.to_string(),
            confidence: Some(0.9),
            timestamp: chrono::Utc::now(),
            session_id: None,
        };

        let results = manager.execute_callback(&context).await.unwrap();
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.success));
    }
}
