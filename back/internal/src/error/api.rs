use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(Debug, Clone)]
pub enum ApiError {
    InternalServerError(String),
    UnprocessableEntity(String),
    BootError(String),
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        Self::BootError(value.to_string())
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::UnprocessableEntity(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
            _ => todo!(),
        }
    }
}
