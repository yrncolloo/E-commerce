use axum::extract::FromRef;
use sea_orm::DatabaseConnection;


#[derive(Clone, FromRef)]
pub struct AppState{
    pub database: DatabaseConnection,
    pub base_url: Wrapper,
}

#[derive(Clone)]
pub struct Wrapper{
    pub base_url: String,
    pub port: String
}
