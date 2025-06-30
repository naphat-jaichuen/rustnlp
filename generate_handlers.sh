#!/bin/bash

# Array of system commands
system_commands=(
    "find_file"
    "find_content" 
    "get_file_from"
    "show_tools"
    "open_app"
    "open_file"
    "checkout"
    "diff"
    "google_search"
    "ask_ai"
)

# Array of NLP commands
nlp_commands=(
    "sentiment"
    "summarize"
    "classify"
    "extract_keywords"
    "translate"
    "question_answer"
    "natural_language"
)

# Function to create system handler
create_system_handler() {
    local command=$1
    local filename="${command}_handler.rs"
    local command_display=${command//_/ }
    
    cat > "src/bin/$filename" << EOL
use anyhow::Result;
use chrono::Utc;
use rustlm_server::{SystemCommandHandler, CommandContext};
use serde_json;
use std::env;
use std::io::{self, Read};
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting $command_display handler");
    
    // Get input from command line arguments or stdin
    let input_text = get_input()?;
    let parsed_result = get_parsed_result()?;
    
    // Create command context
    let context = CommandContext {
        command: "$command".to_string(),
        task: "$command".to_string(),
        input_text,
        parsed_result,
        confidence: Some(0.9),
        timestamp: Utc::now(),
        session_id: None,
    };
    
    // Create handler and execute
    let handler = SystemCommandHandler;
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
        // Default parsed result for $command command
        Ok(r#"{"command": "$command", "suggested_commands": ["echo 'Add specific commands for $command_display here'"]}"#.to_string())
    }
}
EOL
}

# Function to create NLP handler
create_nlp_handler() {
    local command=$1
    local filename="${command}_handler.rs"
    local command_display=${command//_/ }
    
    cat > "src/bin/$filename" << EOL
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
    
    info!("Starting $command_display handler");
    
    // Get input from command line arguments or stdin
    let input_text = get_input()?;
    let parsed_result = get_parsed_result()?;
    
    // Create command context
    let context = CommandContext {
        command: "$command".to_string(),
        task: "$command".to_string(),
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
        // Default parsed result for $command command
        Ok(r#"{"action": "$command", "result": "Default $command_display result"}"#.to_string())
    }
}
EOL
}

# Generate system handlers
echo "Generating system command handlers..."
for cmd in "${system_commands[@]}"; do
    echo "Creating $cmd handler..."
    create_system_handler "$cmd"
done

# Generate NLP handlers
echo "Generating NLP command handlers..."
for cmd in "${nlp_commands[@]}"; do
    echo "Creating $cmd handler..."
    create_nlp_handler "$cmd"
done

echo "All handlers generated successfully!"
