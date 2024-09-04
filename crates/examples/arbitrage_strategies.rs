use chrono::{DateTime, Duration, FixedOffset, Utc};
use std::thread;
use std::time::Duration as StdDuration;
use log::{info, warn};


pub fn get_beijing_time() -> DateTime<FixedOffset> {
    // 获取当前的UTC时间
    let utc_now: DateTime<Utc> = Utc::now();
    // 使用 chrono_tz 将 UTC 时间转换为北京时间
    let beijing_time: DateTime<FixedOffset> = utc_now.with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
    
    beijing_time
}

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
        contract1_timestamp: DateTime<FixedOffset>,
        contract2_price: f64,
        contract2_timestamp: DateTime<FixedOffset>,
    ) {
        let spread = contract1_price - contract2_price; // 当前价差
        let duration_time = Duration::seconds(8*3600);
        if get_beijing_time() - contract1_timestamp - duration_time > self.risk_time_limit 
            || get_beijing_time() - contract2_timestamp - duration_time > self.risk_time_limit { // 超过风控时间限制
            warn!("Price data is outdated, skipping trading.");
            return;
        }
        println!("Spread: {}", spread);
        if spread.abs() > self.threshold { // 满足价差要求
            for _ in 0..(self.total_volume / self.order_volume) {
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
                        println!("Placing opposite order due to spread: {}", spread);
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
            //TODO: CTP order insert interface
            println!("Placing order: Sell A at ${}, Buy B at ${}", price_a, price_b);
            // Place sell order for A and buy order for B
        } else {
            println!("Placing order: Sell B at ${}, Buy A at ${}", price_b, price_a);
            // Place sell order for B and buy order for A
        }
    }

    fn monitor_order(&self, price_a: f64, price_b: f64, is_a_first: bool) -> bool {
        let start_time = get_beijing_time();
        while get_beijing_time().signed_duration_since(start_time) <= self.opposite_time_limit {
            // Check if the order is filled
            // Simulate order being filled
            thread::sleep(StdDuration::from_secs(1));
            let filled = true; // This should be replaced with the actual order status check
            if filled {
                println!("Order filled successfully.");
                return true;
            }
        }
        false
    }

    fn place_opposite_order(&self, price_a: f64, price_b: f64, is_a_first: bool) {
        if is_a_first {
            println!("Placing opposite order: Buy A at ${}, Sell B at ${}", price_a, price_b);
            // Place opposite order: Buy A and Sell B
        } else {
            println!("Placing opposite order: Buy B at ${}, Sell A at ${}", price_b, price_a);
            // Place opposite order: Buy B and Sell A
        }
    }

    fn lock_position(&self) {
        println!("Locking position to prevent further losses.");
        // Logic to lock the position
    }
}



// #[cfg(test)]
// mod tests {
//     use super::*;
//     use chrono::Utc;
//     use log::Level;


//     #[test]
//     fn test_execute() {
//         let _ = env_logger::builder().filter_level(
//             Level::Info.to_level_filter()
//         ).try_init();

//         let strategy = ArbitrageStrategy::new(
//             0.5,  // 下单价差要求
//             0.2, // 对手价成交价差要求
//             Duration::seconds(5), // 对手价成交时间要求(秒)
//             1000, // 下单总量
//             100, // 单次下单量
//             StdDuration::from_secs(1), // 下单时间间隔(秒)
//             Duration::seconds(10), // 风控时间要求(秒)
//         );

//         let now = get_beijing_time();
//         let contract1_price = 100.0;
//         let contract1_timestamp = now;
//         let contract2_price = 99.0;
//         let contract2_timestamp = now;

//         strategy.execute(contract1_price, contract1_timestamp, contract2_price, contract2_timestamp);
//     }
// }