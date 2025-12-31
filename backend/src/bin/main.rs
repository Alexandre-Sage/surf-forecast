use forecast_api::infrastructure::http::{server::HttpServer, state::ServerConfig};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = envy::from_env::<ServerConfig>().unwrap();
    let server = HttpServer::spawn(config).await.unwrap();
    server.boot().await.unwrap();
}
