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

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::Duration as StdDuration;
use arbitrage_strategies::ArbitrageStrategy;
use chrono::{DateTime, Duration, FixedOffset, Utc, NaiveDateTime};
use redis::RedisError;
mod redis_handler_channel;
mod arbitrage_strategies;

// use tokio;
// use chrono::Duration;
// use std::time::Duration as StdDuration;
// use log::LevelFilter;
// use simple_logger::SimpleLogger;

fn main() -> Result<(), RedisError> {

    /************************************************************************************************
     * ********************************* 获取合约行情数据 *********************************************
     * **********************************************************************************************
     */
    let channels = vec!["market:IC2409", "market:IC2410"];
    let receivers = redis_handler_channel::subscribe_to_channels(channels)?;
    
    // // 定义两个变量用于存储最新的两个合约价格
    // let mut last_price_ic: Option<f64> = None;
    // let mut last_price_lc: Option<f64> = None;

    // 定义两个变量用于存储最新的两个合约价格
    let last_price_ic = Arc::new(Mutex::new(None::<f64>));
    let last_price_lc = Arc::new(Mutex::new(None::<f64>));

    // 定义两个变量用于存储最新的两个合约时间戳
     // 定义两个变量用于存储最新的更新时间
     let publish_time_ic = Arc::new(Mutex::new(String::new()));
     let publish_time_lc = Arc::new(Mutex::new(String::new()));


    // 创建一个通道用于通知主线程更新价格
    let (tx, rx) = mpsc::channel();

    for (_i, receiver) in receivers.into_iter().enumerate() {
        let last_price_ic = Arc::clone(&last_price_ic);
        let last_price_lc = Arc::clone(&last_price_lc);

        let publish_time_ic = Arc::clone(&publish_time_ic);
        let publish_time_lc = Arc::clone(&publish_time_lc);

        let tx = tx.clone();

        thread::spawn(move || {
            for data in receiver {
                if data.instrument_id == "IC2409" {
                    let mut price = last_price_ic.lock().unwrap();
                    *price = Some(data.last_price);
                    let mut publish_time = publish_time_ic.lock().unwrap();
                    *publish_time = data.publish_time;
                } else if data.instrument_id == "IC2410" {
                    let mut price = last_price_lc.lock().unwrap();
                    *price = Some(data.last_price);
                    let mut publish_time = publish_time_lc.lock().unwrap();
                    *publish_time = data.publish_time;
                }
                // 通知主线程更新价格
                tx.send(()).unwrap();
            }
        });
    }

    /************************************************************************************************
     **************************************** 初始化套利策略 ******************************************  
     ************************************************************************************************ 
     */
    let arbitrage_strategies = ArbitrageStrategy::new(
        0.5, // 下单价差要求
        0.2, // 对手价成交价差要求
        Duration::seconds(5), // 对手价成交时间要求(秒)
        30, // 下单总量
        10, // 单次下单量
        StdDuration::from_secs(1), // 下单时间间隔(秒)
        Duration::seconds(20), // 风控时间要求(秒)
    );


    // 主线程监控和打印最新价格
    loop {
        rx.recv().unwrap(); // 等待更新通知
        let ic_price = last_price_ic.lock().unwrap();
        let lc_price = last_price_lc.lock().unwrap();
        println!("IC2409:{:?}, IC2410:{:?}", ic_price, lc_price);

        let ic_publish_time = publish_time_ic.lock().unwrap();
        let lc_publish_time = publish_time_lc.lock().unwrap();
        println!("IC2409 publish_time:{:?}, IC2410 publish_time:{:?}", ic_publish_time, lc_publish_time);
        // thread::sleep(::std::time::Duration::from_secs(1)); // 上述代码，只要有更新便会立即打印一次，而加上这句后会有一些数据卡顿在队列中。更新速度小于1s
        if ic_price.is_none() || lc_price.is_none() || ic_publish_time.is_empty() || lc_publish_time.is_empty() {
            continue;
        }

        // 转为时间类型
        let ic_timestamp  = DateTime::from_naive_utc_and_offset(NaiveDateTime::parse_from_str(&ic_publish_time , "%Y-%m-%d %H:%M:%S%.f").expect("Failed to parse ic_publish_time"), FixedOffset::east_opt(0).unwrap());
        let lc_timestamp  = DateTime::from_naive_utc_and_offset(NaiveDateTime::parse_from_str(&lc_publish_time , "%Y-%m-%d %H:%M:%S%.f").expect("Failed to parse lc_publish_time"), FixedOffset::east_opt(0).unwrap());
        
        // 执行套利策略
        arbitrage_strategies.execute(ic_price.unwrap(), ic_timestamp, lc_price.unwrap(), lc_timestamp);
        return Ok(());
    }
}