use crate::{
    library::swap_with_route::{Swapper, ToTypeTags},
    utils::sui::ObjectRefFetcher,
};
use anyhow::Result;
use sui_sdk::types::transaction::Argument;

pub async fn swap<'a>(swapper: &mut Swapper<'a>) -> Result<Argument> {
    let type_tags = vec![
        swapper.swap.swap.asset_in.as_str(),
        swapper.swap.swap.asset_out.as_str(),
    ]
    .to_type_tags()?;

    let amount_in = swapper.get_input_coin_value()?;

    let c = &swapper.client;

    let coin_in = c.owned_obj(&swapper.swap.swap.asset_in).await?;
    let dex_info = c.owned_obj(&swapper.config.bluemove.dex_info).await?;
    let coin_in_obj = swapper.tx.obj(coin_in)?;

    let args = vec![
        amount_in,
        coin_in_obj,
        swapper.tx.pure(0)?,
        swapper.tx.obj(dex_info)?,
    ];

    let coin_out = swapper.tx.move_call(
        &swapper.config.aftermath.base.package,
        "router",
        "swap_exact_input_",
        type_tags,
        args,
    )?;

    Ok(coin_out)
}
