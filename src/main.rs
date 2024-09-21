use reqwest::Error;
use serde::Deserialize;
use std::collections::VecDeque;
use std::env;

// Struct to deserialize stock price data from Alpha Vantage API
#[derive(Deserialize, Debug)]
struct AlphaVantageResponse {
    #[serde(rename = "Global Quote")]
    global_quote: GlobalQuote,
}

#[derive(Deserialize, Debug)]
struct GlobalQuote {
    #[serde(rename = "05. price")]
    price: String,
}

// Function to fetch stock price using the Alpha Vantage API
async fn fetch_stock_price(api_url: &str) -> Result<f64, Error> {
    let response = reqwest::get(api_url).await?.json::<AlphaVantageResponse>().await?;
    // Convert price from string to f64
    let price: f64 = response.global_quote.price.parse().unwrap_or(0.0);
    Ok(price)
}

#[tokio::main]
async fn main() {
    // Alpha Vantage API key (replace with your API key)
    let api_key = "YOUR_ALPHA_VANTAGE_API_KEY";

    // Symbol for the stock (e.g., AAPL for Apple, MSFT for Microsoft)
    let symbol = "AAPL";

    // Construct the Alpha Vantage API URL
    let api_url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        symbol, api_key
    );

    let n = 5; // Number of updates to average

    let mut last_n_prices: VecDeque<f64> = VecDeque::with_capacity(n);
    let mut total: f64 = 0.0;

    loop {
        match fetch_stock_price(&api_url).await {
            Ok(price) => {
                // If we've reached the capacity, remove the oldest price and adjust the total
                if last_n_prices.len() == n {
                    if let Some(oldest_price) = last_n_prices.pop_front() {
                        total -= oldest_price;
                    }
                }

                // Add the new price to the list and update the total
                last_n_prices.push_back(price);
                total += price;

                // Calculate and print the average if we have at least n prices
                if last_n_prices.len() == n {
                    let average_price = total / n as f64;
                    println!(
                        "Real-time Average Stock Price for the last {} updates: {:.2}",
                        n, average_price
                    );
                }
            }
            Err(e) => {
                println!("Error fetching stock price: {}", e);
            }
        }

        // Wait for a while before fetching the next price update (e.g., 5 seconds)
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
