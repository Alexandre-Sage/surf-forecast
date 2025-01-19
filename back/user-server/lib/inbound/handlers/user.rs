use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use internal::{api::api_success::ApiSuccess, error::api::ApiError};

use crate::{
    domain::{
        port::user_repository::UserRepository,
        r#type::user::{LoginPayload, UserDto, UserPayload},
    },
    inbound::api::ApiState,
};

pub async fn create_user<U: UserRepository>(
    State(state): State<Arc<ApiState<U>>>,
    Json(payload): Json<UserPayload>,
) -> Result<ApiSuccess<()>, ApiError> {
    state.user_service.insert(payload).await?;
    Ok(ApiSuccess::new(StatusCode::CREATED, ()))
}

pub async fn authenticate_user<U: UserRepository>(
    State(state): State<Arc<ApiState<U>>>,
    Json(payload): Json<LoginPayload>,
) -> Result<ApiSuccess<(UserDto, String)>, ApiError> {
    let user_dto = state
        .user_service
        .authenticate(payload.email.as_str(), payload.password.as_str())
        .await?;
    Ok(ApiSuccess::new(StatusCode::OK, user_dto))
}
