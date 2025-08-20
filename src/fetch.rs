use anyhow::{Context, Ok, Result};
use solana_client::{nonblocking::rpc_client::RpcClient};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{str::FromStr};


pub async fn fetch_data(pubkey: &str, url: &str) -> Result<()> {
 let client = RpcClient::new_with_commitment(
        url.to_string(),
        CommitmentConfig::confirmed()
    );
    let pubkey = Pubkey::from_str(pubkey)
    .context("Address is not valid")?;
    let account = client.get_account(&pubkey).await
    .context("failed to fetch account details, check your address")?;

    println!("{:#?}", account);

    
    Ok(())
}

#[tokio::test]
async fn test_fetch_data() -> Result<()> {
    let address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
    let url = "https://api.devnet.solana.com";
    fetch_data(address, url ).await?;
    Ok(())
}


/*
test for mint address, token address
*/