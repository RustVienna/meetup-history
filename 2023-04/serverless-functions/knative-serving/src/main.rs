use axum::{http::StatusCode, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|| async {
            match reqwest::get("https://www.rust-lang.org").await {
                Ok(res) => (StatusCode::OK, res.text().await.unwrap()),
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Server failed with {}", err),
                ),
            }
        }),
    );
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
