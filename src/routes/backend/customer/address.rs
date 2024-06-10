use axum::http::StatusCode;
use axum::{extract::State, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::utils::app_error::AppError;
use crate::database::address as Address;

#[derive(Serialize, Deserialize)]
pub struct RequestAddress{
    country: String,
    state: String,
    town: String,
    zip: String,
    address_line_1: String,
    address_line_2: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct RespondAddress{
    id: i32,
    country: String,
    state: String,
    town: String,
    zip: String,
    address_line_1: String,
    address_line_2: Option<String>

}

pub async fn create_address(
    State(database): State<DatabaseConnection>,
    Json(request_address): Json<RequestAddress>
    ) -> Result<Json<RespondAddress>, AppError> {
    
    let address = Address::ActiveModel{
        country: Set(request_address.country),
        state: Set(request_address.state),
        town: Set(request_address.town),
        zip: Set(request_address.zip),
        address_line_1: Set(request_address.address_line_1),
        address_line_2: Set(request_address.address_line_2),
        ..Default::default()
    }.save(&database)
    .await
    .map_err(|error|{
        eprintln!("[*] Error adding address to the database: {}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
    })?;

    let address_new_model = address.try_into_model()
        .map_err(|error|{
            eprintln!("[*] Error converting into model {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
        })?;

    Ok(Json(RespondAddress { id: address_new_model.id, country: address_new_model.country, state: address_new_model.state, town: address_new_model.town, zip: address_new_model.zip, address_line_1: address_new_model.address_line_1, address_line_2: address_new_model.address_line_2 }))
}
