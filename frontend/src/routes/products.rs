use askama::Template;
use axum::response::{IntoResponse, Response};

use super::HtmlTemplate;

#[derive(Template)]
#[template(path="products.html")]
pub struct Prouduct;


pub async fn product_page() -> Response{
    
    let proudut_pg = Prouduct{

    };
    HtmlTemplate(proudut_pg).into_response()

}
