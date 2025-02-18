use internal::{
    api::api::Server, cache::redis::RedisCache, error::api::ApiError, r#async::TryFromAsync,
    serializer::json::JsonSerializer,
};
use spot_server::{
    domain::port::forecast_repository::ForecastRepository,
    inbound::{api::Api, env::Env},
    outbound::storm_glass_repository::StormGlassRepository,
};
const REDIS_URL: &str = "redis://localhost:6379";

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let c = redis::Client::open(REDIS_URL).unwrap();
    let cache = RedisCache::new(c, JsonSerializer);
    let f = StormGlassRepository::new(tokio::sync::RwLock::from(cache).into());
    let x = f.weather_forecast(47.5379194, -2.9781178).await.unwrap();
    //dbg!(&x.hours[0]);
    Ok(())
    //tracing_subscriber::fmt().init();
    //let env = Env::default();
    //let api = Api::try_from_async(env).await?;
    //api.start().await
}
