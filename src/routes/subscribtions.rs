use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(from_data: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().into()
}
