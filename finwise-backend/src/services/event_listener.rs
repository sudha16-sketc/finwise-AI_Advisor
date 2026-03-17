use crate::db::Database;
use crate::services::log_transaction;
use reqwest::Client;
use serde_json::Value;
use tokio::time::{interval, Duration};
use std::env;

pub async fn start_event_listener(db: Database) {
    let horizon_url = env::var("HORIZON_URL").unwrap_or_else(|_| "https://horizon-testnet.stellar.org".to_string());
    let contract_address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set"); // Add to .env
    let client = Client::new();

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(30)); // Poll every 30s

        loop {
            interval.tick().await;

            // Poll Soroban events (simplified - use Soroban RPC or Horizon /events in prod)
            let url = format!("{}/soroban/events?contract={}", horizon_url, contract_address);
            match client.get(&url).query(&[("type", "deposit"), ("type", "withdraw")]).send().await {
                Ok(resp) => if let Ok(events) = resp.json::<Vec<Value>>().await {
                    for event in events {
                        if let Some(topics) = event["topics"].as_array() {
                            if topics.len() >= 2 {
                                let event_type = topics[0]["name"].as_str().unwrap_or("");
                                let wallet = topics[1].as_str().unwrap_or("");
                                let amount = event["data"].as_f64();

                                if !wallet.is_empty() {
                                    log_transaction(&db, wallet, event_type, amount).await.ok();
                                }
                            }
                        }
                    }
                },
                Err(e) => log::warn!("Event poll error: {}", e),
            }
        }
    });
}

