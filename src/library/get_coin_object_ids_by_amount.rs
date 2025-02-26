use anyhow::Result;
use sui_sdk::{
    rpc_types::Coin,
    types::base_types::{ObjectID, SuiAddress},
};

use crate::client::get_sui_client;

fn order_coins(array: &mut [Coin], sort_by: &str) {
    let mut swapped;
    let compare_function = if sort_by == "desc" { u64::lt } else { u64::gt };

    loop {
        swapped = false;
        for i in 0..array.len() - 1 {
            let left_value: u64 = array[i].balance;
            let right_value: u64 = array[i + 1].balance;
            if compare_function(&left_value, &right_value) {
                array.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

pub async fn get_coin_object_ids_by_amount(
    address: SuiAddress,
    amount: u64,
    coin_type: &str,
) -> Result<(Vec<ObjectID>, Vec<Coin>, u64)> {
    let mut coin_balances: Vec<Coin> = Vec::new();
    let mut has_next_page = true;
    let mut next_cursor = None;

    while has_next_page {
        let coins = get_sui_client()
            .await?
            .coin_read_api()
            .get_coins(
                address,
                Some(coin_type.to_string()),
                next_cursor.take(),
                None,
            )
            .await?;

        coin_balances.extend(coins.data);
        has_next_page = coins.has_next_page;
        next_cursor = coins.next_cursor;
    }

    order_coins(&mut coin_balances, "desc");

    let mut balance: u64 = 0;
    let mut object_ids = Vec::new();
    let mut object_coins = Vec::new();

    for coin in &coin_balances {
        balance += coin.balance;
        object_ids.push(coin.coin_object_id.clone());
        object_coins.push(coin.clone());
        if balance >= amount {
            break;
        }
    }

    Ok((object_ids, object_coins, balance))
}
