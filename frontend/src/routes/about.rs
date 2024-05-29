use askama::Template;
use axum::response::{IntoResponse, Response};

use super::HtmlTemplate;

#[derive(Template)]
#[template(path = "about.html")]
pub struct About;


pub async fn about_page() -> Response {

    let about_temp = About{

    };
    
    HtmlTemplate(about_temp).into_response()
}
