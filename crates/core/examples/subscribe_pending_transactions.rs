//! Example of subscribing and listening for pending transactions in the public mempool by
//! `WebSocket` subscription.
use alloy::{
    consensus::Transaction,
    primitives::{Address, address},
    providers::{Provider, ProviderBuilder, WsConnect},
    sol,
    sol_types::SolCall,
};
use eyre::Result;
use futures_util::StreamExt;
use utils::get_api_url;

// 定义 Uniswap 合约的 ABI
sol! {
  // Uniswap V2 Router 的 Swap 方法
  interface IUniswapV2Router {
      function swapExactTokensForTokens(
          uint256 amountIn,
          uint256 amountOutMin,
          address[] path,
          address to,
          uint256 deadline
      ) external;
  }

  // Uniswap V3 Router 的 ExactInput 方法
  interface IUniswapV3Router {
      struct ExactInputParams {
          bytes path;
          address recipient;
          uint256 deadline;
          uint256 amountIn;
          uint256 amountOutMinimum;
      }

      function exactInput(ExactInputParams calldata params) external payable;
  }

  interface IERC20 {
    function transfer(address to, uint value) public;
  }
}

// Uniswap 官方合约地址（示例，需验证）
const UNISWAP_V2_ROUTER: Address = address!("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D");
const UNISWAP_V3_ROUTER: Address = address!("0xE592427A0AEce92De3Edee1F18E0157C05861564");
const TETHER: Address = address!("0xdAC17F958D2ee523a2206206994597C13D831ec7");

#[tokio::main]
async fn main() -> Result<()> {
    // Create the provider.

    let ws = WsConnect::new(get_api_url(None));
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // let ws = WsConnect::new("ws://localhost:8545");
    // let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Subscribe to pending transactions.
    // Alteratively use `subscribe_full_pending_transactions` to get the full transaction details
    // directly if supported by the RPC provider.
    let sub = provider.subscribe_pending_transactions().await?;

    // Wait and take the next 3 transactions.
    let mut stream = sub.into_stream().take(20);

    println!("Awaiting pending transactions...");

    // Take the stream and print the pending transaction.
    let handle = tokio::spawn(async move {
        while let Some(tx_hash) = stream.next().await {
            // Get the transaction details.
            if let Ok(tx) = provider.get_transaction_by_hash(tx_hash).await {
                // println!("Transaction details: {tx:#?}");
                analyze_transaction(tx.unwrap()).await;
            }
        }
    });

    

    handle.await?;

    Ok(())
}

async fn analyze_transaction(tx: alloy::rpc::types::Transaction) {
    let inner = tx.inner;
    let to_str = inner.to().unwrap();
    println!(
        "Tx hash: https://etherscan.io/tx/{} to: {}",
        inner.tx_hash(),
        to_str
    );

    let input = inner.input();

    match to_str {
        UNISWAP_V2_ROUTER => {
            if let Ok(decoded) =
                IUniswapV2Router::swapExactTokensForTokensCall::abi_decode(&input, true)
            {
                println!(
                    "[Uniswap V2 Swap]\nAmountIn: {:?}\nPath: {:?}",
                    decoded.amountIn, decoded.path
                );
                return;
            }
        }
        UNISWAP_V3_ROUTER => {
            if let Ok(decoded) = IUniswapV3Router::exactInputCall::abi_decode(&input, true) {
                println!(
                    "[Uniswap V3 Swap]\nAmountIn: {:?}\nPath: {:?}",
                    decoded.params.amountIn, decoded.params.path
                );
                return;
            }
        }
        TETHER => {
            if let Ok(decoded) = IERC20::transferCall::abi_decode(&input, true) {
                println!(
                    "[Tether Transfer]\nValue: {:?}\nFrom: {:?}\nTo: {:?}",
                    decoded.value, tx.from, decoded.to
                );
                return;
            }
        }
        _ => {}
    }
}
