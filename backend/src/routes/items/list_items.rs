use axum::{extract::{Path, State}, http::StatusCode, Json};
use sea_orm::{prelude::Decimal, DatabaseConnection, EntityTrait };
use serde::Serialize;

use crate::{database::product::Entity as Product, utils::app_error::AppError};

#[derive(Serialize)]
pub struct Items{
    name: String,
    description: String,
    image_id: Option<String>,
    price: Decimal,
    stock: i32

}
pub async fn list_one_item(
    State(database): State<DatabaseConnection>,
    Path(id): Path<i32>
    ) -> Result<Json<Items>, AppError> {
    
    let item = Product::find_by_id(id)
        .one(&database)
        .await
        .map_err(|error|{
            eprintln!("Error: Getting tasks from the database, {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was something that went wrong")
        })?;
    if let Some(item) = item{
        return Ok(Json(Items {
            name: item.name, 
            description: item.description, 
            image_id: item.image_id, 
            price: item.price, 
            stock: item.stock }));
    }else {
        
        Err(AppError::new(StatusCode::NOT_FOUND, "product not found"))
    }

}


pub async fn list_all_items(
    State(database): State<DatabaseConnection>
    ) -> Result<Json<Vec<Items>>, AppError>{
    
    let items: Vec<Items> = Product::find()
        .all(&database)
        .await
        .map_err(|error|{
            eprintln!("Error getting all tasks, {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
        })?
    .into_iter()
    .map(|db_item| Items{
        name: db_item.name,
        description: db_item.description,
        image_id: db_item.image_id,
        price: db_item.price,
        stock: db_item.stock
    })
    .collect();

    Ok(Json(items))
}
