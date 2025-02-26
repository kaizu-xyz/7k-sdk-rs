use anyhow::Result;
use sui_sdk::{SuiClient, SuiClientBuilder};

// TODO: Right now, the sdk only works on mainnet
pub async fn get_sui_client() -> Result<SuiClient> {
    let sui_localnet = SuiClientBuilder::default()
        .build("http://127.0.0.1:9000")
        .await?;

    // Sui testnet -- https://fullnode.testnet.sui.io:443
    let sui_testnet = SuiClientBuilder::default().build_testnet().await?;
    println!("Sui testnet version: {}", sui_testnet.api_version());

    // Sui devnet -- https://fullnode.devnet.sui.io:443
    let sui_devnet = SuiClientBuilder::default().build_devnet().await?;
    println!("Sui devnet version: {}", sui_devnet.api_version());

    // TODO
    Ok(sui_localnet)
}
