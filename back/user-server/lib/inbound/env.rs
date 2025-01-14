#[derive(Debug)]
pub struct Env {
    port: i32,
    host_name: String,
}
impl Default for Env {
    fn default() -> Self {
        Self {
            port: 8080,
            host_name: "0.0.0.0".to_string(),
        }
    }
}

impl Env {
    pub fn host(&self) -> String {
        format!("{}:{}", &self.host_name, &self.port)
    }
}
