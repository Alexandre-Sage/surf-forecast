use std::sync::Arc;

use axum::{extract::State, routing};
use tower_http::cors::CorsLayer;

use crate::{
    domain::services::api_keys_service::ApiKeyError,
    infrastructure::http::{
        handlers::{storm_glass_handlers, wave_calculation_handlers::wave_calculation_handler},
        state::AppState,
    },
};

use super::state::ServerConfig;
pub struct HttpServer {
    router: axum::Router,
    listener: tokio::net::TcpListener,
}

#[derive(Debug, thiserror::Error)]
pub enum BootError {
    #[error(transparent)]
    Redis(#[from] redis::RedisError),
    #[error(transparent)]
    Sqlite(#[from] sqlx::Error),
    #[error(transparent)]
    ApiKey(#[from] ApiKeyError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl HttpServer {
    fn router(app_state: Arc<AppState>) -> axum::Router {
        axum::Router::new()
            .route(
                "/ping",
                routing::get(|State(_): State<Arc<AppState>>| async { "Up" }),
            )
            .nest(
                "/api",
                axum::Router::new()
                    .route("/calculate/wave", routing::post(wave_calculation_handler))
                    .nest(
                        "/forecast",
                        axum::Router::new().route(
                            "/storm-glass/waves/{lat}/{lng}",
                            routing::get(storm_glass_handlers::storm_glass_forecast_handler),
                        ),
                    ),
            )
            .layer(CorsLayer::permissive())
            .with_state(app_state)
    }
    pub async fn spawn(config: ServerConfig) -> Result<Self, BootError> {
        env_logger::init();

        log::info!("Spawning server");
        log::debug!("{:#?}", config);

        let app_state = AppState::try_from(&config).await?;

        let router = Self::router(app_state.into());
        // axum::Router::new()
        // .route(
        //     "/ping",
        //     routing::get(|State(_): State<Arc<AppState>>| async { "Up" }),
        // )
        // .nest(
        //     "/api",
        //     axum::Router::new()
        //         .route("/calculate/wave", routing::post(wave_calculation_handler))
        //         .nest(
        //             "/forecast",
        //             axum::Router::new().route(
        //                 "/storm-glass/waves/{lat}/{lng}",
        //                 routing::get(storm_glass_handlers::storm_glass_forecast_handler),
        //             ),
        //         ),
        // )
        // .layer(CorsLayer::permissive())
        // .with_state(Arc::new(app_state));

        let listener =
            tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;

        log::info!(
            "Server binded to host: '{}' and port: '{}'",
            config.host,
            config.port
        );

        Ok(Self { router, listener })
    }

    pub async fn boot(self) -> Result<(), std::io::Error> {
        log::info!("Server ready to accept requests");
        axum::serve(self.listener, self.router).await
    }
}
