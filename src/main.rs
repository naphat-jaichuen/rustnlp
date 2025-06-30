use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};
use uuid::Uuid;

mod nlp;

use nlp::NlpProcessor;

#[derive(Debug, Serialize, Deserialize)]
struct ProcessRequest {
    text: String,
    task: String, // "sentiment", "summarize", "classify", etc.
}

#[derive(Debug, Serialize)]
struct ProcessResponse {
    id: Uuid,
    input_text: String,
    task: String,
    result: String,
    confidence: Option<f32>,
    processing_time_ms: u64,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

#[derive(Clone)]
struct AppState {
    nlp_processor: Arc<NlpProcessor>,
}

fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/process", post(process_text))
        .route("/process/:task", post(process_text_with_task))
        .route("/models", get(list_available_models))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Initializing NLP processor...");
    let nlp_processor = Arc::new(NlpProcessor::new().await?);
    info!("NLP processor initialized successfully");

    let state = AppState { nlp_processor };
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Server starting on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "rustlm-server",
        "version": "0.1.0"
    }))
}

async fn process_text(
    State(state): State<AppState>,
    Json(request): Json<ProcessRequest>,
) -> Result<Json<ProcessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let start_time = std::time::Instant::now();
    let request_id = Uuid::new_v4();

    info!(
        "Processing request {} for task: {}, text length: {}",
        request_id,
        request.task,
        request.text.len()
    );

    match state
        .nlp_processor
        .process(&request.text, &request.task)
        .await
    {
        Ok((result, confidence)) => {
            let processing_time = start_time.elapsed();
            
            info!(
                "Request {} completed in {}ms",
                request_id,
                processing_time.as_millis()
            );

            Ok(Json(ProcessResponse {
                id: request_id,
                input_text: request.text,
                task: request.task,
                result,
                confidence,
                processing_time_ms: processing_time.as_millis() as u64,
            }))
        }
        Err(e) => {
            warn!("Processing failed for request {}: {}", request_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "processing_failed".to_string(),
                    message: e.to_string(),
                }),
            ))
        }
    }
}

async fn process_text_with_task(
    Path(task): Path<String>,
    State(state): State<AppState>,
    Json(text_request): Json<serde_json::Value>,
) -> Result<Json<ProcessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let text = text_request
        .get("text")
        .and_then(|t| t.as_str())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "invalid_request".to_string(),
                    message: "Missing 'text' field in request body".to_string(),
                }),
            )
        })?;

    let request = ProcessRequest {
        text: text.to_string(),
        task,
    };

    process_text(State(state), Json(request)).await
}

async fn list_available_models(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let models = state.nlp_processor.list_available_tasks();
    Json(serde_json::json!({
        "available_tasks": models,
        "description": "List of available NLP processing tasks"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use axum::http::Request;
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn test_health_check() {
        let nlp_processor = Arc::new(NlpProcessor::new().await.unwrap());
        let state = AppState { nlp_processor };
        let app = create_app(state);

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "healthy");
    }

    #[tokio::test]
    async fn test_process_sentiment() {
        let nlp_processor = Arc::new(NlpProcessor::new().await.unwrap());
        let state = AppState { nlp_processor };
        let app = create_app(state);

        let request_body = serde_json::json!({ "text": "I love Rust!", "task": "sentiment" });
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/process")
                    .header("content-type", "application/json")
                    .body(Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["task"], "sentiment");
        assert!(json["result"].to_string().contains("positive"));
    }

    #[tokio::test]
    async fn test_process_task_specific_endpoint() {
        let nlp_processor = Arc::new(NlpProcessor::new().await.unwrap());
        let state = AppState { nlp_processor };
        let app = create_app(state);

        let request_body = serde_json::json!({ "text": "This is terrible!" });
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/process/sentiment")
                    .header("content-type", "application/json")
                    .body(Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["task"], "sentiment");
        assert!(json["result"].to_string().contains("negative"));
    }

    #[tokio::test]
    async fn test_list_available_models() {
        let nlp_processor = Arc::new(NlpProcessor::new().await.unwrap());
        let state = AppState { nlp_processor };
        let app = create_app(state);

        let response = app
            .oneshot(Request::builder().uri("/models").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert!(json["available_tasks"].is_array());
        let tasks = json["available_tasks"].as_array().unwrap();
        assert!(tasks.contains(&serde_json::Value::String("sentiment".to_string())));
    }

    #[tokio::test]
    async fn test_invalid_task() {
        let nlp_processor = Arc::new(NlpProcessor::new().await.unwrap());
        let state = AppState { nlp_processor };
        let app = create_app(state);

        let request_body = serde_json::json!({ "text": "Test text", "task": "invalid_task" });
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/process")
                    .header("content-type", "application/json")
                    .body(Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 500);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["error"], "processing_failed");
    }

    #[tokio::test]
    async fn test_empty_text() {
        let nlp_processor = Arc::new(NlpProcessor::new().await.unwrap());
        let state = AppState { nlp_processor };
        let app = create_app(state);

        let request_body = serde_json::json!({ "text": "", "task": "sentiment" });
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/process")
                    .header("content-type", "application/json")
                    .body(Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 500);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["error"], "processing_failed");
    }
}
