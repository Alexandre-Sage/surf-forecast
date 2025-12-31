use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinate(f32, f32);

impl Coordinate {
    pub fn new(latitude: f32, longitude: f32) -> Self {
        Self(latitude, longitude)
    }

    pub fn latitude(&self) -> f32 {
        self.0
    }

    pub fn longitude(&self) -> f32 {
        self.1
    }
}
