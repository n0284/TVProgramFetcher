use tvprogramfetcher::run_api_server;

#[tokio::main]
async fn main() {
    // lib.rsのrun_api_server()を呼ぶ
    run_api_server().await;
}