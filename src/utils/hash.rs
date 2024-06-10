use axum::http::StatusCode;
use bcrypt::{hash, verify};

use super::app_error::AppError;

pub fn create_hash(password: String) -> Result<String, AppError>{
    hash(password, 13)
        .map_err(|error| {
            eprint!("error while hashing password: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error occured while hashing password")
        })
}

pub fn verifiy_pass(password: String, hash: &str) -> Result<bool, AppError> {
    
    verify(password, hash)
        .map_err(|error|{
            eprintln!("Error verifying your password: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong while verifying your password")
        })
}

