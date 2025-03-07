use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiscanToken {
    pub token_type: String,
    pub object_id: String,
    pub name: String,
    pub supply: Option<f64>,
    pub supply_in_usd: Option<f64>,
    pub token_price: Option<f64>,
    pub dominance: Option<f64>,
    pub circulating_supply: Option<f64>,
    pub market_cap: Option<f64>,
    pub total_volume: Option<f64>,
    pub max_supply: Option<f64>,
    pub fdv: Option<f64>,
    pub holders: Option<f64>,
    pub denom: String,
    pub package_id: String,
    pub create_timestamp: i64,
    pub creator: String,
    pub creator_name: Option<String>,
    pub creator_img: Option<String>,
    pub creator_scam_message: Option<String>,
    pub scam_message: Option<String>,
    pub decimals: i32,
    pub symbol: String,
    pub icon_url: Option<String>,
    pub description: String,
    pub bridge: bool,
    pub verified: bool,
}
