

use reqwest;
use serde::Deserialize;
use tokio;

#[derive(Deserialize, Debug)]
struct Data {
    name: String,
    symbol: String,
    current_price: f64,
    market_cap: i64, // Market cap could be a large number, so using i64
    ath: f64,        // ATH could be a floating-point number, so using f64
    last_updated: String,
}

// Function to fetch data for a specific coin
async fn fetch_data(name: &str, api_key: &str) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids={}&x_cg_demo_api_key={}",
        name, api_key
    );
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let coin_data: Vec<Data> = response.json().await?;

        if let Some(coin) = coin_data.first() {
            let res_print = format!(
                "| name: {} | symbol: {} | Price: {} | Market Cap: {} | All time high: {} | Last updated: {}",
                coin.name,
                coin.symbol,
                coin.current_price,
                coin.market_cap,
                coin.ath,
                coin.last_updated
            );

            println!("-------------------------------------------------------------------------------------------------------------------------------------");
            println!("{}", res_print);
            println!("-------------------------------------------------------------------------------------------------------------------------------------");
        } else {
            println!("No data found for the coin.");
        }
    } else {
        println!("Error: {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let api_key = "CG-bgWEt7JWETG12e1BD5vP2Gn3"; // Your API key
   

    loop {
        let mut coin_name = String::new();
        let mut choice = String::new();
        // choice.clear();
        println!("Live Crypto price tracker:");
        println!("Press:\n1:Get a coin price\n2:Exit");
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice, please enter a number.");
                continue;
            }
        };
        if choice == 2 {
            println!("Exiting....");
            break;
        }
        // coin_name.clear();
        println!("Enter the coin name:");

        std::io::stdin()
            .read_line(&mut coin_name)
            .expect("Failed to read line");

        let coin_name = coin_name.trim().to_lowercase(); // Normalize input

        // Fetch and display the data for the specified coin
        if let Err(e) = fetch_data(&coin_name, api_key).await {
            eprintln!("Error fetching data for {}: {}", coin_name, e);
        }
    }

    // Prompt user for coin name
}

