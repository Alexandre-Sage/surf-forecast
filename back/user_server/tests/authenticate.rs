use axum::http::StatusCode;
use common::users_seeds;
use http_body_util::BodyExt;
use test_lib::{post_req, test_env};
use user_server::{
    domain::r#type::user::{LoginPayload, UserDto},
    inbound::{api::Api, env::Env},
};
mod common;

#[tokio::test]
async fn should_authenticate_user() {
    let (container, app, pool) = test_env::<Api, Env>("users", "migrations").await;
    let _ = users_seeds(pool.clone()).await;
    let payload = LoginPayload {
        email: "hello@world.com".to_string(),
        password: "helloworld".to_string(),
    };
    let response = post_req(app.router, "/users/authenticate", payload).await;
    assert_eq!(response.status(), StatusCode::OK);
    let body = serde_json::from_slice::<(UserDto, String)>(
        &response.into_body().collect().await.unwrap().to_bytes()[..],
    );
    assert!(body.is_ok());
    container.stop().await.unwrap();
}

#[tokio::test]
async fn should_throw_401_for_unknown_email() {
    let (container, app, pool) = test_env::<Api, Env>("users", "migrations").await;
    let _ = users_seeds(pool.clone()).await;
    let payload = LoginPayload {
        email: "he@cyz.com".to_string(),
        password: "helloworld".to_string(),
    };
    let response = post_req(app.router, "/users/authenticate", payload).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    container.stop().await.unwrap();
}

#[tokio::test]
async fn should_throw_401_for_wrong_pwd() {
    let (container, app, pool) = test_env::<Api, Env>("users", "migrations").await;
    let _ = users_seeds(pool.clone()).await;
    let payload = LoginPayload {
        email: "hello@world.com".to_string(),
        password: "xyz".to_string(),
    };
    let response = post_req(app.router, "/users/authenticate", payload).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    container.stop().await.unwrap();
}
