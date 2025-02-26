use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SourceDex {
    Suiswap,
    Turbos,
    Cetus,
    Bluemove,
    Kriya,
    KriyaV3,
    Aftermath,
    Deepbook,
    DeepbookV3,
    Flowx,
    FlowxV3,
    Bluefin,
    Springsui,
    Obric,
    Stsui,
}

impl SourceDex {
    pub fn as_str(&self) -> &str {
        match self {
            SourceDex::Suiswap => "suiswap",
            SourceDex::Turbos => "turbos",
            SourceDex::Cetus => "cetus",
            SourceDex::Bluemove => "bluemove",
            SourceDex::Kriya => "kriya",
            SourceDex::KriyaV3 => "kriya_v3",
            SourceDex::Aftermath => "aftermath",
            SourceDex::Deepbook => "deepbook",
            SourceDex::DeepbookV3 => "deepbook_v3",
            SourceDex::Flowx => "flowx",
            SourceDex::FlowxV3 => "flowx_v3",
            SourceDex::Bluefin => "bluefin",
            SourceDex::Springsui => "springsui",
            SourceDex::Obric => "obric",
            SourceDex::Stsui => "stsui",
        }
    }
}

impl Serialize for SourceDex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for SourceDex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "suiswap" => Ok(SourceDex::Suiswap),
            "turbos" => Ok(SourceDex::Turbos),
            "cetus" => Ok(SourceDex::Cetus),
            "bluemove" => Ok(SourceDex::Bluemove),
            "kriya" => Ok(SourceDex::Kriya),
            "kriya_v3" => Ok(SourceDex::KriyaV3),
            "aftermath" => Ok(SourceDex::Aftermath),
            "deepbook" => Ok(SourceDex::Deepbook),
            "deepbook_v3" => Ok(SourceDex::DeepbookV3),
            "flowx" => Ok(SourceDex::Flowx),
            "flowx_v3" => Ok(SourceDex::FlowxV3),
            "bluefin" => Ok(SourceDex::Bluefin),
            "springsui" => Ok(SourceDex::Springsui),
            "obric" => Ok(SourceDex::Obric),
            "stsui" => Ok(SourceDex::Stsui),
            _ => Err(serde::de::Error::custom(format!(
                "unknown SourceDex value: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SorSwap {
    pub pool_id: String,
    pub asset_in_index: u64,
    pub asset_out_index: u64,
    pub amount: String,
    pub return_amount: String,
    pub asset_in: String,
    pub asset_out: String,
    #[serde(default)]
    pub function_name: String,
    #[serde(default)]
    pub arguments: Vec<String>,
    pub extra: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub address: String,
    pub decimal: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SorPool {
    pub all_tokens: Vec<TokenInfo>,
    #[serde(rename = "type")]
    pub pool_type: SourceDex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SorHop {
    pub pool_id: String,
    pub token_in_amount: String,
    pub token_out_amount: String,
    pub token_in: String,
    pub token_out: String,
    pub pool: SorPool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SorRoute {
    pub hops: Vec<SorHop>,
    pub share: Option<f64>,
    pub token_in: String,
    pub token_in_amount: String,
    pub token_out: String,
    pub token_out_amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    pub effective_price: Option<f64>,
    pub effective_price_reserved: Option<f64>,
    pub price_impact: Option<f64>,
    pub swap_amount: String,
    pub return_amount: String,
    pub return_amount_with_decimal: String,
    pub return_amount_consider_gas_fees: Option<String>,
    pub return_amount_without_swap_fees: Option<String>,
    pub swap_amount_with_decimal: String,
    pub token_addresses: Vec<String>,
    pub token_in: String,
    pub token_out: String,
    pub market_sp: String,
    pub routes: Option<Vec<SorRoute>>,
    pub swaps: Vec<SorSwap>,
    pub warning: String,
}

#[derive(Debug, Clone)]
pub struct Coin {
    pub coin_type: String,
    pub decimals: u8,
}

#[derive(Debug, Clone)]
pub struct TxSorSwap {
    #[allow(dead_code)]
    pub swap: SorSwap,
    pub pool: SorPool,
    pub coin_x: Coin,
    pub coin_y: Coin,
    pub swap_x_to_y: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commission {
    pub partner: String,
    pub commission_bps: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DexConfig {
    pub package: String,
    pub name: String,
    pub url: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub aftermath: AftermathConfig,
    pub bluefin: BluefinConfig,
    pub bluemove: BluemoveConfig,
    pub cetus: CetusConfig,
    pub deepbook: DexConfig,
    pub deepbook_v3: DeepbookV3Config,
    pub flowx: FlowxConfig,
    pub flowx_v3: FlowxV3Config,
    pub kriya: DexConfig,
    pub kriya_v3: KriyaV3Config,
    pub obric: ObricConfig,
    pub springsui: DexConfig,
    pub stsui: DexConfig,
    pub suiswap: DexConfig,
    pub turbos: TurbosConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AftermathConfig {
    pub base: DexConfig,
    pub pool_registry: String,
    pub protocol_fee_vault: String,
    pub treasury: String,
    pub insurance_fund: String,
    pub referral_vault: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluefinConfig {
    pub base: DexConfig,
    pub global_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluemoveConfig {
    pub base: DexConfig,
    pub dex_info: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CetusConfig {
    pub base: DexConfig,
    pub global_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepbookV3Config {
    pub base: DexConfig,
    pub sponsor: String,
    pub sponsor_fund: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowxConfig {
    pub base: DexConfig,
    pub container: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowxV3Config {
    pub base: DexConfig,
    pub registry: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KriyaV3Config {
    pub base: DexConfig,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObricConfig {
    pub base: DexConfig,
    pub pyth_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurbosConfig {
    pub base: DexConfig,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpringsuiConfig {
    pub base: DexConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StsuiConfig {
    pub base: DexConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiswapConfig {
    pub base: DexConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_response_serde() {
        let json_data = r#"{
            "tokenAddresses": [
                "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI"
            ],
            "swaps": [{
                "poolId": "0xd8c0ba598973b30e3e74e48548397bc4899d41125996ed4080752bbdf34af1e9",
                "assetInIndex": 1,
                "assetOutIndex": 0,
                "amount": "1000000000",
                "returnAmount": "3039844",
                "assetIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                "assetOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                "functionName": "",
                "arguments": [],
                "extra": {
                    "x_price_id": "0x801dbc2f0053d34734814b2d6df491ce7807a725fe9a01ad74a07e9c51396c37",
                    "y_price_id": "0x5dec622733a204ca27f5a90d8c2fad453cc6665186fd5dff13a83d0b6c9027ab"
                }
            }],
            "swapAmount": "1",
            "returnAmount": "3.039844",
            "swapAmountWithDecimal": "1000000000",
            "returnAmountWithDecimal": "3039844",
            "tokenIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
            "tokenOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
            "marketSp": "0.328799841123916768",
            "routes": [{
                "hops": [{
                    "poolId": "0xd8c0ba598973b30e3e74e48548397bc4899d41125996ed4080752bbdf34af1e9",
                    "pool": {
                        "allTokens": [{
                            "address": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                            "decimal": 9
                        }, {
                            "address": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                            "decimal": 6
                        }],
                        "type": "obric"
                    },
                    "tokenIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                    "tokenInAmount": "1",
                    "tokenOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                    "tokenOutAmount": "3.039844"
                }],
                "tokenIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                "tokenInAmount": "1",
                "tokenOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                "tokenOutAmount": "3.039844"
            }],
            "effectivePrice": 0.3289642494812234,
            "effectivePriceReserved": 3.039844,
            "priceImpact": 0.0004997757585083563,
            "warning": "None"
        }"#;

        // Test deserialization
        let quote: QuoteResponse = serde_json::from_str(json_data).expect("Failed to deserialize");

        // Verify some key fields
        assert_eq!(quote.token_addresses.len(), 2);
        assert_eq!(quote.swaps.len(), 1);
        assert_eq!(
            quote.swaps[0].pool_id,
            "0xd8c0ba598973b30e3e74e48548397bc4899d41125996ed4080752bbdf34af1e9"
        );
        assert!(quote.routes.is_some());
        assert_eq!(
            quote.routes.as_ref().unwrap()[0].hops[0].pool.pool_type,
            SourceDex::Obric
        );

        // Test serialization
        let serialized = serde_json::to_string(&quote).expect("Failed to serialize");
        let deserialized: QuoteResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        // Verify round-trip
        assert_eq!(quote.token_addresses, deserialized.token_addresses);
        assert_eq!(quote.swaps[0].pool_id, deserialized.swaps[0].pool_id);
        assert_eq!(quote.effective_price, deserialized.effective_price);
    }

    #[test]
    fn test_quote_response_serde_2() {
        let json_data = r#"{
            "tokenAddresses": [
                "0xe44df51c0b21a27ab915fa1fe2ca610cd3eaa6d9666fe5e62b988bf7f0bd8722::musd::MUSD",
                "0x549e8b69270defbfafd4f94e17ec44cdbdd99820b33bda2278dea3b9a32d3f55::cert::CERT",
                "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI"
            ],
            "swaps": [
                {
                "poolId": "0x6c545e78638c8c1db7a48b282bb8ca79da107993fcb185f75cedc1f5adb2f535",
                "assetInIndex": 3,
                "assetOutIndex": 1,
                "amount": "389041257",
                "returnAmount": "373920709",
                "assetIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                "assetOut": "0x549e8b69270defbfafd4f94e17ec44cdbdd99820b33bda2278dea3b9a32d3f55::cert::CERT",
                "extra": {
                    "poolStructTag": "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::Pool<0x549e8b69270defbfafd4f94e17ec44cdbdd99820b33bda2278dea3b9a32d3f55::cert::CERT,0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI>"
                }
                },
                {
                "poolId": "0x8f3c373b2a66cfee321768695807e8d823c5a91af184ceaf9310bd12c304b981",
                "assetInIndex": 1,
                "assetOutIndex": 2,
                "amount": "0",
                "returnAmount": "1173666",
                "assetIn": "0x549e8b69270defbfafd4f94e17ec44cdbdd99820b33bda2278dea3b9a32d3f55::cert::CERT",
                "assetOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                "extra": {
                    "poolStructTag": "0x3492c874c1e3b3e2984e8c41b589e642d4d0a5d6459e5a9cfc2d52fd7c89c267::pool::Pool<0x549e8b69270defbfafd4f94e17ec44cdbdd99820b33bda2278dea3b9a32d3f55::cert::CERT,0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC>"
                }
                },
                {
                "poolId": "0x1adb343ab351458e151bc392fbf1558b3332467f23bda45ae67cd355a57fd5f5",
                "assetInIndex": 3,
                "assetOutIndex": 4,
                "amount": "382146692",
                "returnAmount": "380055633",
                "assetIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                "assetOut": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                "extra": null
                },
                {
                "poolId": "0x0da4bcb1669ae3b6ce80f024e3a2076e2c4e2cc899d4724fce94da0f729bc968",
                "assetInIndex": 4,
                "assetOutIndex": 0,
                "amount": "0",
                "returnAmount": "1151002194",
                "assetIn": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                "assetOut": "0xe44df51c0b21a27ab915fa1fe2ca610cd3eaa6d9666fe5e62b988bf7f0bd8722::musd::MUSD",
                "extra": {
                    "poolStructTag": "0x3492c874c1e3b3e2984e8c41b589e642d4d0a5d6459e5a9cfc2d52fd7c89c267::pool::Pool<0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI,0xe44df51c0b21a27ab915fa1fe2ca610cd3eaa6d9666fe5e62b988bf7f0bd8722::musd::MUSD>"
                }
                },
                {
                "poolId": "0xa6f9e2f3a25ab5befcef1459d5bd72c0e6f9678b225b395e35319db86bf58d6d",
                "assetInIndex": 0,
                "assetOutIndex": 2,
                "amount": "0",
                "returnAmount": "1150205",
                "assetIn": "0xe44df51c0b21a27ab915fa1fe2ca610cd3eaa6d9666fe5e62b988bf7f0bd8722::musd::MUSD",
                "assetOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                "extra": {
                    "poolStructTag": "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::Pool<0xe44df51c0b21a27ab915fa1fe2ca610cd3eaa6d9666fe5e62b988bf7f0bd8722::musd::MUSD,0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC>"
                }
                },
                {
                "poolId": "0x1adb343ab351458e151bc392fbf1558b3332467f23bda45ae67cd355a57fd5f5",
                "assetInIndex": 3,
                "assetOutIndex": 4,
                "amount": "197601241",
                "returnAmount": "196519992",
                "assetIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                "assetOut": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                "extra": null
                },
                {
                "poolId": "0x94bba65b79df597d954ff8357f3180d95371d3049c4b953c5d3f7b79c4c5b6c5",
                "assetInIndex": 4,
                "assetOutIndex": 2,
                "amount": "0",
                "returnAmount": "595682",
                "assetIn": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                "assetOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                "extra": {
                    "poolStructTag": "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::Pool<0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC,0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI>"
                }
                },
                {
                "poolId": "0x1adb343ab351458e151bc392fbf1558b3332467f23bda45ae67cd355a57fd5f5",
                "assetInIndex": 3,
                "assetOutIndex": 4,
                "amount": "31210809",
                "returnAmount": "31040027",
                "assetIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                "assetOut": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                "extra": null
                },
                {
                "poolId": "0x11dbeb991fea4b2c7efcba4b6b21cbecd3f94be99ec8c9205839eaf03356d358",
                "assetInIndex": 4,
                "assetOutIndex": 2,
                "amount": "0",
                "returnAmount": "93970",
                "assetIn": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                "assetOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                "extra": {
                    "poolStructTag": "0x3492c874c1e3b3e2984e8c41b589e642d4d0a5d6459e5a9cfc2d52fd7c89c267::pool::Pool<0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI,0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC>"
                }
                }
            ],
            "swapAmount": "1",
            "returnAmount": "3.013523",
            "swapAmountWithDecimal": "1000000000",
            "returnAmountWithDecimal": "3013523",
            "tokenIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
            "tokenOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
            "marketSp": "0.329714775804629968",
            "routes": [
                {
                "hops": [
                    {
                    "poolId": "0x6c545e78638c8c1db7a48b282bb8ca79da107993fcb185f75cedc1f5adb2f535",
                    "pool": {
                        "allTokens": [
                        {
                            "address": "0x549e8b69270defbfafd4f94e17ec44cdbdd99820b33bda2278dea3b9a32d3f55::cert::CERT",
                            "decimal": 9
                        },
                        {
                            "address": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                            "decimal": 9
                        }
                        ],
                        "type": "cetus"
                    },
                    "tokenIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                    "tokenInAmount": "0.389041257",
                    "tokenOut": "0x549e8b69270defbfafd4f94e17ec44cdbdd99820b33bda2278dea3b9a32d3f55::cert::CERT",
                    "tokenOutAmount": "0.373920709"
                    },
                    {
                    "poolId": "0x8f3c373b2a66cfee321768695807e8d823c5a91af184ceaf9310bd12c304b981",
                    "pool": {
                        "allTokens": [
                        {
                            "address": "0x549e8b69270defbfafd4f94e17ec44cdbdd99820b33bda2278dea3b9a32d3f55::cert::CERT",
                            "decimal": 9
                        },
                        {
                            "address": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                            "decimal": 6
                        }
                        ],
                        "type": "bluefin"
                    },
                    "tokenIn": "0x549e8b69270defbfafd4f94e17ec44cdbdd99820b33bda2278dea3b9a32d3f55::cert::CERT",
                    "tokenInAmount": "0.373920709",
                    "tokenOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                    "tokenOutAmount": "1.173666"
                    }
                ],
                "tokenIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                "tokenInAmount": "0.389041257",
                "tokenOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                "tokenOutAmount": "1.173666"
                },
                {
                "hops": [
                    {
                    "poolId": "0x1adb343ab351458e151bc392fbf1558b3332467f23bda45ae67cd355a57fd5f5",
                    "pool": {
                        "allTokens": [
                        {
                            "address": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                            "decimal": 9
                        },
                        {
                            "address": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                            "decimal": 9
                        }
                        ],
                        "type": "stsui"
                    },
                    "tokenIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                    "tokenInAmount": "0.382146692",
                    "tokenOut": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                    "tokenOutAmount": "0.380055633"
                    },
                    {
                    "poolId": "0x0da4bcb1669ae3b6ce80f024e3a2076e2c4e2cc899d4724fce94da0f729bc968",
                    "pool": {
                        "allTokens": [
                        {
                            "address": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                            "decimal": 9
                        },
                        {
                            "address": "0xe44df51c0b21a27ab915fa1fe2ca610cd3eaa6d9666fe5e62b988bf7f0bd8722::musd::MUSD",
                            "decimal": 9
                        }
                        ],
                        "type": "bluefin"
                    },
                    "tokenIn": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                    "tokenInAmount": "0.380055633",
                    "tokenOut": "0xe44df51c0b21a27ab915fa1fe2ca610cd3eaa6d9666fe5e62b988bf7f0bd8722::musd::MUSD",
                    "tokenOutAmount": "1.151002194"
                    },
                    {
                    "poolId": "0xa6f9e2f3a25ab5befcef1459d5bd72c0e6f9678b225b395e35319db86bf58d6d",
                    "pool": {
                        "allTokens": [
                        {
                            "address": "0xe44df51c0b21a27ab915fa1fe2ca610cd3eaa6d9666fe5e62b988bf7f0bd8722::musd::MUSD",
                            "decimal": 9
                        },
                        {
                            "address": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                            "decimal": 6
                        }
                        ],
                        "type": "cetus"
                    },
                    "tokenIn": "0xe44df51c0b21a27ab915fa1fe2ca610cd3eaa6d9666fe5e62b988bf7f0bd8722::musd::MUSD",
                    "tokenInAmount": "1.151002194",
                    "tokenOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                    "tokenOutAmount": "1.150205"
                    }
                ],
                "tokenIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                "tokenInAmount": "0.382146692",
                "tokenOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                "tokenOutAmount": "1.150205"
                },
                {
                "hops": [
                    {
                    "poolId": "0x1adb343ab351458e151bc392fbf1558b3332467f23bda45ae67cd355a57fd5f5",
                    "pool": {
                        "allTokens": [
                        {
                            "address": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                            "decimal": 9
                        },
                        {
                            "address": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                            "decimal": 9
                        }
                        ],
                        "type": "stsui"
                    },
                    "tokenIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                    "tokenInAmount": "0.197601241",
                    "tokenOut": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                    "tokenOutAmount": "0.196519992"
                    },
                    {
                    "poolId": "0x94bba65b79df597d954ff8357f3180d95371d3049c4b953c5d3f7b79c4c5b6c5",
                    "pool": {
                        "allTokens": [
                        {
                            "address": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                            "decimal": 6
                        },
                        {
                            "address": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                            "decimal": 9
                        }
                        ],
                        "type": "cetus"
                    },
                    "tokenIn": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                    "tokenInAmount": "0.196519992",
                    "tokenOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                    "tokenOutAmount": "0.595682"
                    }
                ],
                "tokenIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                "tokenInAmount": "0.197601241",
                "tokenOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                "tokenOutAmount": "0.595682"
                },
                {
                "hops": [
                    {
                    "poolId": "0x1adb343ab351458e151bc392fbf1558b3332467f23bda45ae67cd355a57fd5f5",
                    "pool": {
                        "allTokens": [
                        {
                            "address": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                            "decimal": 9
                        },
                        {
                            "address": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                            "decimal": 9
                        }
                        ],
                        "type": "stsui"
                    },
                    "tokenIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                    "tokenInAmount": "0.031210809",
                    "tokenOut": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                    "tokenOutAmount": "0.031040027"
                    },
                    {
                    "poolId": "0x11dbeb991fea4b2c7efcba4b6b21cbecd3f94be99ec8c9205839eaf03356d358",
                    "pool": {
                        "allTokens": [
                        {
                            "address": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                            "decimal": 9
                        },
                        {
                            "address": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                            "decimal": 6
                        }
                        ],
                        "type": "bluefin"
                    },
                    "tokenIn": "0xd1b72982e40348d069bb1ff701e634c117bb5f741f44dff91e472d3b01461e55::stsui::STSUI",
                    "tokenInAmount": "0.031040027",
                    "tokenOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                    "tokenOutAmount": "0.09397"
                    }
                ],
                "tokenIn": "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                "tokenInAmount": "0.031210809",
                "tokenOut": "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                "tokenOutAmount": "0.09397"
                }
            ],
            "effectivePrice": 0.33183752040385955,
            "effectivePriceReserved": 3.013523,
            "priceImpact": 0.006396939672904085,
            "warning": "None"
            }"#;

        // Test deserialization
        let quote: QuoteResponse = serde_json::from_str(json_data).expect("Failed to deserialize");

        // Test serialization
        let serialized = serde_json::to_string(&quote).expect("Failed to serialize");
        let _deserialized: QuoteResponse =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
    }
}
