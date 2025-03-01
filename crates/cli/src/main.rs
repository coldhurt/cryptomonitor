use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use eyre::Result;
use futures_util::StreamExt;

use monitor_core::{monitor_swaps, monitor_tokens, swap::is_valid_dexs};
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
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Monitor dex swaps
    Swap {
        /// Dex names
        #[arg(short, long, default_values = ["uniswap", "sushiswap", "uniswap_v3"], value_delimiter = ',')]
        dexs: Vec<String>,
    },
    /// Monitor token transfers
    Token {
        /// Token names
        #[arg(short, long, default_values = ["usdt"], value_delimiter = ',')]
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
        Commands::Swap { dexs } => {
            config.dexs = dexs.clone();
            if !is_valid_dexs(&config.dexs) {
                println!("Invalid dex names");
                return Ok(());
            }
            println!("{:?}", config.dexs);
        }
        Commands::Token { tokens } => {
            config.tokens = tokens.clone();
            println!("{:?}", config.tokens);
        }
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
                println!("Tx {}", tx_hash);
                monitor_swaps(&tx, &config.dexs).await;
                monitor_tokens(&tx, &config.tokens).await;
            }
        }
    });

    handle.await?;

    Ok(())
}
