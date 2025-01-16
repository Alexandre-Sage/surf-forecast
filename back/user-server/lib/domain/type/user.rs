use argon2::{
    password_hash::{
        self, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use fake::Fake;
use internal::error::api::ApiError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::port::user_repository::UserError;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPayload {
    user_name: String,
    email: String,
    first_name: String,
    last_name: String,
    password: String,
    confirm_password: String,
}

impl UserPayload {
    pub(crate) fn validate_password(&self) -> Result<(), UserError> {
        if self.password != self.confirm_password {
            return Err(UserError::PasswordMismatch);
        }
        if self.password.len() < 8 {
            return Err(UserError::PasswordLength);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub(crate) id: Uuid,
    pub(crate) user_name: String,
    pub(crate) email: String,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) password: String,
    pub(crate) created_at: NaiveDateTime,
}

impl User {
    pub(crate) fn hash_password(self) -> Result<Self, ApiError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(self.password.as_bytes(), &salt)
            .map_err(|err| ApiError::InternalServerError(err.to_string()))?
            .to_string();
        Ok(Self {
            password: password_hash,
            id: self.id,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            user_name: self.user_name,
            created_at: self.created_at,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    id: Uuid,
    user_name: String,
    email: String,
    first_name: String,
    last_name: String,
    created_at: DateTime<Utc>,
}

impl TryFrom<UserPayload> for User {
    type Error = UserError;
    fn try_from(value: UserPayload) -> Result<Self, Self::Error> {
        value.validate_password()?;
        Ok(Self {
            id: Uuid::new_v4(),
            user_name: value.user_name,
            first_name: value.first_name,
            email: value.email,
            last_name: value.last_name,
            created_at: Utc::now().naive_utc(),
            password: value.password,
        })
    }
}

impl From<User> for UserDto {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            created_at: value.created_at.and_utc(),
            user_name: value.user_name,
        }
    }
}

//#[cfg(test)]
impl UserPayload {
    pub fn fake() -> Self {
        let pass: String = fake::faker::internet::fr_fr::Password(8..10).fake();
        UserPayload {
            last_name: fake::faker::name::fr_fr::LastName().fake(),
            first_name: fake::faker::name::fr_fr::FirstName().fake(),
            user_name: fake::faker::name::fr_fr::Name().fake(),
            email: fake::faker::internet::fr_fr::SafeEmail().fake(),
            password: pass.clone(),
            confirm_password: pass.clone(),
        }
    }
    pub fn fake_without_mail(email: String) -> Self {
        let pass: String = fake::faker::internet::fr_fr::Password(8..10).fake();
        UserPayload {
            last_name: fake::faker::name::fr_fr::LastName().fake(),
            first_name: fake::faker::name::fr_fr::FirstName().fake(),
            user_name: fake::faker::name::fr_fr::Name().fake(),
            email,
            password: pass.clone(),
            confirm_password: pass.clone(),
        }
    }
    pub fn fake_without_user_name(user_name: String) -> Self {
        let pass: String = fake::faker::internet::fr_fr::Password(8..10).fake();
        UserPayload {
            last_name: fake::faker::name::fr_fr::LastName().fake(),
            first_name: fake::faker::name::fr_fr::FirstName().fake(),
            user_name,
            email: fake::faker::internet::fr_fr::SafeEmail().fake(),
            password: pass.clone(),
            confirm_password: pass.clone(),
        }
    }
}
#[cfg(test)]
mod test {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    use chrono::Utc;
    use fake::{
        faker::{
            self,
            internet::{en::SafeEmail, fr_fr::Password},
        },
        Fake,
    };
    use uuid::Uuid;

    use crate::domain::port::user_repository::UserError;

    use super::{User, UserPayload};
    use faker::name::fr_fr;
    #[test]
    fn try_from_user_payload_ok() {
        let payload = UserPayload::fake();
        let result = User::try_from(payload.clone());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.first_name, payload.first_name);
        assert_eq!(result.last_name, payload.last_name);
        assert_eq!(result.password, payload.password);
        assert_eq!(result.email, payload.email);
    }
    #[test]
    fn password_mismatch() {
        let pass: String = Password(8..10).fake();
        let payload = UserPayload {
            last_name: fr_fr::LastName().fake(),
            first_name: fr_fr::FirstName().fake(),
            user_name: fr_fr::Name().fake(),
            email: SafeEmail().fake(),
            password: pass.clone(),
            confirm_password: "".to_string(),
        };
        let result = User::try_from(payload).unwrap_err();
        assert_eq!(result, UserError::PasswordMismatch)
    }
    #[test]
    fn password_to_short() {
        let pass: String = Password(0..7).fake();
        let payload = UserPayload {
            last_name: fr_fr::LastName().fake(),
            first_name: fr_fr::FirstName().fake(),
            user_name: fr_fr::Name().fake(),
            email: SafeEmail().fake(),
            password: pass.clone(),
            confirm_password: pass.clone(),
        };
        let result = User::try_from(payload).unwrap_err();
        assert_eq!(result, UserError::PasswordLength)
    }
    #[test]
    fn should_hash_password() {
        let user = User {
            id: Uuid::new_v4(),
            last_name: fr_fr::LastName().fake(),
            first_name: fr_fr::FirstName().fake(),
            user_name: fr_fr::Name().fake(),
            email: SafeEmail().fake(),
            password: Password(0..7).fake(),
            created_at: Utc::now().naive_utc(),
        };
        let x = user.clone().hash_password().unwrap();
        let binding = x.password;
        let parsed_hash = PasswordHash::new(&binding).unwrap();
        assert!(Argon2::default()
            .verify_password(user.password.as_bytes(), &parsed_hash)
            .is_ok());
    }
}
