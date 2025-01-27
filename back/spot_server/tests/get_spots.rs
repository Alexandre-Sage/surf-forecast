use axum::http::StatusCode;
use spot_server::{
    domain::r#type::spot::{Spot, SpotDto},
    inbound::{api::Api, env::Env},
};
use test_lib::{get_req, parse_json_response, test_env};

#[tokio::test]
async fn should_get_all_spots() {
    let (container, app, _) = test_env::<Api, Env>("spots", "migrations").await;
    let res = get_req(app.router, "/spots").await;
    let status = res.status();
    let body = parse_json_response::<Vec<SpotDto>>(res).await;
    assert!(body.is_ok());
    assert_eq!(status, StatusCode::OK);
    container.stop().await.unwrap();
}

#[tokio::test]
async fn should_get_spot_by_id() {
    let (container, app, pool) = test_env::<Api, Env>("spots", "migrations").await;
    let db_spot = sqlx::query_as!(Spot, "select * from spots;")
        .fetch_one(&pool)
        .await
        .unwrap();
    let res = get_req(app.router, &format!("/spots/{}", db_spot.id)).await;
    let status = res.status();
    let body = parse_json_response::<SpotDto>(res).await;
    assert!(body.is_ok());
    assert_eq!(status, StatusCode::OK);
    container.stop().await.unwrap();
}
