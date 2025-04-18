pub mod api_chat_stream;
pub mod api_reverse_proxy;
mod errors;
mod jwt;
pub mod limits;
mod prompt;
pub mod sse_chat_enricher;
pub mod sse_chat_error;
pub mod synthesize;
pub mod token_count;
pub mod ui_chat_stream;
use axum::Router;
use axum_extra::routing::RouterExt;
use tower_http::cors::{Any, CorsLayer};

use axum_extra::routing::TypedPath;
use serde::Deserialize;

// Re-export the OpenAI API data models from the openai-api crate
pub use openai_api::{
    Completion, FunctionDefinition, Message, Tool, ToolCall, ToolCallFunction, ToolResult,
};

pub fn routes() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allows requests from any origin
        .allow_methods(Any) // Allows any HTTP method
        .allow_headers(Any); // Allows any header

    Router::new()
        .typed_get(api_chat_stream::chat_generate)
        .typed_post(api_chat_stream::chat_generate)
        .typed_post(synthesize::synthesize)
        .typed_get(api_reverse_proxy::handler)
        .typed_post(api_reverse_proxy::handler)
        .typed_post(ui_chat_stream::chat_generate)
        .typed_get(ui_chat_stream::chat_generate)
        .layer(cors) // Apply the CORS layer
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/completions/{chat_id}")]
pub struct UICompletions {
    pub chat_id: i32,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/app/synthesize")]
pub struct UISynthesize {}

#[derive(TypedPath, Deserialize)]
#[typed_path("/v1/{*path}")]
pub struct LLMHandler {
    pub path: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/v1/chat/completions")]
pub struct ApiChatHandler {}
