use anyhow::Result;
use chrono::Utc;
use rustlm_server::{NlpCallbackHandler, CommandContext};
use serde_json;
use std::env;
use std::io::{self, Read};
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting summarize handler");
    
    // Get input from command line arguments or stdin
    let input_text = get_input()?;
    let parsed_result = get_parsed_result()?;
    
    // Create command context
    let context = CommandContext {
        command: "summarize".to_string(),
        task: "summarize".to_string(),
        input_text,
        parsed_result,
        confidence: Some(0.9),
        timestamp: Utc::now(),
        session_id: None,
    };
    
    // Create handler and execute
    let handler = NlpCallbackHandler;
    match handler.handle(&context).await {
        Ok(result) => {
            println!("{}", serde_json::to_string_pretty(&result)?);
            if result.success {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("Handler failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_input() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        Ok(args[1..].join(" "))
    } else {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer.trim().to_string())
    }
}

fn get_parsed_result() -> Result<String> {
    // You can get this from environment variable or another argument
    if let Ok(result) = env::var("PARSED_RESULT") {
        Ok(result)
    } else {
        // Default parsed result for summarize command
        Ok(r#"{"action": "summarize", "result": "Default summarize result"}"#.to_string())
    }
}
