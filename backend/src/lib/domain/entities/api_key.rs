use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub(crate) struct ApiKey {
    pub id: Uuid,
    key: String,
    daily_usage: u8,
    date: DateTime<Utc>,
}

impl ApiKey {
    pub(crate) fn new(id: Uuid, key: String, daily_usage: u8, date: DateTime<Utc>) -> Self {
        Self {
            id,
            key,
            daily_usage,
            date,
        }
    }

    pub(crate) fn daily_usage(&self) -> u8 {
        self.daily_usage
    }

    pub(crate) fn increment_usage(&mut self) {
        self.daily_usage += 1
    }

    pub(crate) fn key(&self) -> &str {
        &self.key
    }

    pub(crate) fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub(crate) fn reset(&mut self) {
        self.date = Utc::now();
        self.daily_usage = 0
    }
}
