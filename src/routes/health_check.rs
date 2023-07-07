use actix_web::{HttpResponse, Responder};

pub async fn heath_check() -> impl Responder {
    HttpResponse::Ok()
}