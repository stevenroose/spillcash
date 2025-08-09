use clap::Parser;

mod cashu;
mod cashu_web;
mod musig;
mod tx;
mod web;

#[derive(Parser)]
#[command(name = "spillcash")]
#[command(about = "A Cashu wallet and payment processor")]
struct Cli {
    /// Mint URL to connect to
    #[arg(short, long, default_value = "https://fake.thesimplekid.dev")]
    mint: String,

    /// Initial amount to mint (in sats)
    #[arg(short, long, default_value = "100000")]
    amount: u64,

    /// API server port
    #[arg(long, default_value = "3000")]
    api_port: u16,

    /// Cashu web interface port
    #[arg(long, default_value = "3001")]
    cashu_port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!("SpillCash starting with mint: {}", cli.mint);

    // Create the main web server (API)
    let api_app = web::create_app(cli.mint.clone());
    let api_listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cli.api_port)).await?;

    // Create the cashu web interface
    let cashu_app = cashu_web::create_cashu_app(cli.mint.clone());
    let cashu_listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cli.cashu_port)).await?;

    println!("API server running on http://localhost:{}", cli.api_port);
    println!(
        "Cashu web interface running on http://localhost:{}",
        cli.cashu_port
    );

    // Run both servers concurrently
    tokio::try_join!(
        axum::serve(api_listener, api_app),
        axum::serve(cashu_listener, cashu_app)
    )?;

    Ok(())
}
