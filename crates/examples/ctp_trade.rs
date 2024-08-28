// use base::util::*;
// use base::*;
// use bincode::config;
// use ctp_futures::query::*;
// use ctp_futures::*;
// use log::info;
// use std::ffi::CStr;
// use std::io::Write;

// #[tokio::main]
// async fn main() {
//     // 初始化日志
//     init_logger(); 
//     // // let trade_front = "tcp://180.168.146.187:10130"; // 7*24
//     // // 初始化交易接口
//     // let account = TradingAccountConfig {
//     //     broker_id: "9999".to_string(),
//     //     account: "227375".to_string(),
//     //     trade_fronts: vec!["tcp://180.168.146.187:10202".to_string()],
//     //     md_fronts: vec!["tcp://180.168.146.187:10212".to_string()],
//     //     name_servers: vec![],
//     //     auth_code: "0000000000000000".to_string(),
//     //     user_product_info: "".to_string(),
//     //     app_id: "simnow_client_test".to_string(),
//     //     password: "Whale18814844533%".to_string(),
//     //     remark: "".into(),
//     //     fens_md_fronts: vec![],
//     //     fens_trade_fronts: vec![],
//     //     hd_serial: "".into(),
//     //     inner_ip_address: "".into(),
//     //     mac_address: "".into(),
//     //     money_password: "".into(),
//     //     route_type: "ctp_futures".into(),
//     //     query_fronts: vec![],
//     //     terminal_info: "".into(),
//     // };
//     // let result = query(&account).await.unwrap();
//     // let config = config::standard();
//     // let encoded: Vec<u8> = bincode::encode_to_vec(&result, config).unwrap();
//     // let save_path = std::path::Path::new(".cache")
//     //     .join("ctp_futures_query_result")
//     //     .join(format!("{}_{}", account.broker_id, account.account));
//     // info!("save_path = {:?}", save_path);
//     // check_make_dir(save_path.to_str().unwrap());
//     // let save_path = save_path.join(format!("{}.dat", result.trading_day));
//     // info!("path={:?}", save_path);
//     // let mut f = std::fs::File::create(&save_path).unwrap();
//     // f.write_all(&encoded).unwrap();
//     // info!("{} 查询完成. bin.len={}", account.account, encoded.len());
//     // let (decoded, _len): (CtpQueryResult, usize) =
//     //     bincode::decode_from_slice(&encoded[..], config).unwrap();
//     // info!(
//     //     "decoded {} {} {}",
//     //     decoded.broker_id, decoded.account, decoded.trading_day
//     // );
//     // let ver = unsafe { CStr::from_ptr(trader_api::get_api_version()) }
//     //     .to_str()
//     //     .unwrap();
//     // info!("version={ver}");


//     // 连接Redis的地址和密码
//     let redis_addr = "127.0.0.1:6379";
//     let redis_password = Some("jz@202407");

//     // 要订阅的合约
//     let contract = "lc2411";

//     // 连接到Redis
//     let mut client = redis_handler_channel::connect_redis(redis_addr, redis_password).await.unwrap();

//     // 订阅行情信息
//     redis_handler_channel::subscribe_to_ticker(&mut client, contract).await.unwrap();
// }



// use std::thread;
// use redis::RedisError;
// mod redis_handler_channel;

// pub fn init_logger() {
//     if std::env::var("RUST_LOG").is_err() {
//         std::env::set_var("RUST_LOG", "info")
//     }
//     tracing_subscriber::fmt::init();
// }

// fn main() -> Result<(), RedisError> {
//     let channels = vec!["market:IC2409", "market:lc2411"];
//     let receivers = redis_handler_channel::subscribe_to_channels(channels)?;

//     for (i, receiver) in receivers.into_iter().enumerate() {
//         thread::spawn(move || {
//             for received in receiver {
//                 println!("Received on channel {}: {}", i, received);
//             }
//         });
//     }

//     // Keep the main thread alive
//     loop {
//         thread::sleep(::std::time::Duration::from_secs(1));
//     }
// }

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
// use futures::channel::mpsc;
use redis::RedisError;
mod redis_handler_channel;
mod arbitrage_strategies;

// use tokio;
// use chrono::Duration;
// use std::time::Duration as StdDuration;
// use log::LevelFilter;
// use simple_logger::SimpleLogger;

fn main() -> Result<(), RedisError> {
    let channels = vec!["market:IC2409", "market:lc2411"];
    let receivers = redis_handler_channel::subscribe_to_channels(channels)?;
    
    // // 定义两个变量用于存储最新的两个合约价格
    // let mut last_price_ic: Option<f64> = None;
    // let mut last_price_lc: Option<f64> = None;

    // 定义两个变量用于存储最新的两个合约价格
    let last_price_ic = Arc::new(Mutex::new(None::<f64>));
    let last_price_lc = Arc::new(Mutex::new(None::<f64>));

    // 创建一个通道用于通知主线程更新价格
    let (tx, rx) = mpsc::channel();

    for (i, receiver) in receivers.into_iter().enumerate() {
        let last_price_ic = Arc::clone(&last_price_ic);
        let last_price_lc = Arc::clone(&last_price_lc);
        let tx = tx.clone();

        thread::spawn(move || {
            for data in receiver {
                if data.instrument_id == "IC2409" {
                    let mut price = last_price_ic.lock().unwrap();
                    *price = Some(data.last_price);
                } else if data.instrument_id == "lc2411" {
                    let mut price = last_price_lc.lock().unwrap();
                    *price = Some(data.last_price);
                }
                // 通知主线程更新价格
                tx.send(()).unwrap();
            }
        });
    }





    // let (tx, rx) = mpsc::channel(2);
    // let tx1 = tx.clone();

    // for (i, receiver) in receivers.into_iter().enumerate() {
    //     thread::spawn(move || {
    //         for data in receiver {
    //             // println!("Received on channel {}: {}", i, received);
    //             // // 解析接收到的消息，提取LastPrice
    //             // let market_data: Result<redis_handler_channel::MarketData, _> = serde_json::from_str(&received);
    //             // println!("received market data: {:?}", market_data)
    //             if data.instrument_id == "IC2409" {
    //                 last_price_ic = Some(data.last_price);
    //             } else if data.instrument_id == "lc2411" {
    //                 last_price_lc = Some(data.last_price);
    //             }
    //             println!("Received on : IC2409:{:?}, lc2411:{:?}", last_price_ic, last_price_lc);
    //             // match received {
    //             //     Ok(data) => {
    //             //         // println!("Received on : {:?}, {}", data, received);
    //             //     }
    //             //     Err(e) => {
    //             //         eprintln!("Failed to parse JSON: {}", e);
    //             //     }
    //             // }

    //         }
    //     });
    // }

    // 主线程监控和打印最新价格
    loop {
        rx.recv().unwrap(); // 等待更新通知
        let ic_price = last_price_ic.lock().unwrap();
        let lc_price = last_price_lc.lock().unwrap();
        println!("IC2409:{:?}, lc2411:{:?}", ic_price, lc_price);
        // thread::sleep(::std::time::Duration::from_secs(1));
    }
}