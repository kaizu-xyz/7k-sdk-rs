use anyhow::Result;
use sui_sdk::types::transaction::Argument;

use crate::{
    library::swap_with_route::{Swapper, ToTypeTags},
    utils::{sui::ObjectRefFetcher, token::normalize_token_type},
};

pub async fn swap<'a>(swapper: &mut Swapper<'a>) -> Result<Argument> {
    let coin_types = swapper.get_type_params()?;
    let lp_coin_type = coin_types.get(0).unwrap();

    let pool_id = swapper.swap.swap.pool_id.clone();
    let return_amount = swapper.swap.swap.return_amount.parse::<u64>()?;
    let coin_in_type = normalize_token_type(&swapper.swap.swap.asset_in);
    let coin_out_type = normalize_token_type(&swapper.swap.swap.asset_out);
    let input_coin_object = swapper.input_coin_object;

    let type_tags = vec![lp_coin_type, coin_in_type, coin_out_type].to_type_tags()?;
    let cfg = &swapper.config.aftermath;
    let c = &swapper.client;

    let pool_obj = c.shared_obj_mut(&pool_id).await?;
    let pool_registry = c.shared_obj_mut(&cfg.pool_registry).await?;
    let protocol_fee_vault = c.shared_obj_mut(&cfg.protocol_fee_vault).await?;
    let treasury = c.shared_obj_mut(&cfg.treasury).await?;
    let insurance_fund = c.shared_obj_mut(&cfg.insurance_fund).await?;
    let referral_vault = c.shared_obj_mut(&cfg.referral_vault).await?;

    let args = vec![
        swapper.tx.obj(pool_obj)?,
        swapper.tx.obj(pool_registry)?,
        swapper.tx.obj(protocol_fee_vault)?,
        swapper.tx.obj(treasury)?,
        swapper.tx.obj(insurance_fund)?,
        swapper.tx.obj(referral_vault)?,
        *input_coin_object,
        swapper.tx.pure(return_amount)?,
        swapper.tx.pure("1000000000000000000")?,
    ];

    let token_out = swapper.tx.move_call(
        &swapper.config.aftermath.base.package,
        "swap",
        "swap_exact_in",
        type_tags,
        args,
    )?;

    Ok(token_out)
}
