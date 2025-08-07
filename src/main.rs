use bitcoin::secp256k1::{Keypair, PublicKey};
use std::str::FromStr;

use crate::cashu::mint;

pub mod cashu;
mod musig;
mod tx;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize cashu operations
    mint(Some(100_000)).await?;
    println!("SpillCash initialized!");

    // Create and start the web server
    let app = web::create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
