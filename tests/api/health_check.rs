use crate::helpers::{spawn_app, TestApp};

#[tokio::test]
async fn health_check_works() {
    let TestApp {
        address,
        db_pool: _db_pool,
        ..
    } = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
