mod users;

use axum::{routing::post, Router};

use crate::utils::app_state::AppState;
use self::users::register;

pub fn create_route(app_state: AppState) -> Router{
    Router::new()
        .route("/register", post(register))
        .with_state(app_state)

}
