use actix_web::{HttpResponse, post, web};
use actix_web_flash_messages::FlashMessage;
use sqlx::PgPool;

use crate::{
    authentication::{AuthError, Credentials, UserId, validate_credentials},
    routes::get_username,
    utils::{error500, see_other},
};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: String,
    new_password: String,
    new_password_check: String,
}

#[post("/password")]
pub async fn change_password(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    if form.new_password != form.new_password_check {
        FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();

        return Ok(see_other("/admin/password"));
    }

    let username = get_username(*user_id, &pool).await.map_err(error500)?;
    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };

    if let Err(err) = validate_credentials(credentials, &pool).await {
        return match err {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect").send();
                Ok(see_other("/admin/password"))
            }
            AuthError::UnexpectedError(_) => Err(error500(err)),
        };
    }

    crate::authentication::change_password(*user_id, form.0.new_password, &pool)
        .await
        .map_err(error500)?;
    FlashMessage::error("Your password has been changed").send();

    Ok(see_other("/admin/password"))
}
