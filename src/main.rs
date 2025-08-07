use bitcoin::secp256k1::{Keypair, PublicKey};
use std::str::FromStr;

use crate::cashu::mint;

pub mod cashu;
mod musig;
mod tx;
mod web;

lazy_static::lazy_static! {
    static ref SERVER_PK: PublicKey = PublicKey::from_str("030c99c81e19622d30cc5fe697f65f03eed93bbf177df99065a2d1f3141dcd095e").unwrap();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize cashu operations
    mint(None).await?;
    println!("SpillCash initialized!");

    let user_key =
        Keypair::from_str("b90c5fa5e7c920f2582d67a440a3f2b1c09fff588f42b0a04754f1e7fb63a2d2")
            .unwrap();
    let fund_address = tx::create_address(*SERVER_PK, user_key.public_key());

    // now send to address

    // Create and start the web server
    let app = web::create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
