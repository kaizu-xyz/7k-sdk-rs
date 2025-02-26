use std::str::FromStr;

use anyhow::Result;
use move_core_types::language_storage::StructTag;
use sui_sdk::{
    types::{transaction::Argument, TypeTag},
    SuiClient,
};

use crate::{
    types::aggregators::{Config, SourceDex, TxSorSwap},
    utils::sui::Ptb,
};

use super::protocols::{
    aftermath, bluefin, bluemove, cetus, deepbook, deepbook_v3, flowx, flowx_v3, kriya, kriya_v3,
    obric, springsui, stsui, suiswap, turbos,
};

pub struct Swapper<'a> {
    pub client: &'a SuiClient,
    pub swap: &'a TxSorSwap,
    pub input_coin_object: &'a Argument,
    pub current_account: &'a str,
    pub config: &'a Config,
    pub tx: &'a mut Ptb,
}

pub trait ToTypeTags {
    fn to_type_tags(&self) -> Result<Vec<TypeTag>>;
}

impl ToTypeTags for Vec<&str> {
    fn to_type_tags(&self) -> Result<Vec<TypeTag>> {
        self.iter().map(|s| TypeTag::from_str(s)).collect()
    }
}

impl<'a> Swapper<'a> {
    pub fn get_type_params(&mut self) -> Result<Vec<String>> {
        if let Some(extra) = self.swap.swap.extra.as_ref() {
            let pool_struct_tag = extra.get("pool_struct_tag").unwrap(); // todo: remove unwrap and do not err, instead default to "" type

            let tag: StructTag = StructTag::from_str(pool_struct_tag)?;

            let cannonical_types: Vec<String> = tag
                .type_params
                .iter()
                .map(|ttag| ttag.to_canonical_string(true)) // todo: assuming true: has prefix?
                .collect();

            Ok(cannonical_types)
        } else {
            return Err(anyhow::anyhow!("No pool struct tag.."));
        }
    }

    pub fn get_input_coin_value(&mut self) -> Result<Argument> {
        self.tx.coin_value(
            self.swap.swap.asset_in.clone(),
            self.input_coin_object.to_owned(),
        )
    }

    pub async fn swap(&mut self) -> Result<Argument> {
        let pool_type = self.swap.pool.pool_type;

        let token_out = match pool_type {
            SourceDex::Aftermath => aftermath::swap(self).await?,
            SourceDex::Bluefin => bluefin::swap(self).await?,
            SourceDex::Bluemove => bluemove::swap(self).await?,
            SourceDex::Cetus => cetus::swap(self).await?,
            SourceDex::Deepbook => deepbook::swap(self).await?,
            SourceDex::DeepbookV3 => deepbook_v3::swap(self).await?,
            SourceDex::Flowx => flowx::swap(self).await?,
            SourceDex::FlowxV3 => flowx_v3::swap(self).await?,
            SourceDex::Kriya => kriya::swap(self).await?,
            SourceDex::KriyaV3 => kriya_v3::swap(self).await?,
            SourceDex::Obric => obric::swap(self).await?,
            SourceDex::Springsui => springsui::swap(self).await?,
            SourceDex::Stsui => stsui::swap(self).await?,
            SourceDex::Suiswap => suiswap::swap(self).await?,
            SourceDex::Turbos => turbos::swap(self).await?,
        };

        Ok(token_out)
    }
}

pub async fn swap_with_route(
    client: &SuiClient,
    route: &Vec<TxSorSwap>,
    input_coin_object: Argument,
    current_account: &String,
    config: &Config,
    tx: &mut Ptb,
) -> Result<Argument> {
    let mut next_coin = input_coin_object;

    for swap in route.iter() {
        let mut swapper = Swapper {
            client,
            swap,
            input_coin_object: &next_coin,
            current_account: current_account.as_str(),
            config,
            tx,
        };

        let token_out = swapper.swap().await?;

        next_coin = token_out.clone();
    }
    Ok(next_coin)
}
