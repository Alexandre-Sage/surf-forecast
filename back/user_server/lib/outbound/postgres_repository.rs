use std::sync::Arc;

pub struct PostgresRepository {
    pub pool: Arc<sqlx::PgPool>,
}

impl PostgresRepository {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        Self { pool }
    }
}
