use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::infrastructure::http::errors::ApiError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseBody<Payload> {
    Error { success: bool, error: Payload },
    Success { success: bool, payload: Payload },
}

impl<Payload> ResponseBody<Payload> {
    pub fn error(error: Payload) -> Self {
        Self::Error {
            success: false,
            error,
        }
    }

    pub fn success(payload: Payload) -> Self {
        Self::Success {
            success: true,
            payload,
        }
    }
}

pub struct ApiResponse<Body>(StatusCode, ResponseBody<Body>)
where
    Body: Serialize;

impl<Body> IntoResponse for ApiResponse<Body>
where
    Body: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (self.0, Json(self.1)).into_response()
    }
}

impl<Body> ApiResponse<Body>
where
    Body: Serialize,
{
    pub fn new(status_code: StatusCode, body: ResponseBody<Body>) -> Self {
        Self(status_code, body)
    }

    pub fn success(status_code: StatusCode, payload: Body) -> Self {
        Self(status_code, ResponseBody::success(payload))
    }
}

pub type HandlerResponse<Res> = Result<ApiResponse<Res>, ApiError>;
