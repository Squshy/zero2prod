use crate::helpers::{assert_is_redirect_to, spawn_app};
use uuid::Uuid;

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    let app = spawn_app().await;
    let res = app.get_change_password().await;
    assert_is_redirect_to(&res, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    let res = app
        .post_change_password_body(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password
        }))
        .await;

    assert_is_redirect_to(&res, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let other_new_password = Uuid::new_v4().to_string();

    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    let res = app
        .post_change_password_body(&serde_json::json!({
            "current_password": &app.test_user.password,"new_password": &new_password,
            "new_password_check": &other_new_password
        }))
        .await;
    assert_is_redirect_to(&res, "/admin/password");
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(
        "<p><i>You entered two different new passwords - \
                               the field values must match.</i></p>"
    ));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    let res = app
        .post_change_password_body(&serde_json::json!({
            "current_password": &wrong_password,"new_password": &new_password,
            "new_password_check": &new_password
        }))
        .await;

    assert_is_redirect_to(&res, "/admin/password");

    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>The current password is incorrect.</i></p>"));
}

#[tokio::test]
async fn changing_password_works() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });
    let res = app.post_login(&login_body).await;
    assert_is_redirect_to(&res, "/admin/dashboard");

    let res = app
        .post_change_password_body(&serde_json::json!({
            "current_password": app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password
        }))
        .await;
    assert_is_redirect_to(&res, "/admin/password");

    let html = app.get_change_password_html().await;
    assert!(html.contains("<p><i>Your password has been changed.</i></p>"));

    let res = app.post_logout().await;
    assert_is_redirect_to(&res, "/login");

    let html = app.get_login_html().await;
    assert!(html.contains("<p><i>You have successfully logged out.</i></p>"));

    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &new_password,
    });
    let res = app.post_login(&login_body).await;
    assert_is_redirect_to(&res, "/admin/dashboard");
}
