use internal::{api::api::Server, error::api::ApiError, r#async::TryFromAsync};
use spot_server::{
    domain::port::forecast_repository::ForecastRepository,
    inbound::{api::Api, env::Env},
    outbound::storm_glass_repository::StormGlassRepository,
};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let f = StormGlassRepository::new();
    //,
    let x = f.weather_forecast(47.5379194, -2.9781178).await.unwrap();
    //dbg!(&x.hours[0]);
    Ok(())
    //tracing_subscriber::fmt().init();
    //let env = Env::default();
    //let api = Api::try_from_async(env).await?;
    //api.start().await
}
