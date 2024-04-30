mod register;
mod login;

use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};

use crate::utils::app_state::AppState;
use self::{login::login, register::register};

pub fn create_route(app_state: AppState) -> Router{
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
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
