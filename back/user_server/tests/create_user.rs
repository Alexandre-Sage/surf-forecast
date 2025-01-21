use axum::http::StatusCode;
use common::{post_req, test_env, users_seeds};
use user_server::domain::r#type::user::UserPayload;
mod common;

#[tokio::test]
async fn should_create_user() {
    let (container, app, pool) = test_env("users").await;
    let payload = UserPayload::fake();
    let response = post_req(app.router, "/users", payload).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    let db_res = sqlx::query("SELECT * FROM users;")
        .fetch_all(&*pool)
        .await
        .unwrap();
    assert_eq!(db_res.len(), 1);
    container.stop().await.unwrap();
}
#[tokio::test]
async fn should_send_422_for_dup_email() {
    let (container, app, pool) = test_env("users").await;
    let seeded = users_seeds(pool.clone()).await;
    let payload = UserPayload::fake_without_mail(seeded[0].email.clone());
    let response = post_req(app.router, "/users", payload).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let db_res = sqlx::query("SELECT * FROM users;")
        .fetch_all(&*pool)
        .await
        .unwrap();
    assert_eq!(db_res.len(), seeded.len());
    container.stop().await.unwrap();
}

#[tokio::test]
async fn should_send_422_for_dup_username() {
    let (container, app, pool) = test_env("users").await;
    let seeded = users_seeds(pool.clone()).await;
    let payload = UserPayload::fake_without_user_name(seeded[0].user_name.clone());
    let response = post_req(app.router, "/users", payload).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let db_res = sqlx::query("SELECT * FROM users;")
        .fetch_all(&*pool)
        .await
        .unwrap();
    assert_eq!(db_res.len(), seeded.len());
    container.stop().await.unwrap();
}
