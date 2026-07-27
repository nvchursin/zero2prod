use actix_web::{HttpResponse, get};
use actix_web_flash_messages::IncomingFlashMessages;
use zero2prod_frontend::PageData;

#[get("/newsletters")]
pub async fn newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let messages = flash_messages
        .iter()
        .map(|message| message.content().to_owned())
        .collect();

    let idempotency_key = uuid::Uuid::new_v4();

    Ok(super::super::super::render::page(PageData::Newsletters {
        messages,
        idempotency_key: idempotency_key.to_string(),
    }))
}
