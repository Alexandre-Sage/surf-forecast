use internal::{
    api::jwt::{encode_jwt, Claims},
    crypto::verify_hash,
};

use crate::domain::{
    port::user_repository::{UserError, UserRepository},
    r#type::user::{User, UserDto, UserPayload},
};

pub struct UserService<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> UserService<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub async fn insert(&self, payload: UserPayload) -> Result<(), UserError> {
        let user = User::try_from(payload)?;
        let users = self.repository.get_all().await?;
        let email_exist = users.iter().find(|u| u.email == user.email);
        let user_name_exist = users.iter().find(|u| u.user_name == user.user_name);
        if let Some(_) = email_exist {
            return Err(UserError::EmailExist);
        }
        if let Some(_) = user_name_exist {
            return Err(UserError::UserNameTaken);
        }
        self.repository.insert(user).await
    }

    pub async fn authenticate(
        &self,
        email: &str,
        password: &str,
        secret: &str,
    ) -> Result<(UserDto, String), UserError> {
        let user = self.repository.get_by_email(email).await?;
        match user {
            Some(user) => {
                let hash = user.password.as_str();
                if verify_hash(hash, password) {
                    let user_dto = UserDto::from(user); //user.into();
                    let token = encode_jwt(&user_dto, secret)
                        .map_err(|e| UserError::Uncontroled(e.to_string()))?;
                    Ok((user_dto, token))
                } else {
                    Err(UserError::AuthError)
                }
            }
            None => Err(UserError::AuthError),
        }
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use internal::api::jwt::{decode_jwt, Claims};

    use crate::domain::{
        port::user_repository::{UserError, UserRepository},
        r#type::user::{User, UserDto, UserPayload},
    };

    use super::UserService;

    #[derive(Clone)]
    pub struct FakeUserRepo;
    #[async_trait]
    impl UserRepository for FakeUserRepo {
        async fn insert(&self, _user: User) -> Result<(), UserError> {
            Ok(())
        }
        async fn get_all(&self) -> Result<Vec<User>, UserError> {
            let mut users: Vec<User> = [0..50]
                .into_iter()
                .map(|_| UserPayload::fake().try_into().unwrap())
                .collect();
            users.push(
                UserPayload::fake_without_mail("hello@world.com".to_string())
                    .try_into()
                    .unwrap(),
            );
            users.push(
                UserPayload::fake_without_user_name("helloworld".to_string())
                    .try_into()
                    .unwrap(),
            );
            Ok(users)
        }
        async fn get_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
            let user: User =
                UserPayload::fake_without_mail_and_pass("hello@world.com", "helloworld")
                    .try_into()
                    .unwrap();
            if user.email == email {
                Ok(Some(user))
            } else {
                Ok(None)
            }
        }
    }
    #[tokio::test]
    async fn should_create_user() {
        let repo = FakeUserRepo;
        let service = UserService::new(repo);
        let payload = UserPayload::fake();
        let result = service.insert(payload).await;
        assert!(result.is_ok())
    }
    #[tokio::test]
    async fn should_throw_email_error() {
        let repo = FakeUserRepo;
        let service = UserService::new(repo);
        let payload = UserPayload::fake_without_mail("hello@world.com".to_string());
        let result = service.insert(payload).await.unwrap_err();
        assert_eq!(result, UserError::EmailExist)
    }
    #[tokio::test]
    async fn should_throw_username_error() {
        let repo = FakeUserRepo;
        let service = UserService::new(repo);
        let payload = UserPayload::fake_without_user_name("helloworld".to_string());
        let result = service.insert(payload).await.unwrap_err();
        assert_eq!(result, UserError::UserNameTaken)
    }
    #[tokio::test]
    async fn should_authenticate_user() {
        let repo = FakeUserRepo;
        let service = UserService::new(repo);
        let email = "hello@world.com";
        let pass = "helloworld";
        let result = service.authenticate(email, pass, "secret").await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0.email, email.to_string());
        assert!(decode_jwt::<Claims<UserDto>, _>(result.1.as_str(), "secret").is_ok());
        //assert!()
    }
    #[tokio::test]
    async fn should_failed_for_unknown_mail() {
        let repo = FakeUserRepo;
        let service = UserService::new(repo);
        let email = "heo@world.com";
        let result = service
            .authenticate(email, "helloworld", "secret")
            .await
            .unwrap_err();
        assert_eq!(result, UserError::AuthError)
    }
    #[tokio::test]
    async fn should_failed_for_password_mismatch() {
        let repo = FakeUserRepo;
        let service = UserService::new(repo);
        let email = "hello@world.com";
        let result = service
            .authenticate(email, "xyz", "secret")
            .await
            .unwrap_err();
        assert_eq!(result, UserError::AuthError)
    }
}
