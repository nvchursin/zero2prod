use actix_web::{HttpResponse, get, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;

#[get("/newsletters")]
pub async fn newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let html_message: String = flash_messages
        .iter()
        .map(|msg| format!("<p><i>{}</i></p>", msg.content()))
        .collect();

    let idempotency_key = uuid::Uuid::new_v4();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
        <!doctype html>
<html lang="en" style="height: 100%">
  <head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Login</title>
  </head>
  <body
    style="
      display: flex;
      justify-content: center;
      align-items: center;
      height: 100%;
    "
  >
    <form
      action="/admin/newsletters"
      method="post"
      style="display: flex; flex-direction: column; gap: 16px; width: 320px"
    >
      {html_message}
      <div style="display: flex; gap: 8px">
        <label
          >Title
          <input type="text" placeholder="Enter title" name="title" required />
        </label>
        <label
          >Text content
          <input
            type="textarea"
            placeholder="Enter text content"
            name="text_content"
            required
          />
        </label>
        <label
          >HTML content
          <input
            type="textarea"
            placeholder="Enter HTML content"
            name="html_content"
            required
          />
        </label>
      </div>
      <input hidden type="text" name="idempotency_key" value={idempotency_key}>
      <button type="submit">Post</button>
    </form>
  </body>
</html>
"#
        )))
}
