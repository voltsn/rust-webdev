use axum::response::{Html, IntoResponse, Response};
use askama::Template;
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}
pub async fn index() -> Response {
    let hello = IndexTemplate{name: "world"};

    Html(hello.render().unwrap()).into_response()
}