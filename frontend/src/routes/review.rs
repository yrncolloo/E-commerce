use askama::Template;
use axum::response::{IntoResponse, Response};

use super::HtmlTemplate;

#[derive(Template)]
#[template(path="review.html")]
pub struct Reviews;


pub async fn review_page() -> Response{
    
    let review_tmp = Reviews{

    };
    HtmlTemplate(review_tmp).into_response()
}
