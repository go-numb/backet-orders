use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: Result,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub category: String,
    pub list: Vec<Instrument>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instrument {
    pub symbol: String,
    #[serde(rename = "contractType")]
    pub contract_type: String,
    pub status: String,
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(rename = "quoteCoin")]
    pub quote_coin: String,
    #[serde(rename = "launchTime")]
    pub launch_time: String,
    #[serde(rename = "deliveryTime")]
    pub delivery_time: String,
    #[serde(rename = "deliveryFeeRate")]
    pub delivery_fee_rate: String,
    #[serde(rename = "priceScale")]
    pub price_scale: String,
    #[serde(rename = "leverageFilter")]
    pub leverage_filter: LeverageFilter,
    #[serde(rename = "priceFilter")]
    pub price_filter: PriceFilter,
    #[serde(rename = "lotSizeFilter")]
    pub lot_size_filter: LotSizeFilter,
    #[serde(rename = "unifiedMarginTrade")]
    pub unified_margin_trade: bool,
    #[serde(rename = "fundingInterval")]
    pub funding_interval: i32,
    #[serde(rename = "settleCoin")]
    pub settle_coin: String,
    #[serde(rename = "copyTrading")]
    pub copy_trading: String,
    #[serde(rename = "upperFundingRate")]
    pub upper_funding_rate: String,
    #[serde(rename = "lowerFundingRate")]
    pub lower_funding_rate: String,
    #[serde(rename = "isPreListing", default)]
    pub is_pre_listing: Option<bool>,
    #[serde(rename = "preListingInfo", default)]
    pub pre_listing_info: Option<PreListingInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeverageFilter {
    #[serde(rename = "minLeverage")]
    pub min_leverage: String,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: String,
    #[serde(rename = "leverageStep")]
    pub leverage_step: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceFilter {
    #[serde(rename = "minPrice")]
    pub min_price: String,
    #[serde(rename = "maxPrice")]
    pub max_price: String,
    #[serde(rename = "tickSize")]
    pub tick_size: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LotSizeFilter {
    #[serde(rename = "maxOrderQty")]
    pub max_order_qty: String,
    #[serde(rename = "maxMktOrderQty")]
    pub max_mkt_order_qty: String,
    #[serde(rename = "minOrderQty")]
    pub min_order_qty: String,
    #[serde(rename = "qtyStep")]
    pub qty_step: String,
    #[serde(rename = "postOnlyMaxOrderQty")]
    pub post_only_max_order_qty: String,
    #[serde(rename = "minNotionalValue")]
    pub min_notional_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreListingInfo {
    #[serde(rename = "curAuctionPhase")]
    pub cur_auction_phase: String,
    pub phases: Vec<Phase>,
    #[serde(rename = "auctionFeeInfo")]
    pub auction_fee_info: AuctionFeeInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phase {
    pub phase: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuctionFeeInfo {
    #[serde(rename = "auctionFeeRate")]
    pub auction_fee_rate: String,
    #[serde(rename = "takerFeeRate")]
    pub taker_fee_rate: String,
    #[serde(rename = "makerFeeRate")]
    pub maker_fee_rate: String,
}
