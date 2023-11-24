use askama::Template;
use axum::debug_handler;

#[derive(Template)]
#[template(path = "test_template.html")]
pub struct TestTemplate;

#[debug_handler]
pub async fn test_html() -> TestTemplate {
  TestTemplate
}
