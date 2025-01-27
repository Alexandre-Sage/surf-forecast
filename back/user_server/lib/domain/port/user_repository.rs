use async_trait::async_trait;
use internal::error::api::ApiError;

use crate::domain::r#type::user::User;

#[derive(Debug, PartialEq, Eq)]
pub enum UserError {
    Uncontroled(String),
    PasswordMismatch,
    PasswordLength,
    UserNameTaken,
    EmailExist,
    AuthError,
}

impl From<UserError> for ApiError {
    fn from(error: UserError) -> Self {
        match error {
            UserError::Uncontroled(err) => Self::InternalServerError(err),
            UserError::PasswordMismatch => {
                Self::UnprocessableEntity("PASSWORD_MISMATCH".to_string())
            }
            UserError::PasswordLength => Self::UnprocessableEntity("PASSWORD_LENGTH".to_string()),
            UserError::EmailExist => Self::UnprocessableEntity("EMAIL_EXIST".to_string()),
            UserError::UserNameTaken => Self::UnprocessableEntity("USERNAME_TAKEN".to_string()),
            UserError::AuthError => Self::Unauthorized("AUTHENTICATION_FAILED".to_string()),
        }
    }
}

#[async_trait]
pub trait UserRepository {
    async fn insert(&self, user: User) -> Result<(), UserError>;
    async fn all(&self) -> Result<Vec<User>, UserError>;
    async fn by_email(&self, email: &str) -> Result<Option<User>, UserError>;
}
