
use axum::{body::Body, extract::{Request, State}, middleware::Next, response::Response};
use axum_extra::headers::{authorization::{self, Bearer}, Authorization, HeaderMapExt};
use reqwest::{StatusCode};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::{app_error::AppError, app_state::TokenWrapper, jwt::validate_jwt};
use crate::database::customer::{self, Entity as Users};

pub async fn frontend_guard(
    State(database): State<DatabaseConnection>,

    State(jwt_secret): State<TokenWrapper>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {

    let token = req.headers().typed_get::<Authorization<Bearer>>()
        .ok_or(AppError::new(StatusCode::UNAUTHORIZED, "No token found"))?
        .token()
        .to_owned();

    let user = Users::find()
        .filter(customer::Column::Token.eq(&token))
        .one(&database)
        .await
        .map_err(|error|{
            eprintln!("Error finding user: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
        })?;
    validate_jwt(&jwt_secret.0, &token)?;

    if let Some(user) = user{
        req.extensions_mut().insert(user);

    }else {
        return Err( AppError::new(StatusCode::UNAUTHORIZED, "You are not authorized"));
    }


    Ok(next.run(req).await)


}
