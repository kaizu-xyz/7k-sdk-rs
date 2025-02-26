use crate::{
    destruct,
    library::swap_with_route::{Swapper, ToTypeTags},
    utils::{
        sui::{ArgumentExt, ObjectArgExt, ObjectRefFetcher},
        token::normalize_token_type,
    },
};
use anyhow::Result;
use std::str::FromStr;
use sui_sdk::types::{TypeTag, base_types::SuiAddress, transaction::Argument};

const PACKAGE_ID: &str = "0xd075d51486df71e750872b4edf82ea3409fda397ceecc0b6aedf573d923c54a0";
const MODULE_NAME: &str = "pool";
const SUI_CLOCK_OBJECT_ID: &str = "0x2::clock::Clock";

pub async fn swap<'a>(swapper: &mut Swapper<'a>) -> Result<Argument> {
    let pool_id = swapper.swap.swap.pool_id.clone();
    let swap_x_to_y = swapper.swap.swap_x_to_y;
    let input_coin = swapper.input_coin_object;
    let coin_x_type = normalize_token_type(&swapper.swap.coin_x.coin_type);
    let coin_y_type = normalize_token_type(&swapper.swap.coin_y.coin_type);

    let input_coin_type = if swap_x_to_y {
        TypeTag::from_str(coin_x_type)
    } else {
        TypeTag::from_str(coin_y_type)
    }?;

    let type_arguments = vec![coin_x_type, coin_y_type].to_type_tags()?;
    let call_func = if swap_x_to_y {
        "do_swap_x_to_y_direct"
    } else {
        "do_swap_y_to_x_direct"
    };

    let pool_obj = swapper.client.object_ref(&pool_id).await?.shared_obj(true);
    let clock_obj = swapper
        .client
        .object_ref(SUI_CLOCK_OBJECT_ID)
        .await?
        .shared_obj(true);

    let input_amount = swapper.get_input_coin_value()?;

    let args = vec![
        swapper.tx.obj(pool_obj)?,
        swapper
            .tx
            .make_move_vec(input_coin_type, vec![*input_coin])?,
        input_amount,
        swapper.tx.obj(clock_obj)?,
    ];

    let res = swapper
        .tx
        .move_call(PACKAGE_ID, MODULE_NAME, call_func, type_arguments, args)?
        .split(2)?;

    let (token_in, token_out) = destruct!(2, res);

    // Transfer or destroy zero coin
    swapper.tx.transfer_or_destroy_zero_coin(
        &swapper.swap.swap.asset_in,
        token_in,
        Some(SuiAddress::from_str(&swapper.current_account)?),
    )?;

    Ok(token_out)
}
