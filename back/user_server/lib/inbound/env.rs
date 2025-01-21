use std::sync::Arc;

use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct Env {
    port: i32,
    host_name: String,
    pub pool: Arc<PgPool>,
    pub secret: String,
}
impl Default for Env {
    fn default() -> Self {
        Self {
            port: 8080,
            host_name: "0.0.0.0".to_string(),
            pool: Arc::new(
                sqlx::PgPool::connect_lazy(
                    "postgres://alexandre:alexandre@0.0.0.0:5432/user_server",
                )
                .unwrap(),
            ),
            secret: "secret".to_string(),
        }
    }
}

impl Env {
    pub fn new(port: i32, host_name: String, database_url: String, secret: String) -> Self {
        Self {
            port,
            host_name,
            pool: sqlx::PgPool::connect_lazy(&database_url).unwrap().into(),
            secret,
        }
    }

    pub fn host(&self) -> String {
        format!("{}:{}", &self.host_name, &self.port)
    }
}
