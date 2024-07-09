use askama::Template;
use axum::{response::IntoResponse, Form};
use serde::{Deserialize, Serialize};

use super::{contact::ContactDetails, HtmlTemplate};

#[derive(Template)]
#[template(path="index.html")]
pub struct HomeTemplate{}

pub async fn home() -> impl IntoResponse{
    let temp = HomeTemplate{};
    HtmlTemplate(temp)

}

#[derive(Template)]
#[template(path="contact.html")]
pub struct ContactTemplate{}


pub async fn contact() -> impl IntoResponse{
    let temp = ContactTemplate{};
    HtmlTemplate(temp)

}

#[derive(Template)]
#[template(path="detail.html")]
pub struct DetailTemplate{}

pub async fn detail() -> impl IntoResponse{
    let temp = DetailTemplate{};
    HtmlTemplate(temp)

}

#[derive(Template)]
#[template(path="checkout.html")]
pub struct CheckoutTemplate{}

pub async fn checkout() -> impl IntoResponse{
    let temp = CheckoutTemplate{};
    HtmlTemplate(temp)

}



#[derive(Template)]
#[template(path="cart.html")]
pub struct CartTemplate{}

pub async fn cart() -> impl IntoResponse{
    let temp = CartTemplate{};
    HtmlTemplate(temp)

}

#[derive(Template)]
#[template(path="components/thankyou.html")]
pub struct ThankyouTemplate{
    pub message: String
}

