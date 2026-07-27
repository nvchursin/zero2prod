use actix_web::{HttpResponse, get, web};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;
use zero2prod_frontend::PageData;

use crate::{
    session_state::TypedSession,
    utils::{error500, see_other},
};

#[instrument(name = "Get username", skip(pool))]
pub async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
    SELECT username
    FROM users
    WHERE user_id = $1
    "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| anyhow::Error::msg("Failed to perform a query to retrieve a username"))?;

    Ok(row.username)
}

#[get("/dashboard")]
pub async fn admin_dashboard(
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(error500)? {
        get_username(user_id, &pool).await.map_err(error500)?
    } else {
        return Ok(see_other("/login"));
    };

    Ok(super::super::render::page(PageData::Dashboard { username }))
}
