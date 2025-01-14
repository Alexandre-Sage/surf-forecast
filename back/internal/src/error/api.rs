#[derive(Debug, Clone)]
pub enum ApiError {
    InternalServerError(String),
    BootError(String),
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        Self::BootError(value.to_string())
    }
}
