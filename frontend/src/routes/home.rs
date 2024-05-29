use askama::Template;
use axum::response::{IntoResponse, Response};

use super::HtmlTemplate;

#[derive(Template)]
#[template(path="index.html")]
pub struct HomeHtml;



pub async fn home_page() -> Response{
   
    let temp = HomeHtml{};
    
    HtmlTemplate(temp).into_response()
}
