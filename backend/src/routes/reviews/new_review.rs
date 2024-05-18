use axum::http::StatusCode;
use axum::{extract::State, Json};
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TryIntoModel};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::utils::app_error::AppError;
use crate::database::{customer, product, product_review as Product_review};
use crate::database::customer::Entity as User;
use crate::database::product::Entity as Product;

#[derive(Serialize, Deserialize)]
pub struct GetReview{
    username: String,
    message: Option<String>,
    rating: i32,
    product_id: i32

}
#[derive(Serialize, Deserialize)]
pub struct RespondReview{
    review_id: i32,
    customer_id: i32,
    product_id: i32,
    rating: i32,
    rev_message: String,
    review_date: NaiveDateTime
}

pub async fn create_review(
    State(database): State<DatabaseConnection>,
    Json(review): Json<GetReview>
    ) -> Result<Json<RespondReview>, AppError>{

    let user = User::find()
        .filter(customer::Column::Username.eq(review.username))
        .one(&database)
        .await
        .map_err(|error|{
            eprintln!("error finding user: {}", error);
            AppError::new(StatusCode::NOT_FOUND, "User not found")
        })?;
    let product = Product::find()
        .filter(product::Column::Id.eq(review.product_id))
        .one(&database)
        .await
        .map_err(|error|{
            eprintln!("error finding the product: {}", error);
            AppError::new(StatusCode::NOT_FOUND, "Product NOT_FOUND")
        })?;
    let now = Utc::now();

    let rev_date = now.naive_utc();

    let product = Product_review::ActiveModel{
        review_text: Set(review.message),
        rating: Set(review.rating),
        customer_id: if let Some(user) = user{
            Set(user.id)
        }else{return Err(AppError::new(StatusCode::NOT_FOUND, "User not found"));},
        product_id: if let Some(product) = product{
            Set(product.id)
        }else {return Err(AppError::new(StatusCode::NOT_FOUND, "product not found"));},
        review_date: Set(Some(rev_date)),
        ..Default::default()
    }.save(&database)
    .await
    .map_err(|error|{
        eprintln!("Error saving review: {}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
    })?;
    
   let final_product = product.try_into_model()
        .map_err(|error| {
            eprintln!("Error converting into model: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
        })?;

    Ok(Json(RespondReview { review_id: final_product.id, customer_id: final_product.customer_id, product_id: final_product.product_id, rating: final_product.rating, rev_message: final_product.review_text.unwrap(), review_date: final_product.review_date.unwrap() }))
}
