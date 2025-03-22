use serde::Deserialize; 

#[derive(Debug, Deserialize)]
struct PriceResponse {
    price: String, 
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let url = "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT";

    let response = reqwest::get(url)
        .await?
        .error_for_status()?; 

    let price_data: PriceResponse = response.json().await?;

    println!("BTC/USDT: {}", price_data.price);

    Ok(())
}
