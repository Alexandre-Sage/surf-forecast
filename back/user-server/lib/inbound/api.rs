use std::net::SocketAddr;

use async_trait::async_trait;
use axum::routing::get;
use internal::{error::api::ApiError, r#async::TryFromAsync};

use super::env::Env;

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
        let router = axum::Router::new()
            .route("/ping", get(|| async { "PONG" }))
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
