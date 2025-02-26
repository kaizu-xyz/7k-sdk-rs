use crate::{consts::SUI_FULL_TYPE, types::token::SuiscanToken};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const MAX_TOTAL_IDS: usize = 500;
const MAX_IDS_PER_REQUEST: usize = 100;
const PRICES_API: &str = "https://prices.7k.ag";
const NATIVE_USDC_TOKEN_TYPE: &str =
    "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPrice {
    pub token_type: String,
    pub price: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub volume_24h: Option<f64>,
    pub market_cap: Option<f64>,
}

impl From<SuiscanToken> for TokenPrice {
    fn from(token: SuiscanToken) -> Self {
        Self {
            token_type: token.token_type,
            price: token.token_price,
            price_change_24h: None, // Not available in SuiscanToken
            volume_24h: token.total_volume,
            market_cap: token.market_cap,
        }
    }
}

pub async fn get_token_price(id: &str) -> Result<f64> {
    let client = Client::new();
    let url = format!(
        "{}/price?ids={}&vsCoin={}",
        PRICES_API, id, NATIVE_USDC_TOKEN_TYPE
    );
    let response = client.get(&url).send().await?;
    let prices_res: serde_json::Value = response.json().await?;
    Ok(prices_res[id]["price"].as_f64().unwrap())
}

pub async fn get_token_prices(
    ids: Vec<String>,
    vs_coin: &str,
) -> Result<std::collections::HashMap<String, f64>> {
    let limited_ids: Vec<String> = ids.into_iter().take(MAX_TOTAL_IDS).collect();
    let id_chunks: Vec<Vec<String>> = limited_ids
        .chunks(MAX_IDS_PER_REQUEST)
        .map(|chunk| chunk.to_vec())
        .collect();

    let client = Client::new();
    let mut responses = Vec::new();

    for chunk in id_chunks {
        let response = client
            .post(&format!("{}/price", PRICES_API))
            .json(&serde_json::json!({ "ids": chunk, "vsCoin": vs_coin }))
            .send()
            .await?;
        let prices_res: std::collections::HashMap<String, TokenPrice> = response.json().await?;
        responses.push(prices_res);
    }

    let mut combined_prices = std::collections::HashMap::new();
    for prices_res in responses {
        for (id, token_price) in prices_res {
            combined_prices.insert(id, token_price.price.unwrap_or(0.0));
        }
    }

    let final_prices =
        limited_ids
            .into_iter()
            .fold(std::collections::HashMap::new(), |mut acc, id| {
                acc.insert(id.clone(), *combined_prices.get(&id).unwrap_or(&0.0));
                acc
            });

    Ok(final_prices)
}

// pub async fn get_token_prices(token_types: Vec<String>) -> Result<Vec<TokenPrice>> {
//     let mut prices = Vec::new();
//     for token_type in token_types {
//         if let Ok(price) = get_token_price(&token_type).await {
//             prices.push(price);
//         }
//     }
//     Ok(prices)
// }

pub async fn get_sui_price() -> Result<f64> {
    let token_price = get_token_price(&SUI_FULL_TYPE).await?;
    Ok(token_price)
}
