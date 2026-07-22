use actix_web::{HttpResponse, get, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;

#[get("/login")]
pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let error_html: String = flash_messages
        .iter()
        .map(|msg| format!("<p><i>{}</i></p>", msg.content()))
        .collect();

    HttpResponse::Ok()
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
      flex-direction: column;
      justify-content: center;
      align-items: center;
      height: 100%;
    "
  >
  {error_html}
    <form
      action="/login"
      method="post"
      style="display: flex; flex-direction: column; gap: 16px; width: 320px"
    >
      <div style="display: flex; gap: 8px">
        <label
          >Username
          <input
            type="text"
            placeholder="Enter your username"
            name="username"
          />
        </label>
        <label
          >Password
          <input type="password" placeholder="Enter password" name="password" />
        </label>
      </div>
      <button type="submit">Login</button>
    </form>
  </body>
</html>
"#
        ))
}
