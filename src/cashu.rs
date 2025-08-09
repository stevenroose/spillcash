use std::time::Duration;

use anyhow::{Result, bail};
use tokio::time::sleep;

use std::sync::Arc;

use cdk::Amount;
use cdk::amount::SplitTarget;
use cdk::nuts::nut00::ProofsMethods;
use cdk::nuts::{CurrencyUnit, MintQuoteState};
use cdk::wallet::MintQuote;
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

async fn get_wallet(mint_url: &str) -> Result<Wallet> {
    // Generate a random seed for the wallet
    let seed = random::<[u8; 64]>();

    // Currency unit
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

pub async fn create_mint_quote(amount: Option<u64>, mint_url: &str) -> Result<MintQuote> {
    let wallet = get_wallet(mint_url).await?;

    // Request a mint quote from the wallet
    let quote = wallet
        .mint_quote(amount.map(Amount::from).unwrap_or(1000.into()), None)
        .await?;

    Ok(quote)
}

pub async fn complete_mint(quote_id: &str, mint_url: &str) -> Result<u64> {
    let wallet = get_wallet(mint_url).await?;

    // Check the quote state with a shorter timeout for immediate feedback
    let timeout = Duration::from_secs(30); // 30 second timeout for immediate check
    let start = std::time::Instant::now();

    loop {
        let status = wallet.mint_quote_state(quote_id).await?;

        if status.state == MintQuoteState::Paid {
            break;
        }

        if start.elapsed() >= timeout {
            eprintln!("Payment not found yet - the invoice may not have been paid");
            bail!("Payment not found yet - the invoice may not have been paid");
        }

        println!("Quote state: {}", status.state);

        sleep(Duration::from_secs(2)).await;
    }

    // Mint the received amount
    let proofs = wallet.mint(quote_id, SplitTarget::default(), None).await?;
    let receive_amount = proofs.total_amount()?;
    println!("Minted {}", receive_amount);

    Ok(receive_amount.into())
}

pub async fn mint(amount: Option<u64>, mint_url: &str) -> Result<()> {
    let quote = create_mint_quote(amount, mint_url).await?;
    println!("Payment request: {}", quote.request);
    println!("Please pay the invoice to continue...");

    let minted_amount = complete_mint(&quote.id, mint_url).await?;
    println!("Successfully minted {} sats", minted_amount);

    Ok(())
}

pub async fn get_token(amount: u64, mint_url: &str) -> Result<String> {
    let wallet = get_wallet(mint_url).await?;

    if wallet.total_balance().await? < amount.into() {
        mint(Some(amount), mint_url).await?;
    }

    let prep_send = wallet
        .prepare_send(amount.into(), Default::default())
        .await?;

    let token = wallet.send(prep_send, None).await?;

    Ok(token.to_string())
}

pub async fn get_balance(mint_url: &str) -> Result<u64> {
    let wallet = get_wallet(mint_url).await?;
    let balance = wallet.total_balance().await?;
    Ok(balance.into())
}
