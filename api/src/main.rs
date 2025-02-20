use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(health_check));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
