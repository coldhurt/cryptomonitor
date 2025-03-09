use alloy::{
    primitives::TxHash, providers::{Provider, ProviderBuilder, WsConnect}, rpc::types::Transaction
};
use eyre::Result;
use futures_util::StreamExt;

use monitor_core::{monitor_pairs, monitor_swaps, monitor_tokens, swap::is_valid_dexs};
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

    /// Only matched transactions are output
    #[arg(short, long, default_value_t = false)]
    quite: bool,

    /// Network name, ethereum or base
    #[arg(short, long, default_value = "ethereum")]
    network: String,

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
    /// Monitor new pairs
    Pair {
        /// Dex names
        #[arg(short, long, default_values = ["uniswap", "sushiswap", "uniswap_v3"], value_delimiter = ',')]
        dexs: Vec<String>,
    },
    /// Monitor token transfers
    Token {
        /// Token names
        #[arg(short, long, default_values = ["usdt", "usdc"], value_delimiter = ',')]
        tokens: Vec<String>,
    },
    /// Monitor token transfers
    Price {
        /// Token names
        #[arg(short, long, default_values = ["eth"], value_delimiter = ',')]
        tokens: Vec<String>,
    },
}

struct Config {
    dexs: Vec<String>,
    tokens: Vec<String>,
    pair_dexs: Vec<String>,
    quite: bool,
    network: String,
    price_tokens: Vec<String>
}

fn quite_println(quite: bool, text: String) {
    if !quite {
        println!("{}", text);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut config = Config {
        dexs: vec![],
        tokens: vec![],
        pair_dexs: vec![],
        quite: cli.quite,
        network: cli.network,
        price_tokens: vec![],
    };
    println!("Network: {}", config.network);
    match cli.command {
        Commands::Swap { dexs } => {
            config.dexs = dexs.clone();
            if !is_valid_dexs(&config.network, &config.dexs) {
                println!("Invalid dex names");
                return Ok(());
            }
            println!("{:?}", config.dexs);
        }
        Commands::Pair { dexs } => {
            config.pair_dexs = dexs.clone();
            if !is_valid_dexs(&config.network, &config.pair_dexs) {
                println!("Invalid dex names");
                return Ok(());
            }
            println!("{:?}", config.pair_dexs);
        }
        Commands::Token { tokens } => {
            config.tokens = tokens.clone();
            println!("{:?}", config.tokens);
        }
        Commands::Price { tokens } => {
            config.price_tokens = tokens.clone();
            monitor_core::price::get_tokens_price(tokens).await;
            return Ok(());
        }
    }

    let ws = WsConnect::new(get_api_url(Option::Some(&config.network)));
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // let block_number = provider.get_block_number().await?;
    // let block = provider.get_block(block_number).await?;
    println!("Awaiting pending transactions...");

    if config.network == "base" {
        let handle = tokio::spawn(async move {
            let mut processed_tx_count = cli.count;
            loop {
                let block = provider
                    .get_block_by_number(
                        alloy::eips::BlockNumberOrTag::Latest,
                        alloy::rpc::types::BlockTransactionsKind::Hashes,
                    )
                    .await.unwrap().unwrap();
                
                for tx_hash in block.transactions.as_hashes().unwrap() {
                    processed_tx_count -= 1;
                    if processed_tx_count == 0 {
                        return;
                    }
                    if let Ok(tx) = provider.get_transaction_by_hash(TxHash::from(*tx_hash)).await {
                        if tx.is_none() {
                            return;
                        }
                        analyze_tx(&config, &tx.unwrap()).await;
                    }
                }
                return;
            }
        });
        handle.await?;
    } else {
        let sub = provider.subscribe_pending_transactions().await?;
        // Wait and take the next transactions.
        let mut stream = sub.into_stream().take(cli.count);

        // Take the stream and print the pending transaction.
        let handle = tokio::spawn(async move {
            while let Some(tx_hash) = stream.next().await {
                // Get the transaction details.
                if let Ok(tx) = provider.get_transaction_by_hash(tx_hash).await {
                    if tx.is_none() {
                        return;
                    }
                    analyze_tx(&config, &tx.unwrap()).await;
                }
            }
        });
        handle.await?;
    }
    Ok(())
}

async fn analyze_tx(config: &Config, tx: &Transaction) {
    quite_println(config.quite, format!("Tx {}", tx.inner.tx_hash()));

    if config.dexs.len() > 0 {
        // quite_println(config.quite, "Checking if it is a dex swap".to_string());
        monitor_swaps(&config.network, &tx, &config.dexs).await;
    }
    if config.tokens.len() > 0 {
        // quite_println(
        //     config.quite,
        //     "Checking if it is a token transfer".to_string(),
        // );
        monitor_tokens(&config.network, &tx, &config.tokens).await;
    }
    if config.pair_dexs.len() > 0 {
        // quite_println(config.quite, "Checking if it is a new pair".to_string());
        monitor_pairs(&config.network, &tx, &config.pair_dexs).await;
    }
}
