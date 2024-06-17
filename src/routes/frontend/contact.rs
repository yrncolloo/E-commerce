use axum::{extract::rejection::FormRejection, response::{Html, IntoResponse}, Form};
use serde::{Deserialize, Serialize};

use crate::routes::frontend::{handlers::{ContactTemplate, ThankyouTemplate}, HtmlTemplate};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactDetails{
    name: String,
    email: String,
    subject: String,
    message: String
}

pub async fn contact_us(form_res: Result<Form<ContactDetails>, FormRejection> ) -> impl IntoResponse {
    dbg!(&form_res);
    match form_res {
        Ok(Form(details)) => {

            dbg!(&details);
            let message = format!("Thankyou {}, Your form has been submitted successfull", details.name);
            let temp = ThankyouTemplate{
                message
            };
            HtmlTemplate(temp)

        }
        Err(e) => {
            eprintln!("something went wrong: {}:", e);
            let message = format!("Something went wrong");
            let temp = ThankyouTemplate{
                message
            };
            HtmlTemplate(temp)
        }
        
    }
    


}
