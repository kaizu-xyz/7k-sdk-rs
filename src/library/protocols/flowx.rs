use anyhow::Result;
use sui_sdk::types::transaction::Argument;

use crate::{
    library::swap_with_route::{Swapper, ToTypeTags},
    utils::{sui::ObjectRefFetcher, token::normalize_token_type},
};

pub async fn swap<'a>(swapper: &mut Swapper<'a>) -> Result<Argument> {
    let coin_in_type = normalize_token_type(&swapper.swap.swap.asset_in);
    let coin_out_type = normalize_token_type(&swapper.swap.swap.asset_out);

    let config = &swapper.config.flowx;

    let container_obj = swapper.client.shared_obj_mut(&config.container).await?;

    let type_tags = vec![coin_in_type, coin_out_type].to_type_tags()?;

    let args = vec![swapper.tx.obj(container_obj)?, *swapper.input_coin_object];

    let token_out = swapper.tx.move_call(
        &config.base.package,
        "router",
        "swap_exact_input_direct",
        type_tags,
        args,
    )?;

    Ok(token_out)
}
