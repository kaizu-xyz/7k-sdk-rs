use std::str::FromStr;

use anyhow::Result;
use sui_sdk::types::{base_types::SuiAddress, transaction::Argument};

use crate::{
    destruct,
    library::swap_with_route::{Swapper, ToTypeTags},
    utils::{
        sui::{ArgumentExt, ObjectRefFetcher},
        token::normalize_token_type,
    },
};

pub async fn swap<'a>(swapper: &mut Swapper<'a>) -> Result<Argument> {
    let coin_types = swapper.get_type_params()?;
    let coin_x_type = coin_types.get(0).unwrap();
    let swap_x_to_y =
        normalize_token_type(&swapper.swap.swap.asset_in) == normalize_token_type(&coin_x_type);

    let input_coin_object = swapper.input_coin_object;

    let type_tags = swapper
        .get_type_params()?
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .to_type_tags()?;

    let c = &swapper.client;
    let cfg = &swapper.config.deepbook_v3;

    let sponsor_fund = c.shared_obj_mut(&cfg.sponsor_fund).await?;
    let pool_id = c.shared_obj_mut(swapper.swap.swap.pool_id.as_str()).await?;

    let args = vec![
        swapper.tx.obj(sponsor_fund)?,
        swapper.tx.obj(pool_id)?,
        *input_coin_object,
        swapper.tx.pure(0u64)?,
        swapper.tx.clock()?,
    ];

    let target = if swap_x_to_y {
        "swap_exact_base_for_quote"
    } else {
        "swap_exact_quote_for_base"
    };

    let res = swapper
        .tx
        .move_call(&cfg.sponsor, "sponsored", &target, type_tags, args)?
        .split(2)?;

    let (base, quote) = destruct!(2, res);

    let coin_in = if swap_x_to_y { base } else { quote };
    let coin_out = if swap_x_to_y { quote } else { base };

    // Transfer or destroy zero coin
    swapper.tx.transfer_or_destroy_zero_coin(
        &swapper.swap.swap.asset_in,
        coin_in,
        Some(SuiAddress::from_str(&swapper.current_account)?),
    )?;

    Ok(coin_out)
}
