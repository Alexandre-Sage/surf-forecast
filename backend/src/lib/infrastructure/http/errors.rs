use axum::{http::StatusCode, response::IntoResponse};

use crate::domain::port::forecast_provider::ForecastError;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Something went wrong please retry")]
    InternalError(String),
    #[error("{0}")]
    UnprocessableEntity(String),
    #[error("Forbidden")]
    Forbidden,
    #[error("Payment required for external api")]
    PaymentRequired,
}

impl From<ForecastError> for ApiError {
    fn from(value: ForecastError) -> Self {
        log::error!("{}", value);
        match value {
            ForecastError::Cache(err) => Self::InternalError(err.to_string()),
            ForecastError::Unknown(err) => Self::InternalError(err),
            ForecastError::UrlError(err) => Self::InternalError(err.to_string()),
            ForecastError::Forbidden(_) => Self::Forbidden,
            ForecastError::ClientError(err) => Self::InternalError(err.to_string()),
            ForecastError::ApiKeyError(err) => Self::InternalError(err.to_string()),
            ForecastError::PaymentRequired(_) => Self::PaymentRequired,
            ForecastError::UnprocessableEntity(err) => Self::UnprocessableEntity(err),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        use crate::infrastructure::http::response::{ApiResponse, ResponseBody};

        let (status, body) = match self {
            ApiError::InternalError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong please retry".to_string(),
            ),
            ApiError::UnprocessableEntity(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            ApiError::PaymentRequired => (StatusCode::PAYMENT_REQUIRED, self.to_string()),
        };

        let body = ResponseBody::error(body);

        ApiResponse::new(status, body).into_response()
    }
}
