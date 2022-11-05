use dotenvy::dotenv;
use ftx::{
    options::Options,
    rest::{GetMarket, GetAccount, GetPositions, Rest, Result},
};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api = Rest::new(Options::from_env());
    // Get account info and positions
    println!("Account:");
    println!("{:#?}", api.request(GetAccount {}).await.unwrap());
    println!("Positions:");
    println!("{:#?}", api.request(GetPositions {}).await.unwrap());

    let price = api.request(GetMarket::new("ETH-PERP")).await?.price;
    println!("ETH live price is {} USD.", price.unwrap());

    Ok(())
}