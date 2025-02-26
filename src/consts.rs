// 7k consts
pub const _7K_PACKAGE_ID: &'static str =
    "0x7ea6e27ad7af6f3b8671d59df1aaebd7c03dddab893e52a714227b2f4fe91519";
//legacy V2: "0xa13447019cd982d6bef91cf7b46ad384a52583b1dfc2bdecf31ef0c4bd787a0f";
//legacy V1: "0xd48e7cdc9e92bec69ce3baa75578010458a0c5b2733d661a84971e8cef6806bc";
pub const _7K_CONFIG: &'static str =
    "0x0f8fc23dbcc9362b72c7a4c5aa53fcefa02ebfbb83a812c8c262ccd2c076d9ee";
pub const _7K_VAULT: &'static str =
    "0x39a3c55742c0e011b6f65548e73cf589e1ae5e82dbfab449ca57f24c3bcd9514";

// Explorer
pub const EXPLORER_URI: &'static str = "https://suiscan.xyz";

// Coin types
pub const SUI_TYPE: &'static str = "0x2::sui::SUI";
pub const SUI_FULL_TYPE: &'static str =
    "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI";
pub const USDC_TOKEN_TYPE: &'static str =
    "0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf::coin::COIN";
pub const NATIVE_USDC_TOKEN_TYPE: &'static str =
    "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC";

// Token types
pub const TOKEN_TYPES: [&str; 2] = [SUI_TYPE, SUI_FULL_TYPE];

pub const MIN_SQRT_PRICE: u128 = 4295048016;
pub const MAX_SQRT_PRICE: u128 = 79226673515401279992447579055;
