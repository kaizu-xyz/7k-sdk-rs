use super::aggregators::{Commission, QuoteResponse};
use crate::utils::sui::Ptb;
use sui_sdk::types::transaction::Argument;

// #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonParams {
    pub quote_response: QuoteResponse,
    pub account_address: String,
    pub slippage: f64,
    pub commission: Commission,
    pub extend_tx: Option<ExtendTx>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendTx {
    pub tx: Ptb,
    pub coin_in: Option<Argument>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildTxParams {
    // #[serde(flatten)]
    pub common: CommonParams,
    pub dev_inspect: Option<bool>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimateGasFeeParams {
    // #[serde(flatten)]
    pub common: CommonParams,
    pub sui_price: Option<f64>,
}
