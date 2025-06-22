use reqwest::Client;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // APIキーとSlack Webhook URLを.env等から取得
    let nhk_api_key = env::var("NHK_API_KEY").expect("ない");
    let slack_webhook_url = env::var("SLACK_WEBHOOK_URL").expect("ない");

    // NHK APIにリクエスト
    let url = format!(
        "https://api.nhk.or.jp/v2/pg/list/130/e1/2025-06-22.json?key={}",
        nhk_api_key
    );
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    let body = res.text().await?;

    // JSONデコード
    let parsed: NHKResponse = serde_json::from_str(&body)?;

    // 特定のキーワードを含む番組を検索
    let config = load_config()?;
    let keyword = config.keyword;
    let matching: Vec<_> = parsed.list.values()
        .flat_map(|list| list)
        .filter(|program| program.title.contains(&keyword) || program.content.contains(&keyword))
        .collect();

    // Slackに通知
    if !matching.is_empty() {
        let message = format!(
            "🔔 今日の番組に '{}' を含むものがあります:\n{}",
            keyword,
            matching.iter().map(|p| format!("- {}", p.title)).collect::<Vec<_>>().join("\n")
        );
        Client::new()
            .post(&slack_webhook_url)
            .json(&serde_json::json!({ "text": message }))
            .send()
            .await?;
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct NHKResponse {
    list: std::collections::HashMap<String, Vec<Program>>,
}

#[derive(Debug, Deserialize)]
struct Program {
    id: String,
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct Config {
    keyword: String,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}