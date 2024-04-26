use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::utils::app_error::AppError;
use crate::database::customer as User;

#[derive(Serialize, Deserialize)]
pub struct RequestUser{
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    telephone: String,
    default_address_id: Option<i32>,
    password: String,
    salt: String,
    password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct RespondUser{
    user_id: i32,
    username: String,
    telephone: String,
    email: String
}

pub async fn register(
    State(database): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>
    ) -> Result<Json<RespondUser>, AppError>{

    let new_user = User::ActiveModel{
        username: Set(request_user.username),
        first_name: Set(request_user.first_name),
        last_name: Set(request_user.last_name),
        email: Set(request_user.email),
        telephone: Set(request_user.telephone),
        default_address_id: Set(request_user.default_address_id),
        password_hash: Set(request_user.password_hash),
        salt: Set(request_user.salt),
        ..Default::default()
    }.save(&database)
        .await
        .map_err(|error|{
            let error_mess = error.to_string();
            if error_mess.contains("duplicate key value violates unique constraint"){
                return AppError::new(StatusCode::BAD_REQUEST, "Another user having those details");
            }
            eprintln!("Error could not create new user: {} ", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Could not create the account, Please try again later")
        })?;
   let user = new_user.try_into_model()
        .map_err(|error|{
            eprintln!("Error, could not convert users into model: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
        })?;

    Ok(Json(RespondUser { user_id: user.id, username: user.username, telephone: user.telephone, email: user.email }))
    
}

