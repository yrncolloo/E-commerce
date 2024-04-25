use axum::{extract::State, routing::get, Router};
use sea_orm::DatabaseConnection;

use crate::utils::app_state::AppState;

pub fn create_route(app_state: AppState) -> Router{
    Router::new()
        .route("/test", get(test))
        .with_state(app_state)
        

}

pub async fn test(
    State(database): State<DatabaseConnection>,
    ) -> String{
    "hehehe".to_string()
    
}
