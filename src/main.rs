use crate::cashu::{get_token, mint};

mod cashu;
mod musig;
mod tx;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize cashu operations
    mint(None).await?;
    println!("SpillCash initialized!");

    let token = get_token(10).await?;
    println!("Generated token: {}", token);

    // Create and start the web server
    let app = web::create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
