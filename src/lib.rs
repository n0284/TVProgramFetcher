use chrono::{DateTime, Duration, Utc};
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::fs;
pub mod api;

// 番組表取得と通知処理
pub async fn run_batch() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();

    // APIキーとSlack Webhook URLを.env等から取得
    let nhk_api_key = env::var("NHK_API_KEY").expect("ない");
    let slack_webhook_url = env::var("SLACK_WEBHOOK_URL").expect("ない");

    // NHK APIにリクエスト
    // 一週間分の日付を生成
    let dates = get_dates_for_next_week();
    // 一日ずつリクエスト、番組検索
    let mut all_matching = Vec::new();
    for date in dates {
        // リクエスト
        let url = format!(
            "https://api.nhk.or.jp/v2/pg/list/130/e1/{}.json?key={}",
            date, nhk_api_key
        );
        let client = reqwest::Client::new();
        let res = client.get(url).send().await?;
        let body = res.text().await?;
        // 番組表が取得できなかった日はスキップ
        if body.contains("error") {
            println!("⚠️ {} の番組表が取得できないためスキップします。", date);
            continue; // ← この日だけスキップして次の日へ
        }
        // JSONデコード
        let parsed: NHKResponse = serde_json::from_str(&body)?;
        // 特定のキーワードを含む番組を検索
        let config = load_config()?;
        let keywords = config.keywords;
        let matching: Vec<_> = parsed
            .list
            .values()
            .flat_map(|list| list)
            .filter(|program| {
                keywords.iter().any(|keyword| {
                    program.title.contains(keyword) || program.content.contains(keyword)
                })
            })
            .cloned()
            .collect();
        if !matching.is_empty() {
            all_matching.extend(matching);
        }
    }

    // Slackに通知
    if !all_matching.is_empty() {
        let message = all_matching
            .iter()
            .map(|program| {
                // 日付パース
                let parsed_time = DateTime::parse_from_rfc3339(&program.start_time);
                // 整形 or fallback
                match parsed_time {
                    Ok(dt) => format!(
                        "📅 {} 📺 {}",
                        dt.format("%Y年%-m月%-d日 %H:%M～"),
                        program.title
                    ),
                    Err(_) => format!("📅 {} 📺 {}", program.start_time, program.title), // パース失敗時はそのまま
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
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

#[derive(Debug, Deserialize, Clone)]
struct Program {
    title: String,
    content: String,
    start_time: String,
}

#[derive(Deserialize)]
struct Config {
    keywords: Vec<String>,
}

// configファイルの読み込み
fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

// 一週間分の日付生成
fn get_dates_for_next_week() -> Vec<String> {
    let today = Utc::now().date_naive();
    (0..6)
        .map(|i| (today + Duration::days(i)).format("%Y-%m-%d").to_string())
        .collect()
}


// サーバー起動処理呼び出し
pub async fn run_api_server() {
    // src/api.rsのサーバー起動処理呼び出し
    api::run_api_server().await;
}