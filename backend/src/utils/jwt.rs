use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims{
    exp: usize,
    iat: usize
}
pub fn create_jwt(jwt_secret: &str) -> Result<String, AppError> {
    
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp_time = Duration::try_minutes(10).expect("Enter valid number");
    let exp = (now + exp_time).timestamp() as usize;

    let claims = Claims{
        iat,
        exp

    };
    let key = &EncodingKey::from_secret(jwt_secret.as_bytes());
    encode(&Header::default(), &claims, key)
        .map_err(|error|{
            eprintln!("Could not create jwt {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error occured when creating jwt token, Please try again later")
        })

}
pub fn validate_jwt(jwt_secret: &str, token: &str) -> Result<bool, AppError>{
    let key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let validation = &Validation::new(Algorithm::HS256);

    decode::<Claims>(token, &key, &validation)
        .map_err(|error|
                 match error.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::new(StatusCode::UNAUTHORIZED, "Your session has expired"),
                    _=> {
                        eprintln!("Error validating token: {}", error);
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong please try again later")
                    }

                 })?;
    
    Ok(true)
}
