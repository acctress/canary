use std::collections::HashMap;
use std::path::Path;
use notify_rust::Notification;
use serde::Deserialize;
use reqwest::header;
use reqwest::header::{HeaderValue, USER_AGENT};

#[derive(Deserialize, Debug)]
struct Config {
    ticker: String,
    interval: u64,
    alert_above: f64,
    alert_below: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
        .summary("canary")
        .body("Canary started")
        .icon("dialog-information")
        .show()?;

    let default_config: Config = Config {
        ticker: "AAPL".to_string(),
        interval: 60,
        alert_above: 100.0,
        alert_below: 100.0,
    };

    let mut config = default_config;

    let cfg_file = Path::new("config.json");
    match std::fs::metadata(cfg_file) {
        Ok(_) => {
            println!("[canary] using config.json");

            match std::fs::read(cfg_file) {
                Ok(data) => {
                    println!("[canary] read config.json");
                    config = serde_json::from_slice(&data)?;
                }
                Err(e) => panic!("[canary] failed to read config.json: {}", e),
            }
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => println!("[canary] config not found, using default config."),
        Err(e) => println!("[canary] error whilst processing config: {}", e),
    }

    let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{}", config.ticker);

    println!("[canary] request url: {}", url);

    let mut headers = header::HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0"));

    let client = reqwest::Client::new();

    let mut last_price = 0.0;

    loop {

        let res = client
            .get(&url)
            .headers(headers.clone())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let current_price = res["chart"]["result"][0]["meta"]["regularMarketPrice"]
            .as_f64().unwrap();

        let change = current_price - last_price;
        let perc = if last_price != 0.0 { (change / last_price) * 100.0 } else { 0.0 };
        let sym = if change >= 0.0 { "↑" } else { "↓" };

        println!(
            "[canary] {} ${:.2}  {} {:.2} ({:.2}%)",
            config.ticker, current_price, sym, change.abs(), perc.abs()
        );

        if current_price > config.alert_above {
            println!("[canary] ⚠ alert: {} hit above threshold (${:.2})", config.ticker, config.alert_above);

            Notification::new()
                .summary("canary")
                .body(&format!("{} hit above ${:.2}", config.ticker, current_price))
                .icon("dialog-warning")
                .show()?;

        } else if current_price < config.alert_below {
            println!("[canary] ⚠ alert: {} hit below threshold (${:.2})", config.ticker, config.alert_below);

            Notification::new()
                .summary("canary")
                .body(&format!("{} hit below ${:.2}", config.ticker, current_price))
                .icon("dialog-warning")
                .show()?;
        }

        last_price = current_price;

        tokio::time::sleep(tokio::time::Duration::from_secs(config.interval)).await;
    }
}