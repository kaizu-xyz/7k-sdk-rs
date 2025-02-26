use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingHistoryItem {
    pub digest: String,
    pub timestamp: String,
    pub coin_in: String,
    pub coin_out: String,
    pub amount_in: String,
    pub amount_out: String,
    pub volume: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingHistoryResponse {
    pub count: u64,
    pub history: Vec<TradingHistoryItem>,
}

pub struct GetSwapHistoryParams {
    pub owner: String,
    pub offset: u64,
    pub limit: u64,
    pub token_pair: Option<String>,
}

pub async fn get_swap_history(params: GetSwapHistoryParams) -> Result<TradingHistoryResponse> {
    let mut url_params = vec![
        ("addr", params.owner),
        ("offset", params.offset.to_string()),
        ("limit", params.limit.to_string()),
    ];

    if let Some(token_pair) = params.token_pair {
        url_params.push(("token_pair", token_pair));
    }

    let query_string = url_params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");

    let url = format!("https://statistic.7k.ag/trading-history?{}", query_string);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch swap history"));
    }

    let history = response.json::<TradingHistoryResponse>().await?;
    Ok(history)
}
