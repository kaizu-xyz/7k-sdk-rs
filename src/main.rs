use anyhow::Result;
use features::swap::get_quote::{GetQuoteParams, get_quote};

pub mod client;
pub mod consts;
pub mod features;
pub mod library;
pub mod types;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // let client = SuiClientBuilder::default()
    //     .build("https://fullnode.mainnet.sui.io:443")
    //     .await?;

    let quote = get_quote(GetQuoteParams {
        token_in: "0x2::sui::SUI".to_string(),
        token_out: "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC"
            .to_string(),
        amount_in: "1000000000".to_string(),
        ..Default::default()
    })
    .await?;

    println!("Quote: {:?}", quote);

    Ok(())
}
