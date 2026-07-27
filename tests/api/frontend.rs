use crate::helpers::spawn_app;

#[tokio::test]
async fn home_page_is_server_rendered_with_subscription_form() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .get(&app.address)
        .send()
        .await
        .expect("Failed to request the home page");

    assert!(response.status().is_success());
    let html = response.text().await.unwrap();
    assert!(html.contains("Ideas worth opening your inbox for."));
    assert!(html.contains(r#"action="/subscriptions""#));
    assert!(html.contains(r#"name="email""#));
    assert!(html.contains(r#"/pkg/zero2prod.js"#));
    assert!(html.contains(r#"/pkg/zero2prod.css"#));
}

#[tokio::test]
async fn login_page_contains_an_accessible_form() {
    let app = spawn_app().await;
    let html = app.get_login_html().await;

    assert!(html.contains(r#"action="/login""#));
    assert!(html.contains(r#"autocomplete="username""#));
    assert!(html.contains(r#"autocomplete="current-password""#));
}

#[tokio::test]
async fn authenticated_admin_pages_are_server_rendered() {
    let app = spawn_app().await;
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    let dashboard = app.get_admin_dashboard_html().await;
    let password = app.get_change_password_html().await;
    let newsletters = app.get_newsletters_html().await;

    assert!(dashboard.contains("Editorial overview"));
    assert!(password.contains(r#"action="/admin/password""#));
    assert!(newsletters.contains(r#"action="/admin/newsletters""#));
    assert!(newsletters.contains("<textarea"));
    assert!(newsletters.contains(r#"name="idempotency_key""#));
}
