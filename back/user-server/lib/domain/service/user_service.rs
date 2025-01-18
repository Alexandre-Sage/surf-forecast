use crate::domain::{
    port::user_repository::{UserError, UserRepository},
    r#type::user::{User, UserPayload},
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
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use crate::domain::{
        port::user_repository::{UserError, UserRepository},
        r#type::user::{User, UserPayload},
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
        let service = UserService::new(repo.clone());
        let payload = UserPayload::fake_without_mail("hello@world.com".to_string());
        let result = service.insert(payload).await.unwrap_err();
        assert_eq!(result, UserError::EmailExist)
    }
    #[tokio::test]
    async fn should_throw_username_error() {
        let repo = FakeUserRepo;
        let service = UserService::new(repo.clone());
        let payload = UserPayload::fake_without_user_name("helloworld".to_string());
        let result = service.insert(payload).await.unwrap_err();
        assert_eq!(result, UserError::UserNameTaken)
    }
}
