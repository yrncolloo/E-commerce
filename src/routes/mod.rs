use askama::filters::format;
use axum::Router;
use tower_http::services::ServeDir;

use crate::utils::app_state::AppState;

use self::{backend::backend_routes, frontend::frontend_routes};

pub mod backend;
pub mod frontend;

pub fn create_route(app_state: AppState) -> Router {
    
    let asset_path = std::env::current_dir().unwrap();
    Router::new()
        .nest("/api/v1", backend_routes(app_state))
        .nest("/", frontend_routes())
        .nest_service("/assets", ServeDir::new(format!("{}/assets", asset_path.to_str().unwrap())))
}
