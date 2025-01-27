use uuid::Uuid;

use crate::domain::{
    port::spot_repository::{SpotError, SpotRepository},
    r#type::spot::SpotDto,
};

#[derive(Debug)]
pub struct SpotService<T>
where
    T: SpotRepository,
{
    repository: T,
}

impl<T> SpotService<T>
where
    T: SpotRepository,
{
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<SpotDto>, SpotError> {
        self.repository
            .all()
            .await
            .map(|spots| spots.into_iter().map(SpotDto::from).collect())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<SpotDto, SpotError> {
        let spot = self.repository.by_id(id).await?;
        match spot {
            Some(spot) => Ok(spot.into()),
            None => Err(SpotError::NotFound),
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use async_trait::async_trait;
    use uuid::Uuid;

    use crate::domain::{
        port::spot_repository::{SpotError, SpotRepository},
        r#type::spot::Spot,
    };

    use super::SpotService;

    struct FakeRepo;
    #[async_trait]
    impl SpotRepository for FakeRepo {
        async fn all(&self) -> Result<Vec<Spot>, SpotError> {
            Ok([0..50].iter().map(|_| Spot::fake()).collect())
        }
        async fn by_id(&self, id: Uuid) -> Result<Option<Spot>, SpotError> {
            let spot = Spot::fake_without_id(
                Uuid::from_str("c8361d79-3bb4-4ef6-a781-95a9be466841").unwrap(),
            );
            if id == spot.id {
                Ok(Some(spot))
            } else {
                Ok(None)
            }
        }
    }

    #[tokio::test]
    async fn get_all() {
        let repo = FakeRepo;
        let spots = repo.all().await.unwrap();
        let service = SpotService::new(repo);
        let result = service.get_all().await.unwrap();
        assert_eq!(result.len(), spots.len())
    }

    #[tokio::test]
    async fn get_by_id() {
        let id = Uuid::from_str("c8361d79-3bb4-4ef6-a781-95a9be466841").unwrap();
        let service = SpotService::new(FakeRepo);
        let result = service.get_by_id(id).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn should_return_unknwo_spot_error() {
        let service = SpotService::new(FakeRepo);
        let result = service.get_by_id(Uuid::new_v4()).await.unwrap_err();
        assert_eq!(result, SpotError::NotFound)
    }
}
