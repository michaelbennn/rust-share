### rust编写的一些交易工具集合
* crates/ctp_futures 期货CTP接口的封装
* crates/tora_stock 证券交易接口的封装
* crates/rust_share_util 公共的实用函数集合
* crates/examples 一些示例
* crates/spider 抓取一些财经网站的数据
* crates/gateway 交易网关, 完成订单执行，持仓调整
* app/log_pump 简单的消息推送

### 说明
* 如果在ubuntu上安装clang，需要首先apt-get update 和 apt-get upgrade，然后通过apt-get install clang-15即可。15.0.7版本才可以运行当前的rust代码。
* 编译过程中会用到clang(15.0.7), 需要手动下载安装对应的依赖, [下载](https://github.com/llvm/llvm-project/releases/tag/llvmorg-15.0.7)
* 仅在ubuntu 20.04 和 windows11 上编译测试过

### 编译
```
cargo build
```

### 示例
```
cargo run --example ctp-query
```

### gateway
* 运行网关
```
sh scripts/run_gateway.sh
```


* 网关处理账号登陆，重连，查询，下单的细节
* 网关默认提供一个http api供远程调用
* 网关提供一个持仓对齐功能，比如发送请求设定合约 SHFE:ru2409为3手多头持仓，那么post发送以下请求到网关即可:
```
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
	"broker_id": "9999",
	"account": "143650", 
  	"target":{
		"exchange":"SHFE",
		"symbol": "ru2409",
		"position": 3, 
		"shift": 1,
		"close_priority": "YesterdayFirt"
	}, 
	"credential":""}' \
  http://localhost:10111/api/set_contract_target

```
* 网关会自动订阅合约行情，根据最新报价偏移shift * price_tick作为发单价
* close_priority 用于指定优先平仓顺序，因为某些情况下平昨和平今的手续费不一样


### 计划
第一步需要参考ctp_trade.rs，在examples目录下实现策略的功能，然后直接使用cargo run的形式运行编译文件。
未来可能需要将ctp相关的交易模块编译为一个动态链接库，然后编写rust策略调用。

### BUG 修复

使用vscode的rust-analyzer插件，会报错  `tokio::main: proc-macro crate is missing its build datarust-analyzermacro-error，`
需要在设置扩展为     "rust-analyzer.cargo.features": ["full"], 但是这也会有一个问题，就是自定义的各种方法，不能通过鼠标点击+Ctrl跳转到定义

