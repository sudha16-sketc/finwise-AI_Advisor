// src/services/event_listener.rs
use crate::db::Database;
use crate::services::log_transaction;
use reqwest::Client;
use serde_json::Value;
use tokio::time::{interval, Duration};
use std::env;

pub async fn start_event_listener(db: Database) {
    let horizon_url = env::var("HORIZON_URL")
        .unwrap_or_else(|_| "https://horizon-testnet.stellar.org".to_string());

    // ✅ CONTRACT_ADDRESS is optional — skip listener gracefully if not set
    let contract_address = match env::var("CONTRACT_ADDRESS") {
        Ok(addr) if !addr.is_empty() => addr,
        _ => {
            log::warn!("⚠️ CONTRACT_ADDRESS not set — skipping event listener");
            return;
        }
    };

    let client = Client::new();

    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(30));

        loop {
            ticker.tick().await;

            let url = format!(
                "{}/soroban/events?contract={}",
                horizon_url, contract_address
            );

            match client
                .get(&url)
                .query(&[("type", "deposit"), ("type", "withdraw")])
                .send()
                .await
            {
                Ok(resp) => {
                    match resp.json::<Vec<Value>>().await {
                        Ok(events) => {
                            for event in events {
                                if let Some(topics) = event["topics"].as_array() {
                                    if topics.len() >= 2 {
                                        let event_type =
                                            topics[0]["name"].as_str().unwrap_or("");
                                        let wallet = topics[1].as_str().unwrap_or("");
                                        let amount = event["data"].as_f64();

                                        if !wallet.is_empty() && !event_type.is_empty() {
                                            if let Err(e) = log_transaction(
                                                &db,
                                                wallet,
                                                event_type,
                                                amount,
                                            )
                                            .await
                                            {
                                                log::warn!(
                                                    "⚠️ Failed to log transaction: {}",
                                                    e
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => log::warn!("⚠️ Failed to parse events: {}", e),
                    }
                }
                Err(e) => log::warn!("⚠️ Event poll error: {}", e),
            }
        }
    });
}