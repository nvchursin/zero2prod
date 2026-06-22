use actix_web::{HttpResponse, Responder, get};
use tracing::instrument;

#[get("/healthz")]
#[instrument(name = "healthz")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
