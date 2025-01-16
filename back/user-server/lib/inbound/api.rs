use std::{net::SocketAddr, sync::Arc};

use async_trait::async_trait;
use axum::routing::{get, post};
use internal::{error::api::ApiError, r#async::TryFromAsync};

use crate::{
    domain::{port::user_repository::UserRepository, service::user_service::UserService},
    outbound::postgres_repository::PostgresRepository,
};

use super::{env::Env, handlers::user::create_user};

pub struct Api {
    router: axum::Router,
    listener: tokio::net::TcpListener,
}

pub enum ApiBootError {
    HostBinding(String),
}

impl From<ApiBootError> for ApiError {
    fn from(value: ApiBootError) -> Self {
        match value {
            ApiBootError::HostBinding(err) => Self::InternalServerError(err),
        }
    }
}

pub struct ApiState<U>
where
    U: UserRepository,
{
    pub user_service: UserService<U>,
}

#[async_trait]
impl TryFromAsync<Env> for Api {
    type Error = ApiBootError;
    async fn try_from_async(env: Env) -> Result<Self, ApiBootError> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http()
        .make_span_with(|req: &axum::extract::Request<_>| {
            let ip = req.extensions().get::<SocketAddr>().map(|ip|
            ip.to_string()).unwrap_or("No ip associated to request".to_string());
            tracing::info_span!("new_request",method = ?req.method(), uri= req.uri().to_string(), from = ip)
        });
        let compression_layer = tower_http::compression::CompressionLayer::new();

        let pool = sqlx::PgPool::connect_lazy(&env.database_url).unwrap();
        let pool = Arc::new(pool);
        let user_repo = PostgresRepository::new(pool.clone());
        let user_service = UserService::new(user_repo);
        let app_state = ApiState { user_service };
        let app_state = Arc::new(app_state);
        let router = axum::Router::new()
            .route("/ping", get(|| async { "PONG" }))
            .route("/user", post(create_user))
            .with_state(app_state)
            .layer(trace_layer)
            .layer(compression_layer);
        tokio::net::TcpListener::bind(env.host())
            .await
            .map_err(|e| Self::Error::HostBinding(e.to_string()))
            .map(|listener| Self { listener, router })
    }
}

impl Api {
    pub async fn start(self) -> Result<(), ApiError> {
        axum::serve(self.listener, self.router)
            .await
            .map_err(|e| ApiError::BootError(e.to_string()))
    }
}
