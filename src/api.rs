use axum::{Json, Router, routing::post, serve};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Debug, Deserialize)]
struct KeywordInput {
    keyword: String,
}

async fn update_keyword(Json(payload): Json<KeywordInput>) -> &'static str {
    println!("受け取ったキーワード: {}", payload.keyword);
    // TODO: config.toml に書き込む処理などをここに入れる
    "OK"
}

pub async fn run_api_server() {
    let app = Router::new().route("/keywords", post(update_keyword));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("APIサーバー起動中 → http://{}", addr);
    serve(listener, app).await.unwrap();
}
