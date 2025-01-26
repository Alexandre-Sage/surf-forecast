use async_trait::async_trait;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::error::api::ApiError;

#[async_trait]
pub trait Server {
    async fn start(self) -> Result<(), ApiError>;
}

pub trait ServerEnv: Sized + Default {
    fn pool(&self) -> sqlx::PgPool;
    fn from_dotenv() -> dotenvy::Result<Self>;
    fn from_container(db_url: &str) -> Self;
}

pub struct ApiSuccess<T>(StatusCode, T)
where
    T: Serialize;
impl<T> ApiSuccess<T>
where
    T: Serialize,
{
    pub fn new(status: StatusCode, body: T) -> Self {
        Self(status, body)
    }
}
impl<T> IntoResponse for ApiSuccess<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (self.0, Json(self.1)).into_response()
    }
}
