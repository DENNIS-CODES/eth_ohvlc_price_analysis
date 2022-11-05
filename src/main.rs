use dotenvy::dotenv;
use ftx::{
    options::Options,
    rest::{GetMarket, Rest, Result},
};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api = Rest::new(Options::from_env());

    let price = api.request(GetMarket::new("ETH-PERP")).await?.price;
    println!("ETH live price is {} USD.", price.unwrap());

    Ok(())
}