use dotenvy::dotenv;
use std::{collections::HashMap, env};

fn get_api_prefix(network: &str) -> String {
    let all_prefix: HashMap<&'static str, String> = HashMap::from([
        ("mainnet", "eth-mainnet".to_string()),
        ("base", "base-mainnet".to_string()),
        ("bnb", "bnb-mainnet".to_string()),
    ]);

    all_prefix
        .get(network)
        .unwrap_or(&"eth-mainnet".to_string())
        .to_string()
}

fn get_bnb_api_url() -> String {
    let bnb_url = env::var("BNB_WS_API_URL").unwrap();

    bnb_url
}

pub fn get_api_url(network: Option<&str>) -> String {
    let network = network.unwrap_or("ethereum");
    dotenv().ok(); // Load .env file
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let prefix = get_api_prefix(network);
    // Create the provider.
    let mut rpc_url = format!("wss://{}.g.alchemy.com/v2/{}", prefix, api_key);
    if network == "bnb" {
        rpc_url = get_bnb_api_url();
    }
    rpc_url
}
