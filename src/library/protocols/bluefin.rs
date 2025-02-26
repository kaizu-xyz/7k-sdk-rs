use super::get_adjusted_sqrt_price_limit;
use crate::{
    destruct,
    library::swap_with_route::{Swapper, ToTypeTags},
    utils::sui::{ArgumentExt, ObjectRefFetcher},
};
use anyhow::{Result, anyhow};
use std::str::FromStr;
use sui_sdk::types::{base_types::SuiAddress, transaction::Argument};

pub async fn swap<'a>(swapper: &mut Swapper<'a>) -> Result<Argument> {
    let pool_id = swapper.swap.swap.pool_id.clone();

    let coins = &swapper.swap.pool.all_tokens;
    let (coin_x, coin_y) = match coins.as_slice() {
        [coin_x, coin_y] => (coin_x.clone(), coin_y.clone()),
        _ => return Err(anyhow!("Expected exactly two coins")),
    };

    let swap_x_to_y = swapper.swap.swap_x_to_y;
    let amount_in = swapper.get_input_coin_value()?;
    let c = &swapper.client;

    let coin_in = c.owned_obj(&swapper.swap.swap.asset_in).await?;

    let coin_in_obj = swapper.tx.obj(coin_in)?;
    let balance_in = swapper
        .tx
        .coin_into_balance(&swapper.swap.swap.asset_in, coin_in_obj)?;
    let balance_out = swapper.tx.zero_balance(&swapper.swap.swap.asset_out)?;

    let cfg = &swapper.config.bluefin;

    let config = c.shared_obj_mut(&cfg.global_config).await?;
    let pool_obj = c.shared_obj_mut(&pool_id).await?;
    let bal_1 = if swap_x_to_y { balance_in } else { balance_out };
    let bal_2 = if swap_x_to_y { balance_out } else { balance_in };

    let type_tags = vec![coin_x.address.as_str(), coin_y.address.as_str()].to_type_tags()?;
    let args = vec![
        swapper.tx.clock()?,
        swapper.tx.obj(config)?,
        swapper.tx.obj(pool_obj)?,
        bal_1,
        bal_2,
        swapper.tx.pure(swap_x_to_y)?,
        swapper.tx.pure(true)?,
        amount_in,
        swapper.tx.pure(0)?,
        swapper
            .tx
            .pure(get_adjusted_sqrt_price_limit(swap_x_to_y))?,
    ];

    let res = swapper
        .tx
        .move_call(
            &swapper.config.aftermath.base.package,
            "pool",
            "swap",
            type_tags,
            args,
        )?
        .split(2)?;

    let (balance_out_x, balance_out_y) = destruct!(2, res);

    let coin_out_x = swapper
        .tx
        .coin_from_balance(&coin_x.address, balance_out_x)?;

    let coin_out_y = swapper
        .tx
        .coin_from_balance(&coin_y.address, balance_out_y)?;

    swapper.tx.transfer_or_destroy_zero_coin(
        &swapper.swap.swap.asset_in,
        if swap_x_to_y { coin_out_x } else { coin_out_y },
        Some(SuiAddress::from_str(&swapper.current_account)?),
    )?;

    Ok(if swap_x_to_y { coin_out_y } else { coin_out_x })
}
