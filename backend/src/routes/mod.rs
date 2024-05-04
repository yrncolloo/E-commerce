mod register;
mod login;
mod logout;

use axum::{middleware, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};

use crate::utils::{app_state::AppState, custom_middleware::guard_routes};
use self::{login::login, logout::logout, register::register};

pub fn create_route(app_state: AppState) -> Router{
    Router::new()
        .route("/logout", post(logout))
        .route("/test", get(test))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), guard_routes))
        .route("/register", post(register))
        .route("/login", post(login))
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
