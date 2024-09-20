# Real-time Stock Price Tracker

This is a simple Rust program that simulates a real-time stock price tracker. The program receives stock price updates in real-time and calculates the average price of the last `n` updates.

## How it works

- The program keeps track of stock price updates.
- It maintains the last `n` prices in a vector.
- After every price update, it calculates the average of the last `n` prices and displays it in real-time.

## Usage

### Requirements
- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### How to run

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/real_time_stock_tracker.git
   ```

2. Navigate to the project directory:
   ```bash
    cd real_time_stock_tracker
   ```
   
3. Build and run the program:
   ```bash
   cargo run
   ```

You will see the real-time average stock prices for the last n updates displayed in the terminal.

## Example Output
   ```bash
   Real-time Average Stock Price for the last 5 updates: 51.00
   Real-time Average Stock Price for the last 5 updates: 51.00
   Real-time Average Stock Price for the last 5 updates: 50.60
   Real-time Average Stock Price for the last 5 updates: 51.04
   Real-time Average Stock Price for the last 5 updates: 50.34
   Real-time Average Stock Price for the last 5 updates: 50.24
   ```

