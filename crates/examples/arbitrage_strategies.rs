// use std::time::Duration;
// use chrono::Utc;
// use tokio::time;
// use reqwest::Client;
// use serde_json::json;

// #[derive(Debug)]
// struct ArbitrageConfig {
//     contract_a: String, // 合约A的名称
//     contract_b: String, // 合约B的名称
//     price_diff_threshold: f64, // 价格差的阈值，超过此阈值则进行套利
//     opponent_price_diff: f64, // 对手价差的阈值，用于判断对手价是否满足条件
//     opponent_time_threshold: Duration, // 对手价格更新时间的阈值，超过此阈值则重新获取对手价格
//     total_order_amount: f64, // 总下单量
//     single_order_amount: f64, // 单笔下单量
//     order_interval: Duration, // 下单间隔
// }

// async fn fetch_price(client: &Client, contract: &str) -> Result<f64, reqwest::Error> {
//     // 模拟获取价格
//     let url = format!("https://api.exchange.com/prices/{}", contract);
//     let response = client.get(&url).send().await?;
//     let price: f64 = response.json().await?;

//     Ok(price)
// }

// async fn place_order(client: &Client, contract: &str, amount: f64, price: f64) -> Result<(), reqwest::Error> {
//     // 模拟下单
//     let url = format!("https://api.exchange.com/orders");
//     let order = json!({
//         "contract": contract,
//         "amount": amount,
//         "price": price,
//     });

//     client.post(&url).json(&order).send().await?;

//     Ok(())
// }

// /**
//  * 异步函数，模拟套利策略
//  */
// async fn arbitrage_strategy(config: ArbitrageConfig) {
//     let client = Client::new(); // 创建HTTP客户端
//     let mut total_ordered = 0.0; // 初始化总下单量为0

//     while total_ordered < config.total_order_amount { // 循环，直到总下单量达到配置中的总下单量
//         let price_a = fetch_price(&client, &config.contract_a).await.unwrap(); // 获取合约A和合约B的价格
//         let price_b = fetch_price(&client, &config.contract_b).await.unwrap(); 
//         let price_diff = (price_a - price_b).abs(); // 计算价格差异

//         if price_diff > config.price_diff_threshold { // 如果价格差异超过阈值，则进行套利
//             // 满足套利条件
//             place_order(&client, &config.contract_a, config.single_order_amount, price_b).await.unwrap();

//             // 等待对手价成交
//             let start_time = Utc::now();
//             loop {
//                 let opponent_price = fetch_price(&client, &config.contract_b).await.unwrap();
//                 let elapsed_time = Utc::now() - start_time;

//                 if elapsed_time.to_std().unwrap() < config.opponent_time_threshold && (opponent_price - price_b).abs() <= config.opponent_price_diff {
//                     place_order(&client, &config.contract_b, config.single_order_amount, opponent_price).await.unwrap();
//                     break;
//                 } else if elapsed_time.to_std().unwrap() >= config.opponent_time_threshold {
//                     // 超时处理，锁仓或其他逻辑
//                     println!("Opponent order timeout, taking risk management action.");
//                     break;
//                 }
//                 time::sleep(Duration::from_millis(100)).await;// 短暂等待再检查
//             }
//             total_ordered += config.single_order_amount;
//             time::sleep(config.order_interval).await; // 下单间隔
//         } else {
//             // 未满足套利条件，等待
//             time::sleep(Duration::from_secs(1)).await;
//         }
//     }
// }

// #[tokio::main]
// async fn main() {
//     // 主函数，程序入口
//     // 1. 定义套利策略的配置
//     // 2. 调用套利策略函数
//     let config = ArbitrageConfig {
//         contract_a: "contract_a".to_string(),
//         contract_b: "contract_b".to_string(),
//         price_diff_threshold: 0.5,
//         opponent_price_diff: 0.2,
//         opponent_time_threshold: Duration::from_secs(5),
//         total_order_amount: 1000.0,
//         single_order_amount: 100.0,
//         order_interval: Duration::from_secs(10),
//     };

//     arbitrage_strategy(config).await;
// }
use chrono::{DateTime, Utc, Duration};
use std::thread;
use std::time::Duration as StdDuration;
use log::{info, warn};

pub struct ArbitrageStrategy {
    threshold: f64,
    opposite_threshold: f64,
    opposite_time_limit: Duration,
    total_volume: u32,
    order_volume: u32,
    order_interval: StdDuration,
    risk_time_limit: Duration,
}

impl ArbitrageStrategy {
    pub fn new(
        threshold: f64,
        opposite_threshold: f64,
        opposite_time_limit: Duration,
        total_volume: u32,
        order_volume: u32,
        order_interval: StdDuration,
        risk_time_limit: Duration,
    ) -> Self {
        Self {
            threshold,
            opposite_threshold,
            opposite_time_limit,
            total_volume,
            order_volume,
            order_interval,
            risk_time_limit,
        }
    }

    pub fn execute(
        &self,
        contract1_price: f64,
        contract1_timestamp: DateTime<Utc>,
        contract2_price: f64,
        contract2_timestamp: DateTime<Utc>,
    ) {
        let spread = contract1_price - contract2_price;

        if contract1_timestamp.signed_duration_since(Utc::now()) > self.risk_time_limit 
            || contract2_timestamp.signed_duration_since(Utc::now()) > self.risk_time_limit {
            warn!("Price data is outdated, skipping trading.");
            return;
        }

        if spread.abs() > self.threshold {
            for _ in 0..(self.total_volume / self.order_volume) {
                info!("Checking spread: {}", spread);
                if spread > 0.0 {
                    self.place_order(contract1_price, contract2_price, true);
                } else {
                    self.place_order(contract2_price, contract1_price, false);
                }
                
                thread::sleep(self.order_interval);
                
                let success = self.monitor_order(contract1_price, contract2_price, spread > 0.0);

                if !success {
                    warn!("Order not filled within time limit, checking opposite threshold.");
                    if spread.abs() > self.opposite_threshold {
                        info!("Placing opposite order due to spread: {}", spread);
                        self.place_opposite_order(contract1_price, contract2_price, spread > 0.0);
                    } else {
                        warn!("Spread does not meet opposite threshold, locking position.");
                        self.lock_position();
                    }
                }
            }
        }
    }

    fn place_order(&self, price_a: f64, price_b: f64, is_a_first: bool) {
        if is_a_first {
            info!("Placing order: Sell A at ${}, Buy B at ${}", price_a, price_b);
            // Place sell order for A and buy order for B
        } else {
            info!("Placing order: Sell B at ${}, Buy A at ${}", price_b, price_a);
            // Place sell order for B and buy order for A
        }
    }

    fn monitor_order(&self, price_a: f64, price_b: f64, is_a_first: bool) -> bool {
        let start_time = Utc::now();
        while Utc::now().signed_duration_since(start_time) <= self.opposite_time_limit {
            // Check if the order is filled
            // Simulate order being filled
            thread::sleep(StdDuration::from_secs(1));
            let filled = true; // This should be replaced with the actual order status check
            if filled {
                info!("Order filled successfully.");
                return true;
            }
        }
        false
    }

    fn place_opposite_order(&self, price_a: f64, price_b: f64, is_a_first: bool) {
        if is_a_first {
            info!("Placing opposite order: Buy A at ${}, Sell B at ${}", price_a, price_b);
            // Place opposite order: Buy A and Sell B
        } else {
            info!("Placing opposite order: Buy B at ${}, Sell A at ${}", price_b, price_a);
            // Place opposite order: Buy B and Sell A
        }
    }

    fn lock_position(&self) {
        info!("Locking position to prevent further losses.");
        // Logic to lock the position
    }
}