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
        }
    }
}

#[async_trait]
pub trait UserRepository {
    async fn insert(&self, user: User) -> Result<(), UserError>;
    async fn get_all(&self) -> Result<Vec<User>, UserError>;
}
