pub mod home;
pub mod about;
pub mod products;
pub mod review;

use askama::Template;
use axum::{http::StatusCode, response::{Html, IntoResponse, Response}, routing::get, Router};

use self::{about::about_page, home::home_page, products::product_page, review::review_page};




pub fn create_routes() -> Router{
    
    Router::new()
        .route("/", get(home_page))
        .route("/about", get(about_page))
        .route("/products", get(product_page))
        .route("/review", get(review_page))
}


pub struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}



