use axum::extract::FromRef;


//#[derive(FromRef)]
pub struct AppState{
    pub database: String,
    pub address: String,
    pub port: String,
}
