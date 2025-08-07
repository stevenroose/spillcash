use crate::cashu::{get_token, mint};

mod cashu;
mod tx;
mod musig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    mint(None).await?;
    println!("Hello, world!");

    let token = get_token(10).await?;

    println!("{}", token);

    Ok(())
}
