use std::time::Duration;

use anyhow::{Result, bail};
use tokio::time::sleep;

use std::sync::Arc;

use cdk::Amount;
use cdk::amount::SplitTarget;
use cdk::nuts::nut00::ProofsMethods;
use cdk::nuts::{CurrencyUnit, MintQuoteState};
use cdk::wallet::Wallet;
use rand::random;

fn get_work_dir() -> String {
    home::home_dir()
        .unwrap()
        .join(".spillcash")
        .to_str()
        .unwrap()
        .to_string()
}

async fn get_wallet() -> Result<Wallet> {
    // Generate a random seed for the wallet
    let seed = random::<[u8; 64]>();

    // Mint URL and currency unit
    let mint_url = "https://fake.thesimplekid.dev";
    let unit = CurrencyUnit::Sat;

    // Initialize the memory store
    let localstore = cdk_sqlite::WalletSqliteDatabase::new(get_work_dir()).await?;

    // Create a new wallet
    Ok(Wallet::new(
        mint_url,
        unit,
        Arc::new(localstore),
        &seed,
        None,
    )?)
}

pub async fn mint(amount: Option<u64>) -> Result<()> {
    let wallet = get_wallet().await?;
    // Request a mint quote from the wallet
    let quote = wallet
        .mint_quote(amount.map(Amount::from).unwrap_or(1000.into()), None)
        .await?;

    // Check the quote state in a loop with a timeout
    let timeout = Duration::from_secs(60); // Set a timeout duration
    let start = std::time::Instant::now();

    loop {
        let status = wallet.mint_quote_state(&quote.id).await?;

        if status.state == MintQuoteState::Paid {
            break;
        }

        if start.elapsed() >= timeout {
            eprintln!("Timeout while waiting for mint quote to be paid");
            bail!("Timeout while waiting for mint quote to be paid");
        }

        println!("Quote state: {}", status.state);

        sleep(Duration::from_secs(5)).await;
    }

    // Mint the received amount
    let proofs = wallet.mint(&quote.id, SplitTarget::default(), None).await?;
    let receive_amount = proofs.total_amount()?;
    println!("Minted {}", receive_amount);

    Ok(())
}

pub async fn get_token(amount: u64) -> Result<String> {
    let wallet = get_wallet().await?;

    let prep_send = wallet
        .prepare_send(amount.into(), Default::default())
        .await?;

    let token = wallet.send(prep_send, None).await?;

    Ok(token.to_string())
}
