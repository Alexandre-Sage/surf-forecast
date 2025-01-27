pub struct HttpRepository {
    base_url: String,
    token: String,
}

impl HttpRepository {
    pub fn new(base_url: String, token: String) -> Self {
        Self { base_url, token }
    }
}
