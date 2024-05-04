use axum::http::StatusCode;
use axum::{extract::State, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::utils::app_error::AppError;
use crate::database::customer::{self, Entity as User};
use crate::utils::app_state::TokenWrapper;
use crate::utils::hash::verifiy_pass;
use crate::utils::jwt::create_jwt;

use super::RespondUser;

#[derive(Serialize, Deserialize)]
pub struct RequestLoginUser{
    pub username: String,
    pub password: String
}
pub async fn login(
    State(database): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    Json(requet_user): Json<RequestLoginUser>
    )-> Result<Json<RespondUser>, AppError> {
    
    let user = User::find()
        .filter(customer::Column::Username.eq(requet_user.username))
        .one(&database)
        .await
        .map_err(|error|{
            eprintln!("Error finding the user {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error login in, Please try again later")
        })?;
    if let Some(user) = user{
        // verify password
        if !verifiy_pass(requet_user.password, &user.password_hash)?{
            return Err(AppError::new(StatusCode::UNAUTHORIZED, "Bad username OR password"));
        }

        // generate jwt
        let token = create_jwt(&jwt_secret.0)?;
        
        let mut user = user.into_active_model();
        user.token = Set(Some(token));
        let saved_user = user.save(&database)
            .await
            .map_err(|error|{
                eprintln!("Error, could not save the token: {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
            })?;
        let user = saved_user.try_into_model()
            .map_err(|error|{
                eprintln!("Error, Could not convert into model: {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
            })?;
        Ok(Json(RespondUser{email: user.email, telephone: user.telephone, user_id: user.id, username:user.username, token: user.token}))

    }else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Bad username OR password"));
    }
}
