use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use sqlx::prelude::FromRow;

#[serde_as]
#[derive(Debug , Serialize , Deserialize , FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DepthInterval {
    #[serde_as(as = "DisplayFromStr")]
    pub asset_depth: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub asset_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "assetPriceUSD")]
    pub asset_price_usd: f64 ,
    #[serde_as(as = "DisplayFromStr")]
    pub end_time: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub liquidity_units: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub luvi: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub members_count: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub rune_depth: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub start_time: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub synth_supply: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub synth_units: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub units: i64,
}

#[serde_as]
#[derive(Debug , Serialize , Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunePoolInterval {
    #[serde_as(as = "DisplayFromStr")]
    pub count: i64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub end_time: i64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub start_time: i64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub units: i64,
}

#[serde_as]
#[derive(Debug , Serialize , Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pool {
    #[serde_as(as = "DisplayFromStr")]
    pub asset_liquidity_fees: i64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub earnings: i64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub pool: String,
    
    #[serde_as(as = "DisplayFromStr")]
    pub rewards: i64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub rune_liquidity_fees: i64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub saver_earning: i64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub total_liquidity_fees_rune: i64,
}

#[serde_as]
#[derive(Debug , Serialize , Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningInterval {
    #[serde_as(as = "DisplayFromStr")]
    pub avg_node_count: f64,

    #[serde_as(as = "DisplayFromStr")]
    pub block_rewards: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub bonding_earnings: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub earnings: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub end_time: i64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub liquidity_earnings: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub liquidity_fees: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    
    pub start_time: i64,
    pub pools: Vec<Pool> 
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapsInterval {
    #[serde_as(as = "DisplayFromStr")]
    pub average_slip: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub end_time: i64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub from_trade_average_slip: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub from_trade_count: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub from_trade_fees: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub from_trade_volume: f64,
    
    #[serde(rename = "fromTradeVolumeUSD")]
    #[serde_as(as = "DisplayFromStr")]
    pub from_trade_volume_usd: f64,
    
    #[serde(rename = "runePriceUSD")]
    #[serde_as(as = "DisplayFromStr")]
    pub rune_price_usd: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub start_time: i64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub synth_mint_average_slip: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub synth_mint_count: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub synth_mint_fees: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub synth_mint_volume: f64,
    
    #[serde(rename = "synthMintVolumeUSD")]
    #[serde_as(as = "DisplayFromStr")]
    pub synth_mint_volume_usd: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub synth_redeem_average_slip: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub synth_redeem_count: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub synth_redeem_fees: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub synth_redeem_volume: f64,
    
    #[serde(rename = "synthRedeemVolumeUSD")]
    #[serde_as(as = "DisplayFromStr")]
    pub synth_redeem_volume_usd: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub to_asset_average_slip: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub to_asset_count: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub to_asset_fees: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub to_asset_volume: f64,
    
    #[serde(rename = "toAssetVolumeUSD")]
    #[serde_as(as = "DisplayFromStr")]
    pub to_asset_volume_usd: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub to_rune_average_slip: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub to_rune_count: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub to_rune_fees: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub to_rune_volume: f64,
    
    #[serde(rename = "toRuneVolumeUSD")]
    #[serde_as(as = "DisplayFromStr")]
    pub to_rune_volume_usd: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub total_count: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub total_fees: f64,
    
    #[serde_as(as = "DisplayFromStr")]
    pub total_volume: f64,

    #[serde(rename = "totalVolumeUSD")]
    #[serde_as(as = "DisplayFromStr")]
    pub total_volume_usd: f64,
}