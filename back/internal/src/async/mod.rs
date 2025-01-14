use async_trait::async_trait;

#[async_trait]
pub trait FromAsync<T> {
    async fn from_async(value: T) -> Self;
}

#[async_trait]
pub trait TryFromAsync<T>: Sized {
    type Error;
    async fn try_from_async(value: T) -> Result<Self, Self::Error>;
}
