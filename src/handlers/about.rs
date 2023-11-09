use askama::Template;
use crate::utils::HtmlTemplate;

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate<'a>{
    name: &'a str,
}

pub async fn about<'a>() -> HtmlTemplate<AboutTemplate<'a>> {
    let about = AboutTemplate{
        name: "Evil Corp"
    };

    HtmlTemplate(about)
}