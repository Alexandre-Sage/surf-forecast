use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

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
