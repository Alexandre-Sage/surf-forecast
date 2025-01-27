use chrono::{DateTime, NaiveDateTime, Utc};
use fake::Fake;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Spot {
    pub id: Uuid,
    pub name: String,
    pub windguru_id: Option<i32>,
    pub longitude: f64,
    pub latitude: f64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpotDto {
    pub(crate) id: Uuid,
    name: String,
    longitude: f64,
    latitude: f64,
}

impl From<Spot> for SpotDto {
    fn from(value: Spot) -> Self {
        Self {
            latitude: value.latitude,
            longitude: value.longitude,
            name: value.name,
            id: value.id,
        }
    }
}
impl Spot {
    pub fn fake() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: fake::faker::name::fr_fr::Name().fake(),
            longitude: fake::faker::address::fr_fr::Longitude().fake(),
            latitude: fake::faker::address::fr_fr::Latitude().fake(),
            windguru_id: None,
            created_at: Utc::now().naive_utc(),
        }
    }
    pub fn fake_without_id(id: Uuid) -> Self {
        Self {
            id,
            name: fake::faker::name::fr_fr::Name().fake(),
            longitude: fake::faker::address::fr_fr::Longitude().fake(),
            latitude: fake::faker::address::fr_fr::Latitude().fake(),
            windguru_id: None,
            created_at: Utc::now().naive_utc(),
        }
    }
}
