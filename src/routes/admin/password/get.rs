use actix_web::{HttpResponse, get};
use actix_web_flash_messages::IncomingFlashMessages;
use zero2prod_frontend::PageData;

use crate::{
    session_state::TypedSession,
    utils::{error500, see_other},
};

#[get("/password")]
pub async fn change_password_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(error500)?.is_none() {
        return Ok(see_other("/login"));
    };

    let messages = flash_messages
        .iter()
        .map(|message| message.content().to_owned())
        .collect();

    Ok(super::super::super::render::page(PageData::Password {
        messages,
    }))
}
