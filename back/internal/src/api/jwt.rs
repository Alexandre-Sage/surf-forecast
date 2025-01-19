use std::time::UNIX_EPOCH;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims<T> {
    pub data: T,
    pub exp: usize,
}

impl<T> Claims<T> {
    pub fn new(data: T) -> Self {
        let exp = std::time::Duration::from_secs(60 * 60 * 2);
        let exp = std::time::SystemTime::now()
            .checked_add(exp)
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        Self { data, exp }
    }
}

pub fn encode_jwt<T, S>(data: T, secret: S) -> Result<String, jsonwebtoken::errors::Error>
where
    T: Serialize,
    S: AsRef<[u8]>,
{
    jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(data),
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode_jwt<T, S>(
    token: &str,
    key: S,
) -> Result<jsonwebtoken::TokenData<T>, jsonwebtoken::errors::Error>
where
    T: for<'a> Deserialize<'a>,
    S: AsRef<[u8]>,
{
    jsonwebtoken::decode::<T>(
        token,
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::default(),
    )
}
