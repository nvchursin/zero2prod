use actix_web::{HttpResponse, get};
use zero2prod_frontend::PageData;

#[get("/")]
async fn index() -> HttpResponse {
    super::render::page(PageData::Home)
}
