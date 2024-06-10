use axum::{extract::State, http::StatusCode, Extension};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};

use crate::{database::customer, utils::app_error::AppError};


pub async fn logout(
    State(database): State<DatabaseConnection>,
    Extension(user): Extension<customer::Model>
    )-> Result<StatusCode, AppError>{

    let mut user = user.into_active_model();
    user.token = Set(None);
    user.save(&database)
        .await
        .map_err(|error|{
            eprintln!("Error login out: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error login out, please try again later")
        })?;
    Ok(StatusCode::OK)
}
