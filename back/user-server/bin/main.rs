use internal::{error::api::ApiError, r#async::TryFromAsync};
use user_server::inbound::{api::Api, env::Env};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    tracing_subscriber::fmt().init();
    let env = Env::default();
    let api = Api::try_from_async(env).await?;
    api.start().await
}
