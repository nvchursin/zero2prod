use crate::{
    authentication::UserId,
    idempotency::{IdempotencyKey, NextAction, save_response, try_processing},
    utils::{error400, error500, see_other},
};
use actix_web::{HttpResponse, post, web};
use actix_web_flash_messages::FlashMessage;
use sqlx::{PgPool, Postgres, Transaction};
use tracing::instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    title: String,
    text_content: String,
    html_content: String,
    idempotency_key: String,
}

#[instrument(skip_all)]
async fn enqueue_delivery_tasks(
    transaction: &mut Transaction<'_, Postgres>,
    newsletter_issue_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO issue_delivery_queue (
        newsletter_issue_id,
        subscriber_email
        )
        SELECT $1, email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#,
        newsletter_issue_id,
    )
    .execute(transaction.as_mut())
    .await?;

    Ok(())
}

#[instrument(skip_all)]
async fn insert_newsletter_issue(
    transaction: &mut Transaction<'_, Postgres>,
    title: &str,
    text_content: &str,
    html_content: &str,
) -> Result<Uuid, sqlx::Error> {
    let newsletter_issue_id = Uuid::new_v4();
    sqlx::query!(
        r#"
INSERT INTO newsletter_issues (
newsletter_issue_id,
title,
text_content,
html_content,
published_at
)
VALUES ($1, $2, $3, $4, now())
"#,
        newsletter_issue_id,
        title,
        text_content,
        html_content
    )
    .execute(transaction.as_mut())
    .await?;

    Ok(newsletter_issue_id)
}

fn success_message() -> FlashMessage {
    FlashMessage::info("The newsletter issue has been accepted - emails will go out shortly")
}

#[tracing::instrument(
    name = "Publish a newsletter issue",
    skip(form, pool)
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
#[post("/newsletters")]
pub async fn publish_newsletter(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    let FormData {
        title,
        text_content,
        html_content,
        idempotency_key,
    } = form.0;

    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(error400)?;

    let mut transaction = match try_processing(&pool, &idempotency_key, *user_id)
        .await
        .map_err(error500)?
    {
        NextAction::StartProcessing(transaction) => transaction,
        NextAction::ReturnSavedResponse(saved_response) => {
            success_message().send();
            return Ok(saved_response);
        }
    };

    let issue_id = insert_newsletter_issue(&mut transaction, &title, &text_content, &html_content)
        .await
        .map_err(|err| {
            error500(anyhow::Error::new(err).context("Failed to store newsletter issue details"))
        })?;

    enqueue_delivery_tasks(&mut transaction, issue_id)
        .await
        .map_err(|err| {
            error500(anyhow::Error::new(err).context("Failed to enqueue delivery tasks"))
        })?;

    let response = see_other("/admin/newsletters");
    let response = save_response(transaction, &idempotency_key, *user_id, response)
        .await
        .map_err(error500)?;

    success_message().send();
    Ok(response)
}
