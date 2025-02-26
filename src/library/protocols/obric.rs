use anyhow::Result;
use sui_sdk::types::transaction::Argument;

use crate::{
    library::swap_with_route::{Swapper, ToTypeTags},
    utils::{
        sui::{ObjectArgExt, ObjectRefFetcher},
        token::normalize_token_type,
    },
};

const PACKAGE_ID: &str = "0xb84e63d22ea4822a0a333c250e790f69bf5c2ef0c63f4e120e05a6415991368f";
const PYTH_STATE: &str = "0x1f9310238ee9298fb703c3419030b35b22bb1cc37113e3bb5007c99aec79e5b8";
const SUI_CLOCK_OBJECT_ID: &str = "0x2::clock::Clock";

pub async fn swap<'a>(swapper: &mut Swapper<'a>) -> Result<Argument> {
    let pool_tokens = &swapper.swap.pool.all_tokens;
    let x_to_y = swapper.swap.swap_x_to_y;
    let extra = swapper
        .swap
        .swap
        .extra
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("x_price_id and y_price_id are required"))?;
    let x_price_id = extra
        .get("x_price_id")
        .ok_or_else(|| anyhow::anyhow!("x_price_id is required"))?;
    let y_price_id = extra
        .get("y_price_id")
        .ok_or_else(|| anyhow::anyhow!("y_price_id is required"))?;

    let coin_x_type = normalize_token_type(&pool_tokens[0].address);
    let coin_y_type = normalize_token_type(&pool_tokens[1].address);

    let type_tags = vec![coin_x_type, coin_y_type].to_type_tags()?;

    let pool_obj = swapper
        .client
        .object_ref(&swapper.swap.swap.pool_id)
        .await?
        .shared_obj(true);
    let clock_obj = swapper
        .client
        .object_ref(SUI_CLOCK_OBJECT_ID)
        .await?
        .shared_obj(true);
    let pyth_state_obj = swapper
        .client
        .object_ref(PYTH_STATE)
        .await?
        .shared_obj(true);
    let x_price_obj = swapper
        .client
        .object_ref(x_price_id)
        .await?
        .shared_obj(true);
    let y_price_obj = swapper
        .client
        .object_ref(y_price_id)
        .await?
        .shared_obj(true);

    let args = vec![
        swapper.tx.obj(pool_obj)?,
        swapper.tx.obj(clock_obj)?,
        swapper.tx.obj(pyth_state_obj)?,
        swapper.tx.obj(x_price_obj)?,
        swapper.tx.obj(y_price_obj)?,
        *swapper.input_coin_object,
    ];

    let token_out = swapper.tx.move_call(
        PACKAGE_ID,
        "v2",
        if x_to_y { "swap_x_to_y" } else { "swap_y_to_x" },
        type_tags,
        args,
    )?;

    Ok(token_out)
}
