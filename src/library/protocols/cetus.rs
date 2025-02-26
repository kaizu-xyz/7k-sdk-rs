use super::get_default_sqrt_price_limit;
use crate::{
    destruct,
    library::swap_with_route::{Swapper, ToTypeTags},
    utils::sui::{ArgumentExt, ObjectRefFetcher},
};
use anyhow::Result;
use std::str::FromStr;
use sui_sdk::types::{base_types::SuiAddress, transaction::Argument};

pub async fn swap<'a>(swapper: &mut Swapper<'a>) -> Result<Argument> {
    let sqrt_price_limit = if swapper.swap.swap_x_to_y {
        get_default_sqrt_price_limit(true)
    } else {
        get_default_sqrt_price_limit(false)
    };

    let type_arguments = vec![
        swapper.swap.swap.asset_in.as_str(),
        swapper.swap.swap.asset_out.as_str(),
    ]
    .to_type_tags()?;

    let zero_out = swapper.tx.zero_coin(if swapper.swap.swap_x_to_y {
        swapper.swap.swap.asset_out.as_str()
    } else {
        swapper.swap.swap.asset_in.as_str()
    })?;

    let amount_in = swapper.get_input_coin_value()?;

    let c = &swapper.client;
    let cfg = &swapper.config.cetus;

    // Get object references
    let global_config = c.shared_obj_mut(&cfg.global_config).await?;
    let pool_id = c.shared_obj_mut(&swapper.swap.swap.pool_id).await?;

    let args = vec![
        swapper.tx.obj(global_config)?,
        swapper.tx.obj(pool_id)?,
        if swapper.swap.swap_x_to_y {
            *swapper.input_coin_object
        } else {
            zero_out.clone()
        },
        if swapper.swap.swap_x_to_y {
            zero_out.clone()
        } else {
            *swapper.input_coin_object
        },
        swapper.tx.pure(swapper.swap.swap_x_to_y)?,
        swapper.tx.pure(true)?,
        amount_in,
        swapper.tx.pure(sqrt_price_limit.to_string())?,
        swapper.tx.pure(false)?,
        swapper.tx.clock()?,
    ];

    let res = swapper
        .tx
        .move_call(
            &swapper.config.cetus.base.package,
            "router",
            "swap",
            type_arguments,
            args,
        )?
        .split(2)?;

    let (receive_a, receive_b) = destruct!(2, res);

    // Transfer or destroy zero coin
    swapper.tx.transfer_or_destroy_zero_coin(
        &swapper.swap.swap.asset_in,
        if swapper.swap.swap_x_to_y {
            receive_a
        } else {
            receive_b
        },
        Some(SuiAddress::from_str(&swapper.current_account)?),
    )?;

    Ok(if swapper.swap.swap_x_to_y {
        receive_b
    } else {
        receive_a
    })
}
