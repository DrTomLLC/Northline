use northline_core::{run_query, AppConfig, UserQuery};

#[test]
fn run_query_currently_returns_error() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = AppConfig {};
    let query = UserQuery { conversation_id: None, text: "test".into() };
    let result = run_query(&cfg, query);
    assert!(result.is_err());
    Ok(())
}
