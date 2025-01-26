use chrono::{DateTime, Utc};
use fake::Fake;
use uuid::Uuid;

#[derive(Debug)]
pub struct Spot {
    pub id: Uuid,
    name: String,
    windguru_id: Option<i32>,
    longitude: f32,
    latitude: f32,
    created_at: DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct SpotDto {
    pub(crate) id: Uuid,
    name: String,
    longitude: f32,
    latitude: f32,
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
            created_at: Utc::now(),
        }
    }
    pub fn fake_without_id(id: Uuid) -> Self {
        Self {
            id,
            name: fake::faker::name::fr_fr::Name().fake(),
            longitude: fake::faker::address::fr_fr::Longitude().fake(),
            latitude: fake::faker::address::fr_fr::Latitude().fake(),
            windguru_id: None,
            created_at: Utc::now(),
        }
    }
}
