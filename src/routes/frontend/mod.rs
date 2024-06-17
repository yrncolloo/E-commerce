pub mod handlers;
pub mod contact;

use askama::Template;
use axum::{response::{Html, IntoResponse, Response}, routing::{get, post}, Router};
use hyper::StatusCode;

use self::{contact::contact_us, handlers::{cart, checkout, contact, detail, home, shop}};


pub fn frontend_routes() -> Router{
    
    Router::new()
        .route("/", get(home))
        .route("/contact", get(contact))
        .route("/contact", post(contact_us))
        .route("/detail", get(detail))
        .route("/checkout", get(checkout))
        .route("/shop", get(shop))
        .route("/cart", get(cart))
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



