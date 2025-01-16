use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use internal::r#async::TryFromAsync;
use serde_json::json;
use sqlx::migrate;
use testcontainers::runners::AsyncRunner;
use tower::ServiceExt;
use user_server::{
    domain::r#type::user::UserPayload,
    inbound::{api::Api, env::Env},
};

#[tokio::test]
async fn should_create_user() {
    let container = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .unwrap();
    let database_url = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        container.get_host_port_ipv4(5432).await.unwrap()
    );
    let env = Env::new(8080, "0.0.0.0".to_string(), database_url);
    migrate!().run(&*env.pool).await.unwrap();
    let app = Api::try_from_async(env).await.unwrap();
    let payload = json!(UserPayload::fake()).to_string();
    let payload = Body::from(payload);
    let request = Request::builder()
        .uri("/users")
        .header("content-type", "application/json")
        .method("POST")
        .body(payload)
        .unwrap();
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED)
}
