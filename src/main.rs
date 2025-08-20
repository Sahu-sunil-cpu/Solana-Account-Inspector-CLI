use core::panic;

use clap::Parser;
use anyhow::{Result};
use fetch::*;
mod fetch;



#[derive(Parser)]
struct Cli {
    cluster: String,
    address: String
}

#[tokio::main]
async fn main() -> Result<()> {
      let args = Cli::parse();
      let url = match args.cluster.as_str() {
        "mainnet" => "https://api.mainnet-beta.solana.com",
        "devnet" => "https://api.devnet.solana.com",
        "testnet" => "https://api.testnet.solana.com",
         cluster if cluster.contains("http://localhost:") => cluster,
         _ => panic!("Ented valid cluster type")
      };


    fetch_data(args.address.as_str(), url).await? ;

   Ok(())
}

