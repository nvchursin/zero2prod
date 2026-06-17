use actix_web::{HttpResponse, Responder, get};

#[get("/healthz")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
