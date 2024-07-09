use askama::Template;
use axum::{extract::{Request, State}, http::HeaderValue, response::IntoResponse};
use hyper::{header::AUTHORIZATION, HeaderMap};
use reqwest::Client;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

use crate::utils::{app_state::AppState, custom_frontend_middleware::frontend_guard};

use super::HtmlTemplate;

#[derive(Template)]
#[template(path="shop.html")]
pub struct ShopTemplate{
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items{
    product_name: String,
    description: String,
    image_name: String,
    price: Decimal,
    quantity: String,
    date_added: String,
}

pub async fn shop(State(client): State<Client>, mut req: Request) -> impl IntoResponse{
    let url = "http://localhost:3000/api/v1/list";
    let jwt_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3MjA1NDMzODcsImlhdCI6MTcyMDU0Mjc4N30.969xF0E3uLS-VFYgGD34lI0vTQo9dfzs2cyYKwSFfRc";

    let tkn = format!("Bearer {}", jwt_token).parse().unwrap();
    req.headers_mut().insert("Authorization", tkn);

    let temp = ShopTemplate{
        token: jwt_token.to_string()
    };
    HtmlTemplate(temp)

}
