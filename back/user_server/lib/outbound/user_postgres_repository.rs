use async_trait::async_trait;

use crate::domain::{
    port::user_repository::{UserError, UserRepository},
    r#type::user::User,
};

use super::postgres_repository::PostgresRepository;

#[async_trait]
impl UserRepository for PostgresRepository {
    async fn get_all(&self) -> Result<Vec<User>, UserError> {
        sqlx::query_file_as!(User, "queries/user/get_all.sql")
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| UserError::Uncontroled(e.to_string()))
    }
    async fn insert(&self, user: User) -> Result<(), UserError> {
        sqlx::query_file!(
            "queries/user/insert.sql",
            user.id,
            user.user_name,
            user.email,
            user.first_name,
            user.last_name,
            user.password,
            user.created_at
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| UserError::Uncontroled(e.to_string()))?;
        Ok(())
    }
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        sqlx::query_file_as!(User, "queries/user/get_by.sql", email.to_owned())
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| UserError::Uncontroled(e.to_string()))
    }
}
