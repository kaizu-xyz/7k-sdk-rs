use anyhow::Result;
use sui_sdk::types::transaction::Argument;

use crate::{
    library::swap_with_route::{Swapper, ToTypeTags},
    utils::{
        sui::{ObjectArgExt, ObjectRefFetcher},
        token::normalize_token_type,
    },
};

const PACKAGE_ID: &str = "0xa0eba10b173538c8fecca1dff298e488402cc9ff374f8a12ca7758eebe830b66";
const MODULE_NAME: &str = "spot_dex";

pub async fn swap<'a>(swapper: &mut Swapper<'a>) -> Result<Argument> {
    let swap_x_to_y = swapper.swap.swap_x_to_y;
    let coin_in_type = normalize_token_type(&swapper.swap.swap.asset_in);
    let coin_out_type = normalize_token_type(&swapper.swap.swap.asset_out);
    let pool_id = swapper.swap.swap.pool_id.clone();
    let input_coin_object = swapper.input_coin_object;

    let type_tags = if swap_x_to_y {
        vec![coin_in_type, coin_out_type]
    } else {
        vec![coin_out_type, coin_in_type]
    }
    .to_type_tags()?;

    let pool_obj = swapper.client.object_ref(&pool_id).await?.shared_obj(true);

    let args = vec![
        swapper.tx.obj(pool_obj)?,
        *input_coin_object,
        swapper.get_input_coin_value()?,
        swapper.tx.pure(0u64)?,
    ];

    let token_out = swapper.tx.move_call(
        PACKAGE_ID,
        MODULE_NAME,
        if swap_x_to_y {
            "swap_token_x"
        } else {
            "swap_token_y"
        },
        type_tags,
        args,
    )?;

    Ok(token_out)
}
