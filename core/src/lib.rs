pub mod errors;
pub mod config;
pub mod providers;
pub mod tools;
pub mod engine;

pub use errors::{CoreError, ErrorCode};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AppConfig {
    // TODO: fill according to docs/ARCHITECTURE.md
}

#[derive(Debug, Clone)]
pub struct UserQuery {
    pub conversation_id: Option<Uuid>,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct AnswerChunk {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Source {
    pub url: String,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct ToolTrace {
    pub tool_name: String,
    pub input: String,
    pub output_summary: String,
}

#[derive(Debug, Clone)]
pub struct FinalAnswer {
    pub text: String,
    pub sources: Vec<Source>,
    pub tool_trace: Vec<ToolTrace>,
}

pub trait ModelBackend: Send + Sync {
    fn complete(
        &self,
        query: &UserQuery,
    ) -> Result<Box<dyn Iterator<Item = AnswerChunk> + Send>, CoreError>;

    fn embed(&self, text: &str) -> Result<Vec<f32>, CoreError>;
}

pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn invoke(&self, input: &str) -> Result<String, CoreError>;
}

pub fn run_query(
    _config: &AppConfig,
    _query: UserQuery,
) -> Result<FinalAnswer, CoreError> {
    Err(CoreError::new(
        ErrorCode::Internal,
        "run_query not implemented yet",
    ))
}
