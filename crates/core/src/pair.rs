use alloy::{
    consensus::{Transaction, TxEnvelope},
    primitives::{Address, address},
    sol,
    sol_types::SolCall,
};

sol! {
// Uniswap V2 Router
interface IUniswapV2Router {
    function swapExactTokensForTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] path,
        address to,
        uint256 deadline
    ) external;
}

// Uniswap V3 Router
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
}

// Uniswap router addresses on Ethereum mainnet
const UNISWAP_V2_ROUTER: Address = address!("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D");
const UNISWAP_V3_ROUTER: Address = address!("0xE592427A0AEce92De3Edee1F18E0157C05861564");

pub fn print_tx(tx: &TxEnvelope) {
    println!("Tx: https://etherscan.io/tx/{}", tx.tx_hash(),);
}

pub async fn monitor_pairs(tx: &alloy::rpc::types::Transaction, dexs: &Vec<String>) {
    let inner = &tx.inner;

    let input = inner.input();
    let to_str = inner.to().unwrap();
    let has_uni = dexs.iter().any(|x| x == "uniswap");

    if has_uni {
        match to_str {
            UNISWAP_V2_ROUTER => {
                if let Ok(decoded) =
                    IUniswapV2Router::swapExactTokensForTokensCall::abi_decode(&input, true)
                {
                    println!(
                        "[Uniswap V2 Swap]\nAmountIn: {:?}\nPath: {:?}",
                        decoded.amountIn, decoded.path
                    );
                    print_tx(&inner);
                    return;
                }
            }
            UNISWAP_V3_ROUTER => {
                if let Ok(decoded) = IUniswapV3Router::exactInputCall::abi_decode(&input, true) {
                    println!(
                        "[Uniswap V3 Swap]\nAmountIn: {:?}\nPath: {:?}",
                        decoded.params.amountIn, decoded.params.path
                    );
                    print_tx(&inner);
                    return;
                }
            }
            _ => {}
        }
    }
}
