use axum::extract::FromRef;
use sea_orm::DatabaseConnection;


#[derive(Clone, FromRef)]
pub struct AppState{
    pub database: DatabaseConnection,
    pub base_url: Wrapper,
    pub jwt_secret: TokenWrapper
}

#[derive(Clone)]
pub struct Wrapper{
    pub url: String,
    pub port: String
}

#[derive(Clone)]
pub struct TokenWrapper(pub String);
