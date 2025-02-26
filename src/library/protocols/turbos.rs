#![allow(dead_code)]

use super::get_default_sqrt_price_limit;
use crate::{
    destruct,
    library::swap_with_route::{Swapper, ToTypeTags},
    utils::{
        sui::{ArgumentExt, ObjectRefFetcher},
        token::normalize_token_type,
    },
};
use anyhow::Result;
use std::str::FromStr;
use sui_sdk::types::{TypeTag, base_types::SuiAddress, transaction::Argument};

const MAX_TICK_STEP: u64 = 100;
const MAX_TICK_INDEX: u64 = 443636;
const MIN_TICK_INDEX: i64 = -443636;
const ONE_MINUTE: u64 = 60 * 1000;

pub async fn swap<'a>(swapper: &mut Swapper<'a>) -> Result<Argument> {
    let swap_x_to_y = swapper.swap.swap_x_to_y;
    let type_args_string = swapper.get_type_params()?;

    let type_args = type_args_string
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>();
    let amount_in = swapper.get_input_coin_value()?;

    let coin_x_type = normalize_token_type(&swapper.swap.coin_x.coin_type);
    let coin_y_type = normalize_token_type(&swapper.swap.coin_y.coin_type);
    let input_coin_type = if swap_x_to_y {
        TypeTag::from_str(coin_x_type)
    } else {
        TypeTag::from_str(coin_y_type)
    }?;

    let c = &swapper.client;
    let cfg = &swapper.config.turbos;
    let pool_id = c.shared_obj_mut(&swapper.swap.swap.pool_id).await?;
    let version = c.shared_obj_mut(cfg.version.as_str()).await?;

    let args = vec![
        swapper.tx.obj(pool_id)?,
        swapper
            .tx
            .make_move_vec(input_coin_type, vec![*swapper.input_coin_object])?,
        swapper.tx.pure(amount_in)?,
        swapper.tx.pure(0u64)?,
        swapper.tx.pure(get_default_sqrt_price_limit(swap_x_to_y))?,
        swapper.tx.pure(true)?,
        swapper
            .tx
            .pure(SuiAddress::from_str(swapper.current_account)?)?,
        swapper
            .tx
            .pure(chrono::Utc::now().timestamp_millis() as u64 + ONE_MINUTE * 3)?,
        swapper.tx.clock()?,
        swapper.tx.obj(version)?,
    ];

    let res = swapper.tx.move_call(
        &cfg.base.package,
        "amm",
        format!(
            "swap_{}_with_return",
            if swap_x_to_y { "a_b" } else { "b_a" }
        )
        .as_str(),
        type_args.to_type_tags()?,
        args,
    )?;

    let (token_out, token_in) = destruct!(2, res.split(2)?);

    swapper.tx.transfer_or_destroy_zero_coin(
        &swapper.swap.swap.asset_in,
        token_in,
        Some(SuiAddress::from_str(swapper.current_account)?),
    )?;

    Ok(token_out)
}

// fn get_default_sqrt_price_limit(swap_x_to_y: bool) -> u128 {
//     if swap_x_to_y {
//         1u128
//     } else {
//         u128::MAX - 1
//     }
// }
