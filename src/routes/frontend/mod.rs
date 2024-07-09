pub mod handlers;
pub mod contact;
pub mod shop;

use askama::Template;
use axum::{extract::{Request, State}, middleware::{self, Next}, response::{Html, IntoResponse, Response}, routing::{get, post}, Router};
use hyper::StatusCode;
use reqwest::Client;

use crate::utils::{app_error::AppError, app_state::AppState, custom_frontend_middleware::{self, frontend_guard}, custom_middleware::guard_routes};

use self::{contact::contact_us, handlers::{cart, checkout, contact, detail, home}, shop::shop};

use super::backend::RespondUser;


pub fn frontend_routes(app_state: AppState) -> Router{
    let client = Client::new();

    Router::new()
        .route("/contact", get(contact))
        .route("/contact", post(contact_us))
        .route("/detail", get(detail))
        .route("/checkout", get(checkout))
        .route("/shop", get(shop))
        .route("/cart", get(cart))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), frontend_guard))
        .route("/", get(home))
        .layer(
            middleware::from_fn(inject_token)
            )
        //.route_layer(middleware::from_fn_with_state(app_state.clone(), frontend_guard))
        .with_state(client)
        .with_state(app_state)
        
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


async fn inject_token(
   mut req: Request,
    next: Next
    ) -> Result<Response, AppError>{

    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3MjA1MjUwNDAsImlhdCI6MTcyMDUyNDQ0MH0.P-k6mtl0XERRBLE7-WX6dxH2AFAIdXLNnoz47F-vlTU".to_string();
    let tkn = format!("Bearer {}", token).parse().unwrap();
   req.headers_mut().insert("Authorization", tkn);

    Ok(next.run(req).await)
}

