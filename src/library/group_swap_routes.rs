use anyhow::Result;
// use crate::aggregator::{Coin, QuoteResponse, SorPool, SorRoute, SorSwap, TxSorSwap};
// use crate::token::denormalize_token_type;

use crate::{
    types::aggregators::{Coin, QuoteResponse, SorPool, SorRoute, SorSwap, TxSorSwap},
    utils::token::denormalize_token_type,
};

pub fn group_swap_routes(quote_response: &QuoteResponse) -> Result<Vec<Vec<TxSorSwap>>> {
    if quote_response.routes.is_none() || quote_response.swaps.len() == 0 {
        return Ok(vec![]);
    }
    let pool_details = map_pool_ids_to_details(quote_response.routes.as_ref().unwrap());
    let items: Vec<TxSorSwap> = get_tx_sor_swaps(&quote_response.swaps, &pool_details);
    let mut grouped_items: Vec<Vec<TxSorSwap>> = vec![];
    let mut current_group: Vec<TxSorSwap> = vec![];

    for i in 0..items.len() {
        let item = &items[i];
        current_group.push(item.clone());

        let next_item = items.get(i + 1);
        if next_item.is_none() || next_item.as_ref().unwrap().swap.amount.parse::<u64>()? > 0 {
            grouped_items.push(current_group);
            current_group = vec![];
        }
    }

    if !current_group.is_empty() {
        grouped_items.push(current_group);
    }

    Ok(grouped_items)
}

fn map_pool_ids_to_details(routes: &[SorRoute]) -> std::collections::HashMap<String, SorPool> {
    let mut pool_types: std::collections::HashMap<String, SorPool> =
        std::collections::HashMap::new();
    for route in routes {
        for hop in &route.hops {
            pool_types.insert(hop.pool_id.clone(), hop.pool.clone());
        }
    }
    pool_types
}

fn get_tx_sor_swaps(
    swaps: &[SorSwap],
    pool_details: &std::collections::HashMap<String, SorPool>,
) -> Vec<TxSorSwap> {
    swaps
        .iter()
        .map(|swap| {
            let pool = pool_details.get(&swap.pool_id).unwrap();
            let asset_in = denormalize_token_type(&swap.asset_in);
            let asset_out = denormalize_token_type(&swap.asset_out);
            let mut swap_data = swap.clone();

            swap_data.asset_in = asset_in.to_string();
            swap_data.asset_out = asset_out.to_string();

            let coin_x = Coin {
                coin_type: denormalize_token_type(&pool.all_tokens[0].address).to_string(),
                decimals: pool.all_tokens[0].decimal,
            };
            let coin_y = Coin {
                coin_type: denormalize_token_type(&pool.all_tokens[1].address).to_string(),
                decimals: pool.all_tokens[0].decimal,
            };

            let swap_x_to_y = asset_in == coin_x.coin_type;
            TxSorSwap {
                pool: pool.clone(),
                coin_x,
                coin_y,
                swap_x_to_y,
                swap: swap_data,
            }
        })
        .collect()
}
