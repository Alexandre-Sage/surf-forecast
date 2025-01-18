use std::{collections::HashMap, sync::Arc};

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use internal::r#async::TryFromAsync;
use serde::Serialize;
use serde_json::json;
use sqlx::{migrate, PgPool, Postgres};
use testcontainers::runners::AsyncRunner;
use tower::ServiceExt;
use user_server::{
    domain::r#type::user::{User, UserPayload},
    inbound::{api::Api, env::Env},
};

pub async fn test_env(
    db_name: &str,
) -> (
    testcontainers::ContainerAsync<testcontainers_modules::postgres::Postgres>,
    Api,
    Arc<PgPool>,
) {
    let container = testcontainers_modules::postgres::Postgres::default()
        .with_db_name(db_name)
        .start()
        .await
        .unwrap();
    let database_url = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        container.get_host_port_ipv4(5432).await.unwrap()
    );
    let env = Env::new(8080, "0.0.0.0".to_string(), database_url);
    let pool = env.pool.clone();
    migrate!().run(&*env.pool).await.unwrap();
    let app = Api::try_from_async(env).await.unwrap();
    (container, app, pool)
}

pub async fn post_req<T>(router: Router, url: &str, payload: T) -> axum::http::Response<Body>
where
    T: Serialize,
{
    let payload = json!(payload).to_string();
    let payload = Body::from(payload);
    let request = Request::builder()
        .uri(url)
        .header("content-type", "application/json")
        .method("POST")
        .body(payload)
        .unwrap();
    router.oneshot(request).await.unwrap()
}

async fn seeds(pool: Arc<PgPool>) -> Vec<User> {
    // dirty but only for test so ...
    let users = (0..50)
        .map(|_| UserPayload::fake().try_into().unwrap())
        .map(|u: User| (u.email.clone(), u))
        .collect::<HashMap<_, _>>()
        .into_values()
        .map(|u| (u.user_name.clone(), u))
        .collect::<HashMap<_, _>>()
        .into_values()
        .collect::<Vec<User>>();
    let values: Vec<_> = users
        .iter()
        .map(|user| {
            format!(
                "('{}','{}','{}','{}','{}','{}','{}')",
                user.id,
                user.user_name,
                user.email,
                user.first_name,
                user.last_name,
                user.password,
                user.created_at,
            )
        })
        .collect();
    let values = values.join(",");
    let query = format!(
        "INSERT into users (id, user_name, email, first_name, last_name, password, created_at) values {values};"
        );
    sqlx::query(&query).execute(&*pool).await.unwrap();
    users
}

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
    let seeded = seeds(pool.clone()).await;
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
    let seeded = seeds(pool.clone()).await;
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
