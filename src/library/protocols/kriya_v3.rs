use crate::{
    destruct,
    library::swap_with_route::{Swapper, ToTypeTags},
    utils::{
        sui::{ArgumentExt, ObjectRefFetcher},
        token::normalize_token_type,
    },
};
use anyhow::Result;
use sui_sdk::types::transaction::Argument;

const PACKAGE_ID: &str = "0xbd8d4489782042c6fafad4de4bc6a5e0b84a43c6c00647ffd7062d1e2bb7549e";
const VERSION_ID: &str = "0xf5145a7ac345ca8736cf8c76047d00d6d378f30e81be6f6eb557184d9de93c78";

pub async fn swap<'a>(swapper: &mut Swapper<'a>) -> Result<Argument> {
    let type_tags = swapper
        .get_type_params()?
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .to_type_tags()?;

    let swap_x_to_y = swapper.swap.swap_x_to_y;
    let pool_id = swapper.swap.swap.pool_id.clone();
    let low_limit_price = 4295048017u128;
    let limit_price = 79226673515401279992447579050u128;

    let c = &swapper.client;
    let pool_obj = c.shared_obj_mut(&pool_id).await?;
    let version_obj = c.shared_obj_mut(VERSION_ID).await?;
    let input_coin_value = swapper.get_input_coin_value()?;

    let args = vec![
        swapper.tx.obj(pool_obj)?,
        swapper.tx.pure(swap_x_to_y)?,
        swapper.tx.pure(true)?,
        input_coin_value,
        swapper.tx.pure(if swap_x_to_y {
            low_limit_price
        } else {
            limit_price
        })?,
        swapper.tx.clock()?,
        swapper.tx.obj(version_obj)?,
    ];

    let coin_in_type = normalize_token_type(&swapper.swap.swap.asset_in);
    let coin_out_type = normalize_token_type(&swapper.swap.swap.asset_out);

    let res = swapper
        .tx
        .move_call(PACKAGE_ID, "trade", "flash_swap", type_tags.clone(), args)?
        .split(2)?;

    let (receive_a, receive_b, flash_receipt) = destruct!(3, res);

    swapper.tx.move_call(
        "0x2",
        "balance",
        "destroy_zero",
        vec![coin_in_type].to_type_tags()?,
        vec![if swap_x_to_y { receive_a } else { receive_b }],
    )?;

    let zero_out_coin = swapper.tx.move_call(
        "0x2",
        "balance",
        "zero",
        vec![coin_out_type].to_type_tags()?,
        vec![],
    )?;

    let input_coin_balance = swapper.tx.move_call(
        "0x2",
        "coin",
        "into_balance",
        vec![coin_in_type].to_type_tags()?,
        vec![*swapper.input_coin_object],
    )?;

    let pay_coin_a = if swap_x_to_y {
        input_coin_balance
    } else {
        zero_out_coin
    };
    let pay_coin_b = if swap_x_to_y {
        zero_out_coin
    } else {
        input_coin_balance
    };

    let args = vec![
        swapper.tx.obj(pool_obj)?,
        flash_receipt,
        pay_coin_a,
        pay_coin_b,
        swapper.tx.obj(version_obj)?,
    ];

    swapper
        .tx
        .move_call(PACKAGE_ID, "trade", "repay_flash_swap", type_tags, args)?;

    let token_out = swapper.tx.move_call(
        "0x2",
        "coin",
        "from_balance",
        vec![coin_out_type].to_type_tags()?,
        vec![if swap_x_to_y { receive_b } else { receive_a }],
    )?;

    Ok(token_out)
}
