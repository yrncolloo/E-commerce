pub mod create_products;

use axum::{routing::post, Router};

use crate::utils::app_state::AppState;

use self::create_products::create_products;

pub fn admin_routes(app_state: AppState) -> Router{
    Router::new()
        .route("/create_products", post(create_products))
        .with_state(app_state)

}
