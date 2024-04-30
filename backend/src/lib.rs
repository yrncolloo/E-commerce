pub mod utils;
pub mod database;
mod routes;

use axum::http::StatusCode;
use routes::create_route;
use utils::{app_error::AppError, app_state::AppState};

pub async fn launch(app_state: AppState)-> Result<(), AppError>{
    let apps_state = app_state.clone();
    let app = create_route(app_state);

   let address = format!("{}:{}", apps_state.base_url.url, apps_state.base_url.port);
    let listenter = tokio::net::TcpListener::bind(&address)
        .await
        .map_err(|error|{
            eprintln!("There was an issue with the bind address {}, {}", address, error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Could not connect with the address and port")
        })?;

    axum::serve(listenter, app)
        .await
        .map_err(|error|{
            eprintln!("Could not start the server: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Could not start the server")
        })

}

