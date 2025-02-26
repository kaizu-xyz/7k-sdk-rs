use crate::types::aggregators::{QuoteResponse, SourceDex};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sui_sdk::types::base_types::ObjectID;

pub const DEFAULT_SOURCES: &[SourceDex] = &[
    SourceDex::Suiswap,
    SourceDex::Turbos,
    SourceDex::Cetus,
    SourceDex::Bluemove,
    SourceDex::Kriya,
    SourceDex::KriyaV3,
    SourceDex::Aftermath,
    SourceDex::Deepbook,
    SourceDex::DeepbookV3,
    SourceDex::Flowx,
    SourceDex::FlowxV3,
    SourceDex::Bluefin,
    SourceDex::Springsui,
    SourceDex::Obric,
    SourceDex::Stsui,
];

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GetQuoteParams {
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
    pub sources: Option<Vec<SourceDex>>,
    pub target_pools: Option<Vec<String>>,
    pub excluded_pools: Option<Vec<String>>,
}

pub async fn get_quote(params: GetQuoteParams) -> Result<QuoteResponse> {
    let GetQuoteParams {
        token_in,
        token_out,
        amount_in,
        sources,
        target_pools,
        excluded_pools,
    } = params;

    let sources = sources.unwrap_or_else(|| DEFAULT_SOURCES.to_vec());

    let mut url_params = vec![
        ("amount", amount_in),
        ("from", normalize_struct_tag(&token_in)),
        ("to", normalize_struct_tag(&token_out)),
        (
            "sources",
            sources
                .iter()
                .map(|s| s.as_str().to_string())
                .collect::<Vec<_>>()
                .join(","),
        ),
    ];

    if let Some(pools) = target_pools {
        url_params.push((
            "target_pools",
            pools
                .iter()
                .map(|p| normalize_sui_object_id(p))
                .collect::<Vec<_>>()
                .join(","),
        ));
    }

    if let Some(pools) = excluded_pools {
        url_params.push((
            "excluded_pools",
            pools
                .iter()
                .map(|p| normalize_sui_object_id(p))
                .collect::<Vec<_>>()
                .join(","),
        ));
    }

    let query_string = url_params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");

    let url = format!("https://api.7k.ag/quote?{}", query_string);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch aggregator quote"));
    }

    let json_value: Value = response.json().await?;
    println!("response: {}", serde_json::to_string_pretty(&json_value)?);

    let quote_response: QuoteResponse = serde_json::from_value(json_value)?;
    Ok(quote_response)
}

fn normalize_struct_tag(tag: &str) -> String {
    // Basic normalization - a more complete implementation would be needed
    tag.to_string()
}

fn normalize_sui_object_id(id: &str) -> String {
    // Basic normalization - a more complete implementation would be needed
    match ObjectID::from_hex_literal(id) {
        Ok(object_id) => object_id.to_string(),
        Err(_) => id.to_string(),
    }
}
