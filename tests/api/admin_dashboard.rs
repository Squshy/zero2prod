use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn you_must_be_logged_in_to_access_the_admin_dashboard() {
    let app = spawn_app().await;
    let res = app.get_admin_dashboard().await;
    assert_is_redirect_to(&res, "/login");
}

#[tokio::test]
async fn logout_clears_session_state() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });
    let res = app.post_login(&login_body).await;
    assert_is_redirect_to(&res, "/admin/dashboard");

    let html = app.get_admin_dashboard_html().await;
    assert!(html.contains(&format!("Welcome {}", app.test_user.username)));

    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    let html = app.get_login_html().await;
    assert!(html.contains("<p><i>You have successfully logged out.</i></p>"));

    let res = app.get_admin_dashboard().await;
    assert_is_redirect_to(&res, "/login")
}
