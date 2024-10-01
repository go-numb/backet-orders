use log::{error, info, trace};
use serde_json::{json, Value};
use std::env;

mod bybit_v5;
use crate::bybit_v5::{instrument, ticker};
use crypto_botters::{
    bybit::{BybitHttpAuth, BybitOption},
    Client,
};

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of top ranking items to retrieve
    #[arg(short = 'n', long, default_value_t = 30)]
    pub top_n: usize,

    /// Price ratio threshold
    #[arg(short = 'r', long, default_value_t = 0.01)]
    pub price_ratio: f64,

    /// Profit trigger Price ratio threshold
    #[arg(short = 'p', long, default_value_t = 0.01)]
    pub profit_price_ratio: f64,

    /// USD
    #[arg(short = 'u', long, default_value_t = 100.0)]
    pub usd: f64,
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let (top_n, price_ratio, profit_price_ratio, usd) = {
        let args = Args::parse();
        // parse number
        let top_n = args.top_n;
        // parse ratio
        let price_ratio = args.price_ratio;

        // parse profit ratio
        let profit_price_ratio = args.profit_price_ratio;

        // parse usd
        let usd = args.usd;
        (top_n, price_ratio, profit_price_ratio, usd)
    };

    // 実行前にコンソールに確認を求める
    println!(
        "top_n: {}, price_ratio: {}, profit_price_ratio: {}, usd: {}, total_usd: {}",
        top_n,
        price_ratio,
        profit_price_ratio,
        usd,
        top_n as f64 * usd
    );
    println!("Can I run it with the above settings? [y/N]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if !input.trim().to_lowercase().starts_with('y') {
        println!("Canceled");
        return;
    }

    let client = {
        let mut client = Client::new();
        let keys = env::var("BYBIT")
            .expect("BYBIT is not set")
            .split('.')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if keys.is_empty() {
            panic!("BYBIT is not set");
        }

        trace!("{:?}", keys);

        client.update_default_option(BybitOption::Key(keys[0].to_string()));
        client.update_default_option(BybitOption::Secret(keys[1].to_string()));
        client
    };

    // sort by volume
    let targets = {
        let res: ticker::Response = client
            .get(
                "/v5/market/tickers",
                Some(&[("category", "linear")]),
                [BybitOption::HttpAuth(BybitHttpAuth::V3AndAbove)],
            )
            .await
            .expect("failed to get tickers");

        let mut tickers = res.result.list;
        tickers.sort_by(|a, b| {
            let a = {
                let c = a.volume_24h.parse::<f64>().unwrap();
                let d = a.index_price.parse::<f64>().unwrap();
                c * d
            };
            let b = {
                let c = b.volume_24h.parse::<f64>().unwrap();
                let d = b.index_price.parse::<f64>().unwrap();
                c * d
            };
            b.partial_cmp(&a).unwrap()
        });

        // 上位50件を取得
        tickers
            .into_iter()
            .take(top_n)
            .collect::<Vec<ticker::ResultItem>>()
    };

    let instruments = {
        let res: instrument::Response = client
            .get(
                "/v5/market/instruments-info",
                Some(&[("category", "linear")]),
                [BybitOption::HttpAuth(BybitHttpAuth::V3AndAbove)],
            )
            .await
            .expect("failed to get tickers");

        // symbolをkeyにしたmapを作成
        let mut map = std::collections::HashMap::new();
        for item in res.result.list {
            map.insert(item.symbol.clone(), item);
        }
        map
    };

    for target in targets {
        println!("{:?}", target.symbol);

        // order to symbol
        let decimal_places = get_decimal_places(instruments[&target.symbol].price_scale.as_str());
        let qty_decimal_places = get_decimal_places(
            instruments[&target.symbol]
                .lot_size_filter
                .min_order_qty
                .as_str(),
        );

        let ltp = target.index_price.parse::<f64>().unwrap();

        let res: Value = client
            .post(
                "/v5/order/create",
                Some(json!({
                    "category": "linear",
                    "symbol": target.symbol,
                    "isLeverage": 1,
                    "side": "Buy",
                    "orderType": "Limit",
                    "price": format!("{:.*}",decimal_places, ltp * (1.0 - price_ratio)),
                    "qty": format!("{:.*}",qty_decimal_places, usd / ltp),


                    "triggerDirection": 1, // upside trigger
                    "triggerPrice": format!("{:.*}",decimal_places, ltp * (1.0 + profit_price_ratio)),
                    "triggerBy": "MarkPrice",

                    "timeInForce": "PostOnly",
                })),
                [BybitOption::HttpAuth(BybitHttpAuth::V3AndAbove)],
            )
            .await
            .expect("failed to create order");

        if res["retCode"].as_i64().unwrap() != 0 {
            error!(
                "{}, symbol: {}, response: {:?}",
                res["retMsg"], target.symbol, res
            );
            continue;
        } else {
            info!(
                "{}, symbol: {}, response: {:?}",
                res["retMsg"], target.symbol, res["orderId"]
            );
        }

        // sleep 0.25s
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
    }
}

// 浮動小数点の位を取得
fn get_decimal_places(s: &str) -> usize {
    s.split('.')
        .nth(1)
        .map_or(0, |decimal_part| decimal_part.len())
}
