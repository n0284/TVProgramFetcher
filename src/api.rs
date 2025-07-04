use axum::{Json, Router, routing::post, serve};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};

#[derive(Debug, Deserialize)]
struct KeywordInput {
    keyword: String,
}

async fn update_keyword(Json(payload): Json<KeywordInput>) -> &'static str {
    println!("受信成功！");
    println!("受け取ったキーワード: {}", payload.keyword);
    // TODO: config.toml に書き込む処理などをここに入れる
    "OK"
}

pub async fn run_api_server() {
    let app = Router::new()
        .route("/keywords", post(update_keyword))
        .layer(CorsLayer::permissive()); // ← 開発用に全許可（本番は注意）
    // ↓本番用（許可するアドレスを自分のFlutterアプリのみに変える）
    // CorsLayer::new().allow_origin("https://example.com");
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("APIサーバー起動中 → http://{}", addr);
    serve(listener, app).await.unwrap();
}
