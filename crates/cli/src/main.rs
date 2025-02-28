use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use eyre::Result;
use futures_util::StreamExt;

use monitor_core::{monitor_pairs, monitor_tokens};
use utils::get_api_url;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cryptomonitor", arg_required_else_help = true)]
#[command(version = "1.0")]
#[command(author = "myt0.com")]
#[command(about = "A cli to monitor crypto info", long_about = None)]
struct Cli {
    /// Number of transactions to monitor
    #[arg(short, long, default_value = "10")]
    count: usize,

    /// Command types
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Monitor new pairs
    Pair {
        /// Dex names
        #[arg(short, long, default_values = ["uniswap", "sushiswap"])]
        dexs: Vec<String>,
    },
    /// Monitor token transfers
    Token {
        /// Token names
        #[arg(short, long, default_values = ["usdt"])]
        tokens: Vec<String>,
    },
}

struct Config {
    dexs: Vec<String>,
    tokens: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut config = Config {
        dexs: vec![],
        tokens: vec![],
    };
    match cli.command {
        Some(Commands::Pair { dexs }) => {
            config.dexs = dexs.clone();
        }
        Some(Commands::Token { tokens }) => {
            config.tokens = tokens.clone();
        }
        None => {},
    }

    let ws = WsConnect::new(get_api_url());
    let provider = ProviderBuilder::new().on_ws(ws).await?;
    let sub = provider.subscribe_pending_transactions().await?;
    // Wait and take the next transactions.
    let mut stream = sub.into_stream().take(cli.count);

    println!("Awaiting pending transactions...");
    // Take the stream and print the pending transaction.
    let handle = tokio::spawn(async move {
        while let Some(tx_hash) = stream.next().await {
            // Get the transaction details.
            if let Ok(tx) = provider.get_transaction_by_hash(tx_hash).await {
                if tx.is_none() {
                    continue;
                }
                let tx = tx.unwrap();
                monitor_pairs(&tx, &config.dexs).await;
                monitor_tokens(&tx, &config.tokens).await;
            }
        }
    });

    handle.await?;

    Ok(())
}
