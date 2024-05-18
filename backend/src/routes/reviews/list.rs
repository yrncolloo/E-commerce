use axum::{extract::{Path, State}, http::StatusCode, Json};
use chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::utils::app_error::AppError;
use crate::database::product_review::Entity as Review;

#[derive(Serialize, Deserialize)]
pub struct PostReviews{
    review_id: i32,
    product_id: i32,
    customer_id: i32,
    rating: i32,
    review_message: String,
    review_date: NaiveDateTime


}
pub async fn list_reviews(
    State(database): State<DatabaseConnection>,
    Path(id): Path<i32>
    ) -> Result<Json<PostReviews>, AppError>{
    let review = Review::find_by_id(id)
        .one(&database)
        .await
        .map_err(|error|{
            eprintln!("[*] Error finding the review {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Review not found")
        })?;
    if let Some(review) = review{
        return Ok(Json(PostReviews { review_id: review.id, product_id: review.product_id, customer_id: review.customer_id, rating: review.rating, review_message: review.review_text.unwrap(), review_date: review.review_date.unwrap() }));
    }else {
      Err(AppError::new(StatusCode::NOT_FOUND, "Review not found"))  
    }
    
}

pub async fn list_all_reviews(
    State(database): State<DatabaseConnection>
    ) -> Result<Json<Vec<PostReviews>>, AppError>{

    let reviews = Review::find()
        .all(&database)
        .await
        .map_err(|error|{
            eprintln!("[*] Error finding the review, {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
        })?;

   let post: Vec<PostReviews> = reviews.iter()
        .map(|rev| PostReviews{
            review_id: rev.id,
            product_id: rev.product_id,
            customer_id: rev.customer_id,
            rating: rev.rating,
            review_message: rev.review_text.clone().unwrap(),
            review_date: rev.review_date.unwrap()
        }).collect();

   Ok(Json(post)) 
}
