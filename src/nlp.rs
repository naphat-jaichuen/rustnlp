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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nlp_processor_initialization() {
        let processor = NlpProcessor::new().await.unwrap();
        let tasks = processor.list_available_tasks();
        
        assert_eq!(tasks.len(), 6);
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
}
