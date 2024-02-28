use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    println!("form data --- {:?}", _form);
    HttpResponse::Ok().finish()
}
