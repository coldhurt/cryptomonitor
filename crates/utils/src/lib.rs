use dotenvy::dotenv;
use std::{collections::HashMap, env};

fn get_api_prefix(network: &str) -> String {
    let all_prefix: HashMap<&'static str, String> = HashMap::from([
        ("mainnet", "eth-mainnet".to_string()),
        ("base", "base-mainnet".to_string()),
    ]);

    all_prefix.get(network).unwrap_or(&"eth-mainnet".to_string()).to_string()
}

pub fn get_api_url(network: Option<&str>) -> String {
    let network = network.unwrap_or("ethereum");
    dotenv().ok(); // Load .env file
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let prefix = get_api_prefix(network);
    // Create the provider.
    let rpc_url = format!("wss://{}.g.alchemy.com/v2/{}", prefix, api_key);
    rpc_url
}
