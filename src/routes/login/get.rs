use actix_web::{HttpResponse, get};
use actix_web_flash_messages::IncomingFlashMessages;
use zero2prod_frontend::PageData;

#[get("/login")]
pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let messages = flash_messages
        .iter()
        .map(|message| message.content().to_owned())
        .collect();

    super::super::render::page(PageData::Login { messages })
}
