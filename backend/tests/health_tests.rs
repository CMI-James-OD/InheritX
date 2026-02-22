use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use inheritx_backend::{create_app, Config};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_health_check() {
    let db = PgPoolOptions::new()
        .connect_lazy("postgres://postgres:password@localhost/test")
        .expect("Failed to create connection pool");

    let config = Config {
        database_url: "postgres://postgres:password@localhost/test".to_string(),
        port: 8080,
        jwt_secret: "secret".to_string(),
    };

    let app = create_app(db, config).await.expect("Failed to create app");

    let mut req = Request::builder()
        .uri("/health")
        .header("X-Forwarded-For", "127.0.0.1")
        .body(Body::empty())
        .unwrap();

    req.extensions_mut().insert(axum::extract::ConnectInfo(
        std::net::SocketAddr::from(([127, 0, 0, 1], 8080)),
    ));

    let response = app
        .oneshot(req)
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body");

    let body_json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body_json["status"], "ok");
    assert_eq!(body_json["message"], "App is healthy");
}
