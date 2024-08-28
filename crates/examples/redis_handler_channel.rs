use redis;
use std::sync::mpsc;
use std::thread;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct MarketData {
    trading_day: String,
    pub instrument_id: String,
    pub last_price: f64,
    volume: f64,
    update_time: String,
    publish_time: String
}

pub fn subscribe_to_channels(channels: Vec<&str>) -> Result<Vec<mpsc::Receiver<MarketData>>, redis::RedisError> {
    let mut receivers = Vec::new();

    for channel in channels {
        let (tx, rx) = mpsc::channel();
        let channel_clone = channel.to_string();

        thread::spawn(move || {
            let client = match redis::Client::open("redis://:jz@202407@127.0.0.1/") {
                Ok(client) => client,
                Err(e) => {
                    eprintln!("Failed to open Redis client: {}", e);
                    return;
                }
            };
            let mut con = match client.get_connection() {
                Ok(con) => con,
                Err(e) => {
                    eprintln!("Failed to get Redis connection: {}", e);
                    return;
                }
            };
            let mut pubsub = con.as_pubsub();
            if let Err(e) = pubsub.subscribe(&channel_clone) {
                eprintln!("Failed to subscribe to channel {}: {}", channel_clone, e);
                return;
            }

            loop {
                let msg = match pubsub.get_message() {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("Failed to get message: {}", e);
                        continue;
                    }
                };
                let payload: String = match msg.get_payload() {
                    Ok(payload) => payload,
                    Err(e) => {
                        eprintln!("Failed to get payload: {}", e);
                        continue;
                    }
                };
                // 解析 JSON 数据
                let market_data: Result<MarketData, _> = serde_json::from_str(&payload);
                match market_data {
                    Ok(data) => {
                        if tx.send(data).is_err() {
                            eprintln!("Failed to send message to channel");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {}", e);
                    }
                }
            }
        });

        receivers.push(rx);
    }

    Ok(receivers)
}
