use axum::{extract::State, Json};
use chrono::Utc;
use hyper::StatusCode;
use sea_orm::prelude::Decimal;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter, TryIntoModel};
use sea_orm::{prelude::DateTimeUtc, DatabaseConnection};
use serde::{Deserialize, Serialize};
use sea_orm::Set;

use crate::utils::app_error::AppError;
use crate::database::product as ProductDB;
use crate::database::categories::{self as CategoryDB, Model};
use crate::database::colors as ColorDB;

#[derive(Serialize, Deserialize, Clone)]
pub struct GetProducts{
    product_name: String,
    price: Decimal,
    image_name: String,
    total_quantity: i32,
    category: String,
    description: String,
    color: String,
    quantity_per_color: i32
}


#[derive(Serialize, Deserialize)]
pub struct ProductsResponse{
    product_name: String,
    price: String,
    image_name: String,
    date_added: String,
    total_quantity: String,
    category: String,
    color: String,
    color_quantity: String,
    size: String,
    size_quantity: String,
}

pub async fn create_products( State(database): State<DatabaseConnection>,
Json(raw_products): Json<GetProducts>
) ->  Result<Json<ProductsResponse>, AppError>{

    let find_category = CategoryDB::Entity::find()
        .filter(CategoryDB::Column::Name.eq(&raw_products.category))
        .one(&database)
        .await
        .map_err(|error|{
            eprintln!("error finding category: {}", error);
            AppError::new(StatusCode::NOT_FOUND, "category not found")
        })?;


    if find_category.is_none(){

        let category = CategoryDB::ActiveModel{
            name: Set(raw_products.category.clone()),
            ..Default::default()
        }.save(&database)
        .await
            .map_err(|error|{
                let error_mess = error.to_string();

                if error_mess.contains("duplicate key value violates unique constraint"){
                    eprintln!("category already added");
                    return AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "kjsafdlk");
                }

                eprintln!("Error saving category {}", error);
                return AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong");
            })?;

        let category_db_id = category.try_into_model()
            .map_err(|error|{
                eprintln!("Error converting category into model {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
            })?;

        finish_building(database, category_db_id, raw_products).await

    }else{

        let category_db_id = find_category.unwrap().try_into_model()
            .map_err(|error|{
                eprintln!("Error converting category into model {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
            })?;

        finish_building(database, category_db_id, raw_products).await
    }
}


async fn finish_building(database: DatabaseConnection, 
    category_db_id:Model, raw_products: GetProducts) -> Result<Json<ProductsResponse>, AppError>{


    let now = Utc::now();
    let date_product_added = now.naive_utc();

    let product = ProductDB::ActiveModel{
        category_id:  Set(category_db_id.id),
        product_name: Set(raw_products.product_name),
        description: Set(raw_products.description),
        image_name: Set(raw_products.image_name),
        price: Set(raw_products.price),
        quantity: Set(raw_products.total_quantity),
        date_added: Set(date_product_added),
        ..Default::default()
    }.save(&database)
    .await
        .map_err(|error|{
                let error_mess = error.to_string();

                if error_mess.contains("duplicate key value violates unique constraint"){
                    eprintln!("Product already exist");
                    return AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Product already exist");
                }
                eprintln!("Error saving the product {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
        })?;

    let final_product = product.try_into_model()
        .map_err(|error|{
            eprintln!("Error converting product into model{}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
        })?;

    let color = ColorDB::ActiveModel{
        color: Set(raw_products.color),
        quantity: Set(raw_products.quantity_per_color),
        product_id: Set(final_product.id),
        ..Default::default()
    }.save(&database)
    .await
    .map_err(|error|{
        eprintln!("Error saving color: {}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
    })?;
    let color = color.try_into_model()
        .map_err(|error|{
            eprintln!("Error converting product into model{}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
        })?;


    Ok(Json(ProductsResponse { 
        product_name: final_product.product_name,
        price: final_product.price.to_string(),
        image_name: final_product.image_name,
        date_added: final_product.date_added.to_string(),
        total_quantity: final_product.quantity.to_string(),
        category: category_db_id.name,
        color: color.color,
        color_quantity: color.quantity.to_string(),
        size: "to be checked".to_string(),
        size_quantity: "to be checked".to_string()
    }
    ))


}
