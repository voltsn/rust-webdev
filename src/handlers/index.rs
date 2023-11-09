use crate::utils::html_template::*;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;
pub async fn index() -> HtmlTemplate<IndexTemplate>{
    HtmlTemplate(IndexTemplate)
}