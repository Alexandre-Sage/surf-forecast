use std::str::FromStr;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::{entities::api_key::ApiKey, port::api_key_repository::ApiKeyRepositoryError};

pub(super) struct ApiKeySelectModel {
    pub id: String,
    pub key: String,
    pub daily_usage: i64,
    pub date: String,
}

impl TryFrom<ApiKeySelectModel> for ApiKey {
    type Error = ApiKeyRepositoryError;
    fn try_from(value: ApiKeySelectModel) -> Result<Self, Self::Error> {
        let id = Uuid::from_str(&value.id)?;
        let date = DateTime::<Utc>::from_str(&value.date)?;

        Ok(ApiKey::new(id, value.key, value.daily_usage as u8, date))
    }
}
