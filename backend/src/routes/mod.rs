mod register;
mod login;
mod logout;
mod items;
mod reviews;
use axum::{middleware, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};

use crate::utils::{app_state::AppState, custom_middleware::guard_routes};
use self::{items::list_items::{list_all_items, list_one_item}, login::login, logout::logout, register::register, reviews::new_review::create_review};

pub fn create_route(app_state: AppState) -> Router{
    Router::new()
        .route("/api/v1/review", post(create_review))
        .route("/api/v1/logout", post(logout))
        .route("/api/v1/list/:id", get(list_one_item))
        .route("/api/v1/list", get(list_all_items))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), guard_routes))
        .route("/api/v1/register", post(register))
        .route("/api/v1/login", post(login))
        .with_state(app_state)

}


#[derive(Serialize, Deserialize)]
pub struct RespondUser{
    user_id: i32,
    username: String,
    telephone: String,
    email: String,
    token: Option<String>
}
