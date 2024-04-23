use backend::{launch, utils::{app_error::AppError, app_state::AppState}};
use dotenvy_macro::dotenv;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    
    dotenvy::dotenv().ok();
    let port = dotenv!("PORT").to_string();
    let base_address = dotenv!("BASE_ADDRESS").to_string();
    let app_state = AppState{
        database: "none".to_string(),
        address: base_address,
        port

    };
    launch(app_state).await?;
    Ok(())
}
