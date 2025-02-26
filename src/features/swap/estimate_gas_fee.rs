use std::str::FromStr;

use anyhow::Result;
use sui_sdk::SuiClient;
use sui_sdk::rpc_types::SuiTransactionBlockEffects;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::types::gas::GasCostSummary;

use crate::features::prices::get_sui_price;
use crate::features::swap::build_tx::build_tx;
use crate::types::tx::{BuildTxParams, CommonParams, EstimateGasFeeParams};
use crate::utils::token::format_balance;

use super::config::ConfigManager;

pub async fn estimate_gas_fee(
    client: &SuiClient,
    config_manager: &mut ConfigManager,
    params: EstimateGasFeeParams,
) -> Result<f64> {
    let EstimateGasFeeParams { common, sui_price } = params;
    let CommonParams {
        quote_response,
        account_address,
        slippage,
        extend_tx,
        commission,
    } = common;

    if account_address.is_empty() {
        return Ok(0.0);
    }

    let build_result = build_tx(
        client,
        config_manager,
        BuildTxParams {
            common: CommonParams {
                quote_response,
                account_address: account_address.clone(),
                slippage,
                extend_tx,
                commission,
            },
            dev_inspect: Some(true),
        },
    )
    .await;

    let (tx, _) = match build_result {
        Ok(result) => result,
        Err(err) => {
            println!("build tx error: {}", err);
            return Ok(0.0);
        }
    };

    let sui_price = match sui_price {
        Some(price) => price,
        None => get_sui_price().await?,
    };

    let sui_decimals = 9u32;

    let tx_payload = tx.complete();

    let dev_inspect = client
        .read_api()
        .dev_inspect_transaction_block(
            SuiAddress::from_str(&account_address)?,
            tx_payload,
            None,
            None,
            None,
        )
        .await?;

    let SuiTransactionBlockEffects::V1(effects) = &dev_inspect.effects;
    // let effects: &SuiTransactionBlockEffects = &dev_inspect.effects;
    if effects.status.is_err() {
        return Ok(0.0);
    }

    // let MyEnum::SingleVariant(inner) = value;

    let gas_used: &GasCostSummary = &effects.gas_used;
    let fee = gas_used.computation_cost + gas_used.storage_cost - gas_used.storage_rebate;

    let fee_balance = format_balance(fee, sui_decimals);
    let fee_usd = (fee_balance as f64) * sui_price;

    Ok(fee_usd)
}
