use std::{fmt::Debug, path::Path};

use axum::{body::Body, http::Request, Router};
use http_body_util::BodyExt;
use internal::api::api::Server;
use internal::{api::api::ServerEnv, r#async::TryFromAsync};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{migrate, PgPool};
use testcontainers::runners::AsyncRunner;
use tower::ServiceExt;

pub async fn test_env<S, E>(
    db_name: &str,
    miggration_path: &str,
) -> (
    testcontainers::ContainerAsync<testcontainers_modules::postgres::Postgres>,
    S,
    PgPool,
)
where
    S: Server + TryFromAsync<E>,
    E: ServerEnv,
    <S as TryFromAsync<E>>::Error: std::fmt::Debug,
{
    let container = testcontainers_modules::postgres::Postgres::default()
        .with_db_name(db_name)
        .start()
        .await
        .unwrap();
    let database_url = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        container.get_host_port_ipv4(5432).await.unwrap()
    );
    let env = E::from_container(&database_url);
    let pool = env.pool();
    sqlx::query("CREATE EXTENSION IF NOT EXISTS pgcrypto;")
        .execute(&pool)
        .await
        .unwrap();
    migrate::Migrator::new(Path::new(miggration_path))
        .await
        .unwrap()
        .run(&env.pool())
        .await
        .unwrap();
    let app = S::try_from_async(env).await.expect("");
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

pub async fn get_req(router: Router, url: &str) -> axum::http::Response<Body> {
    let request = Request::builder()
        .uri(url)
        .header("content-type", "application/json")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    router.oneshot(request).await.unwrap()
}

pub async fn parse_json_response<T>(
    response: axum::http::Response<Body>,
) -> Result<T, serde_json::Error>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_slice::<T>(&response.into_body().collect().await.unwrap().to_bytes()[..])
}
