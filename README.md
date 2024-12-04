# Live Crypto Price Tracker

This is a Rust-based CLI application that allows users to fetch real-time cryptocurrency data, including price, market cap, all-time high (ATH), and last updated time, using the CoinGecko API.

## Features

- Fetch live crypto prices for various coins.
- Display detailed information for a specific cryptocurrency:
  - Coin name
  - Symbol
  - Current price in USD
  - Market cap
  - All-time high (ATH)
  - Last updated timestamp
- Simple command-line interface (CLI) for easy interaction.

 ## Requirements

- **Rust**: Install Rust from [here](https://www.rust-lang.org/tools/install).

## Dependencies


- **Tokio**: Asynchronous runtime for Rust.
- **Reqwest**: HTTP client for making requests to the CoinGecko API.
- **Serde**: A framework for serializing and deserializing Rust data structures.

You can add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/crypto-price-tracker.git
   cd crypto-price-tracker
2. Update the API key:

   ```bash
   let api_key = 'Your API key';
3. Build and run the application:

   ```bash
   cargo build
   cargo run
### Response:

```
Live Crypto price tracker:
Press:
1:Get a coin price
2:Exit
1
Enter the coin name:
solana
-------------------------------------------------------------------------------------------------------------------------------------
| name: Bitcoin | symbol: BTC | Price: 45678.90 | Market Cap: 850000000000 | All time high: 69000.00 | Last updated: 2024-12-02T12:34:56Z |
-------------------------------------------------------------------------------------------------------------------------------------
Live Crypto price tracker:
Press:
1:Get a coin price
2:Exit
2
Exiting...

```
This will display  
  - Coin name
  - Symbol
  - Current price in USD
  - Market cap
  - All-time high (ATH)
  - Last updated timestamp.
