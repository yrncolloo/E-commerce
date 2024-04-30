use axum::http::StatusCode;
use backend::{launch, utils::{app_error::AppError, app_state::{AppState, TokenWrapper, Wrapper}}};
use dotenvy_macro::dotenv;
use sea_orm::Database;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    
    dotenvy::dotenv().ok();
    let port = dotenv!("PORT").to_string();
    let base_url = dotenv!("BASE_ADDRESS").to_string();
    let database_url = dotenv!("DATABASE_URL");
    let jwt_secret = dotenv!("JWT_SECRET").to_string();
    let database = Database::connect(database_url)
        .await
        .map_err(|error|{
            eprintln!("Error could not connect to the database: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Could not connect to the database")
        })?;
    let app_state = AppState{
        database,
        base_url: Wrapper { url: base_url, port },
        jwt_secret: TokenWrapper(jwt_secret)

    };
    launch(app_state).await?;
    Ok(())
}
