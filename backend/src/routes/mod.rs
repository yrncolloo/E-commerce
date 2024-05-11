mod register;
mod login;
mod logout;
mod items;
use axum::{middleware, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};

use crate::utils::{app_state::AppState, custom_middleware::guard_routes};
use self::{items::list_items::{list_all_items, list_one_item}, login::login, logout::logout, register::register};

pub fn create_route(app_state: AppState) -> Router{
    Router::new()
        .route("/api/v1/logout", post(logout))
        .route("/api/v1/list/:id", get(list_one_item))
        .route("/api/v1/list", get(list_all_items))
        .route("/api/v1/test", get(test))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), guard_routes))
        .route("/api/v1/register", post(register))
        .route("/api/v1/login", post(login))
        .with_state(app_state)

}

pub async fn test() -> String{
    
    "hehehhe".to_owned()
}

#[derive(Serialize, Deserialize)]
pub struct RespondUser{
    user_id: i32,
    username: String,
    telephone: String,
    email: String,
    token: Option<String>
}
