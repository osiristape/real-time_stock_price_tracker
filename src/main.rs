fn main() {
    // A set of example stock prices
    let stock_prices = vec![50.0, 52.5, 49.8, 53.2, 48.5, 51.0, 50.5, 52.0, 49.5, 50.2];
    
    // Number of updates to average over
    let n = 5;

    // A vector to hold the last n stock prices
    let mut last_n_prices: Vec<f64> = Vec::with_capacity(n);
    let mut total: f64 = 0.0;

    // Iterate through the stock prices and update the average in real-time
    for (i, &price) in stock_prices.iter().enumerate() {
        // If the vector is full, remove the oldest price and update the total
        if last_n_prices.len() == n {
            total -= last_n_prices.remove(0);
        }

        // Add the new price to the list and update the total
        last_n_prices.push(price);
        total += price;

        // Calculate and print the average once we have at least n prices
        if i + 1 >= n {
            let average_price = total / last_n_prices.len() as f64;
            println!(
                "Real-time Average Stock Price for the last {} updates: {:.2}",
                n, average_price
            );
        }
    }
}
