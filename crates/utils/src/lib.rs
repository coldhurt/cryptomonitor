use dotenvy::dotenv;
use std::env;

pub fn get_api_url() -> String{
    dotenv().ok(); // Load .env file
    let api_key = env::var("API_KEY").expect("API_KEY not set");

    // Create the provider.
    let rpc_url = format!("wss://eth-mainnet.g.alchemy.com/v2/{}", api_key);
    rpc_url
}
