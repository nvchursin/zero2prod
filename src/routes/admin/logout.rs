use actix_web::{HttpResponse, post};
use actix_web_flash_messages::FlashMessage;

use crate::{
    session_state::TypedSession,
    utils::{error500, see_other},
};

#[post("/logout")]
pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    match session.get_user_id().map_err(error500)? {
        Some(_) => {
            session.log_out();
            FlashMessage::info("You have successfully logged out").send();
            Ok(see_other("/login"))
        }
        None => Ok(see_other("/login")),
    }
}
