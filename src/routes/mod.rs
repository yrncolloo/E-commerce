use axum::Router;

use crate::utils::app_state::AppState;

use self::backend::backend_routes;

pub mod backend;
pub mod frontend;

pub fn create_route(app_state: AppState) -> Router {
    
    Router::new()
        .nest("/api/v1", backend_routes(app_state))
}
