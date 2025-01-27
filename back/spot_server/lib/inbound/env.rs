use internal::api::api::ServerEnv;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Env {
    port: i32,
    host_name: String,
    pool: Arc<PgPool>,
    pub secret: String,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            port: 8080,
            host_name: "0.0.0.0".to_string(),
            pool: Arc::new(
                sqlx::PgPool::connect_lazy(
                    "postgres://alexandre:alexandre@0.0.0.0:5432/spot_server",
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

impl ServerEnv for Env {
    fn pool(&self) -> sqlx::PgPool {
        (*self.pool).clone()
    }
    fn from_dotenv() -> dotenvy::Result<Self> {
        let pool = sqlx::PgPool::connect_lazy(&dotenvy::var("DATABASE_URL")?)
            .expect("Cant connect to pg instance");
        Ok(Self {
            host_name: dotenvy::var("HOST").unwrap_or("localhost".to_string()),
            port: dotenvy::var("PORT")?.parse().unwrap_or(8080),
            pool: pool.into(),
            secret: dotenvy::var("SECRET")?,
        })
    }
    fn from_container(db_url: &str) -> Self {
        let pool = sqlx::PgPool::connect_lazy(db_url).expect("Cant connect to pg instance");
        Self {
            pool: pool.into(),
            ..Default::default()
        }
    }
}
