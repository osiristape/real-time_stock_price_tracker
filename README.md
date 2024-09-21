# Real-time Stock Price Tracker in Rust

This project is a simple real-time stock price tracker built in Rust using the Alpha Vantage API. It fetches real-time stock prices, stores the last `n` updates, and calculates the average of these prices in real-time.

## Features

- Fetches real-time stock prices using the Alpha Vantage API.
- Stores and tracks the last `n` prices.
- Continuously updates and displays the average stock price for the last `n` updates.

## How it works

- The user provides a stock symbol (e.g., AAPL for Apple) and the Alpha Vantage API key.
- The program fetches the current stock price every few seconds.
- It maintains a list of the last `n` prices, and after each update, it calculates the average of the last `n` prices.

## Prerequisites

To run this project, you will need:

- Rust installed: [Rust Installation Guide](https://www.rust-lang.org/tools/install)
- An [Alpha Vantage API Key](https://www.alphavantage.co/support/#api-key)

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/osiristape/real_time_stock_price_tracker.git
   ```
   
2. Navigate to the project directory:
   ```bash
   cd real_time_stock_price_tracker
   ```

3. Add your Alpha Vantage API key to the main.rs file by replacing YOUR_ALPHA_VANTAGE_API_KEY with your actual API key.

4. Build and run the program:
   ```bash
   cargo run
   ```

## Usage
The program continuously fetches stock price updates for the specified stock symbol (e.g., AAPL for Apple) and prints the average price for the last n updates.

## Example Output
   ```bash
   Real-time Average Stock Price for the last 5 updates: 150.32
   Real-time Average Stock Price for the last 5 updates: 150.45
   Real-time Average Stock Price for the last 5 updates: 150.60
   ```

## Customization
- **Stock Symbol:** You can change the stock symbol by modifying the symbol variable in main.rs.
- **Update Interval:** The program fetches stock prices every 5 seconds. You can adjust this interval by changing the delay in tokio::time::sleep(tokio::time::Duration::from_secs(5)).
- **Number of Updates (n):** Modify the variable n to track a different number of updates (default is 5).

## API Limits
The free version of Alpha Vantage has a rate limit of 5 API requests per minute. You can upgrade to a paid plan for more requests if needed.
