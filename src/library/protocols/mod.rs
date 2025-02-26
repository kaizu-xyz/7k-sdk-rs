use crate::consts::{MAX_SQRT_PRICE, MIN_SQRT_PRICE};

pub mod aftermath;
pub mod bluefin;
pub mod bluemove;
pub mod cetus;
pub mod deepbook;
pub mod deepbook_v3;
pub mod flowx;
pub mod flowx_v3;
pub mod kriya;
pub mod kriya_v3;
pub mod obric;
pub mod springsui;
pub mod stsui;
pub mod suiswap;
pub mod turbos;

pub fn get_default_sqrt_price_limit(a2b: bool) -> u128 {
    if a2b {
        MIN_SQRT_PRICE
    } else {
        MAX_SQRT_PRICE
    }
}

pub fn get_adjusted_sqrt_price_limit(a2b: bool) -> u128 {
    let mut price_limit = get_default_sqrt_price_limit(a2b);
    if a2b {
        price_limit += 1u128;
    } else {
        price_limit -= 1u128;
    }
    price_limit
}
