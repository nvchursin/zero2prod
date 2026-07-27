use actix_web::{HttpResponse, http::header::ContentType};
use zero2prod_frontend::{PageData, render_document};

pub fn page(data: PageData) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(render_document(data))
}
