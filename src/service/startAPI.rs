use actix_web::{get,HttpResponse, Responder};

#[get("/")]
pub async fn start() -> impl Responder {
    HttpResponse::Ok().body("Bienvenue")
}