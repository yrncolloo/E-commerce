use axum::{extract::{Request, State}, http::StatusCode, middleware::Next, response::Response};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::{app_error::AppError, app_state::TokenWrapper};
use crate::{database::customer::{self, Entity as Users}, utils::jwt::validate_jwt};

pub async fn guard_routes(
    State(database): State<DatabaseConnection>,
    State(jwt_secret): State<TokenWrapper>,
    mut request: Request,
    next: Next
    )-> Result<Response, AppError> {
    
    let token = request.headers().typed_get::<Authorization<Bearer>>()
        .ok_or_else(||AppError::new(StatusCode::BAD_REQUEST, "Not authenticated Please login"))?
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
        request.extensions_mut().insert(user);

    }else {
        return Err( AppError::new(StatusCode::UNAUTHORIZED, "You are not authorized"));
    }
    Ok(next.run(request).await)
}

