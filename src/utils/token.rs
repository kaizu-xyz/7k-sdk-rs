use crate::{
    consts::{EXPLORER_URI, SUI_FULL_TYPE, SUI_TYPE},
    types::token::SuiscanToken,
};
use serde_json::Value;

// Token normalization functions
pub fn normalize_token_type(typename: &str) -> &str {
    if typename == SUI_TYPE {
        return SUI_FULL_TYPE;
    }
    return typename;
}

pub fn denormalize_token_type(typename: &str) -> &str {
    if typename == SUI_FULL_TYPE {
        return SUI_TYPE;
    }
    return typename;
}

pub fn check_is_sui(typename: &str) -> bool {
    typename == SUI_TYPE || typename == SUI_FULL_TYPE
}

#[allow(dead_code)]
async fn get_suiscan_token_metadata(typename: String) -> Result<SuiscanToken, String> {
    let response = reqwest::get(&format!(
        "{}/api/sui-backend/mainnet/api/coins/{}",
        EXPLORER_URI,
        denormalize_token_type(&typename),
    ))
    .await
    .map_err(|err| format!("Network error: {}", err))?;

    if !response.status().is_success() {
        return Err("Failed to fetch token metadata: HTTP error".into());
    }

    let parsed_data: Value = response
        .json()
        .await
        .map_err(|err| format!("Failed to parse JSON response: {}", err))?;

    let token_type = match parsed_data.get("type").and_then(|val| val.as_str()) {
        Some(type_str) => normalize_token_type(type_str),
        None => SUI_TYPE,
    };

    // Assuming you fill SuiscanToken using the parsed_data
    Ok(SuiscanToken {
        token_type: token_type.to_string(),
        ..serde_json::from_value(parsed_data)
            .map_err(|err| format!("Failed to deserialize token metadata: {}", err))?
    })
}

pub fn format_balance(balance: u64, decimals: u32) -> u64 {
    let divisor = 10_u64.pow(decimals);
    balance / divisor
}

pub fn format_raw_balance(balance: u64, decimals: u32) -> u64 {
    balance.checked_mul(10_u64.pow(decimals)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_balance() {
        assert_eq!(format_balance(1000000, 6), 1_u64);
        assert_eq!(format_balance(1234567000000, 6), 1234567_u64);
    }

    #[test]
    fn test_format_raw_balance() {
        assert_eq!(format_raw_balance(1, 6), 1000000);
        assert_eq!(format_raw_balance(1234, 6), 1234000000);
    }
}
