use anyhow::Result;
use sui_sdk::types::transaction::Argument;

use crate::{
    library::swap_with_route::{Swapper, ToTypeTags},
    utils::{
        sui::{ObjectArgExt, ObjectRefFetcher},
        token::normalize_token_type,
    },
};

const PACKAGE_ID: &str = "0x059f94b85c07eb74d2847f8255d8cc0a67c9a8dcc039eabf9f8b9e23a0de2700";
const SUI_SYSTEM_STATE_OBJECT_ID: &str = "0x2::system_state::SystemState";

pub async fn swap<'a>(swapper: &mut Swapper<'a>) -> Result<Argument> {
    let pool_tokens = &swapper.swap.pool.all_tokens;
    let is_stake = swapper.swap.swap_x_to_y;
    let coin_y_type = normalize_token_type(&pool_tokens[1].address);

    let type_tags = vec![coin_y_type].to_type_tags()?;

    let pool_obj = swapper
        .client
        .object_ref(&swapper.swap.swap.pool_id)
        .await?
        .shared_obj(true);
    let system_state_obj = swapper
        .client
        .object_ref(SUI_SYSTEM_STATE_OBJECT_ID)
        .await?
        .shared_obj(true);

    let args = if is_stake {
        vec![
            swapper.tx.obj(pool_obj)?,
            swapper.tx.obj(system_state_obj)?,
            *swapper.input_coin_object,
        ]
    } else {
        vec![
            swapper.tx.obj(pool_obj)?,
            *swapper.input_coin_object,
            swapper.tx.obj(system_state_obj)?,
        ]
    };

    let token_out = swapper.tx.move_call(
        PACKAGE_ID,
        "liquid_staking",
        if is_stake { "mint" } else { "redeem" },
        type_tags,
        args,
    )?;

    Ok(token_out)
}
