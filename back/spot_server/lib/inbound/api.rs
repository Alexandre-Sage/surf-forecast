use std::{net::SocketAddr, sync::Arc};

use async_trait::async_trait;
use axum::routing::get;
use internal::{
    api::api::{Server, ServerEnv},
    error::api::ApiError,
    r#async::TryFromAsync,
};

use crate::{
    domain::{port::spot_repository::SpotRepository, service::spot_service::SpotService},
    outbound::postgres_repository::PostgresRepository,
};

use super::{
    env::Env,
    handlers::spot::{get_all_spots, get_spot},
};

pub struct Api {
    pub router: axum::Router,
    listener: tokio::net::TcpListener,
}

#[derive(Debug)]
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
    U: SpotRepository,
{
    pub spot_service: SpotService<U>,
    pub secret: String,
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
        let cors = tower_http::cors::CorsLayer::permissive();
        let spot_repo = PostgresRepository::new(env.pool());
        let spot_service = SpotService::new(spot_repo);
        let app_state = ApiState {
            spot_service,
            secret: env.secret.clone(),
        };
        let app_state = Arc::new(app_state);
        let router = axum::Router::new()
            .route("/ping", get(|| async { "PONG" }))
            .route("/spots", get(get_all_spots))
            .route("/spots/{id}", get(get_spot))
            .with_state(app_state)
            .layer(trace_layer)
            .layer(compression_layer)
            .layer(cors);
        tokio::net::TcpListener::bind(env.host())
            .await
            .map_err(|e| Self::Error::HostBinding(e.to_string()))
            .map(|listener| Self { listener, router })
    }
}

#[async_trait]
impl Server for Api {
    async fn start(self) -> Result<(), ApiError> {
        axum::serve(self.listener, self.router)
            .await
            .map_err(|e| ApiError::BootError(e.to_string()))
    }
}
