use alloy::{
    consensus::Transaction,
    dyn_abi::DynSolValue,
    hex,
    primitives::{address, keccak256, Address, TxHash, U256},
    providers::{Provider, ProviderBuilder, WsConnect},
    sol,
    sol_types::SolCall,
};

use std::str::FromStr;

use eyre::Result;
use futures_util::StreamExt;
use std::collections::HashMap;
use IUniswapV2Pair::getReservesReturn;

const UNISWAP_V2_ROUTER: Address = address!("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D");
const UNISWAP_V2_FACTORY: Address = address!("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");

// Uniswap V2
sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface IUniswapV2Router {
        function swapExactTokensForTokens(
            uint amountIn,
            uint amountOutMin,
            address[] calldata path,
            address to,
            uint deadline
        ) external;
    }
    #[sol(rpc)]
    interface IUniswapV2Pair {
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
    }

    #[sol(rpc)]
    interface IERC20 {
        function decimals() external view returns (uint8);
    }
}

struct SandwichCalculator<P: Provider> {
    provider: P,
    token_decimals: HashMap<Address, u8>, // store the tokens' decimals
}

impl<P: Provider> SandwichCalculator<P> {
    /// Need to implement
    fn get_token_price(&self, token: Address) -> f64 {
        0f64
    }

    async fn is_profitable(
        &mut self,
        decoded: &IUniswapV2Router::swapExactTokensForTokensCall,
    ) -> bool {
        // 1. 解析交易参数
        let path = &decoded.path;
        if path.len() < 2 {
            return false;
        }

        let token_in = path[0];
        let token_out = path[path.len() - 1];
        let amount_in = decoded.amountIn;

        // 2. 获取池子储备
        let (reserve_in, reserve_out) = self.get_pool_reserves(token_in, token_out).await;

        // 3. 计算价格影响
        let price_impact = self.calculate_price_impact(amount_in, reserve_in, reserve_out);

        let token_price = self.get_token_price(token_in);
        // 4. 计算预期利润
        let profit = self
            .estimate_profit(token_in, token_price, amount_in, price_impact)
            .await;

        // 5. 验证 Gas 成本
        let gas_cost = self.estimate_gas_cost().await;

        profit > gas_cost
    }

    /// 获取代币精度
    async fn get_decimals(&mut self, token: Address) -> u8 {
        if let Some(dec) = self.token_decimals.get(&token) {
            return *dec;
        }

        let erc20 = IERC20::new(token, &self.provider);
        let dec = erc20.decimals().call().await.unwrap()._0;
        self.token_decimals.insert(token, dec);
        println!("{token} decimals is {dec}");
        dec
    }

    /// 获取池子储备量
    async fn get_pool_reserves(&self, token_a: Address, token_b: Address) -> (U256, U256) {
        let pair_address = self.compute_pair_address(token_a, token_b);
        let pair = IUniswapV2Pair::new(pair_address, &self.provider);

        let getReservesReturn {
            reserve0, reserve1, ..
        } = pair.getReserves().call().await.unwrap();

        // 确保储备量顺序正确
        if token_a < token_b {
            (reserve0.to::<U256>(), reserve1.to::<U256>())
        } else {
            (reserve1.to::<U256>(), reserve0.to::<U256>())
        }
    }

    /// 计算价格影响（简化版）
    fn calculate_price_impact(&self, amount_in: U256, reserve_in: U256, reserve_out: U256) -> f64 {
        let k = reserve_in * reserve_out;
        let new_reserve_in = reserve_in + amount_in;
        let new_reserve_out = k / new_reserve_in;

        let price_before =
            f64::try_from(&reserve_out).unwrap() / f64::try_from(&reserve_in).unwrap();
        let price_after =
            f64::try_from(&new_reserve_out).unwrap() / f64::try_from(&new_reserve_in).unwrap();
        (price_before - price_after) / price_before
    }

    /// 估算利润（单位：美元）
    async fn estimate_profit(
        &mut self,
        token_in: Address,
        token_price: f64,
        amount_in: U256,
        price_impact: f64,
    ) -> f64 {
        // 简化模型：利润 = 输入金额 * 价格影响 * 0.5
        // 实际需要接入价格预言机
        let dec_in = self.get_decimals(token_in).await;
        let amount_usd =
            f64::try_from(&amount_in).unwrap() / 10f64.powi(dec_in as i32) * token_price;

        amount_usd * price_impact * 0.5
    }

    /// 估算 Gas 成本（单位：美元）
    async fn estimate_gas_cost(&self) -> f64 {
        let gas_price = self.provider.get_gas_price().await.unwrap();
        let eth_price = 3000.0; // 需要接入预言机获取实时价格

        // 三明治交易总 Gas（前置 + 目标 + 后置）
        let total_gas = 400_000u64;
        gas_price as f64 * total_gas as f64 * 1e-9 * eth_price
    }

    /// 计算 V2 配对合约地址
    fn compute_pair_address(&self, token_a: Address, token_b: Address) -> Address {
        let init_code_hash =
            hex!("96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f");

        // 排序代币地址
        let (token0, token1) = if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        };

        let token_value = DynSolValue::Tuple(vec![
            DynSolValue::Address(token0),
            DynSolValue::Address(token1),
        ]);

        // 计算 salt
        let salt = keccak256(token_value.abi_encode());

        // 构建 create2 输入
        let mut input = Vec::new();
        input.extend_from_slice(&[0xff]);
        input.extend_from_slice(UNISWAP_V2_FACTORY.as_ref());
        input.extend_from_slice(salt.as_ref());
        input.extend_from_slice(&init_code_hash);

        // 计算 keccak256 哈希并生成地址
        let hash = keccak256(&input);
        Address::from_slice(&hash[12..32])
    }

    async fn analyze_transaction(&mut self, tx: alloy::rpc::types::Transaction) {
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

                    self.get_decimals(decoded.to).await;

                    // if self.is_profitable(&decoded).await {
                    //     println!("x");
                    // }
                    return;
                }
            }
            _ => {}
        }
    }
}

use utils::get_api_url;

#[path = "../src/utils/mod.rs"]
mod utils;
// 使用示例
#[tokio::main]
async fn main() -> Result<()> {
    // Create the provider.

    let ws = WsConnect::new(get_api_url());
    let provider = ProviderBuilder::new().on_ws(ws).await?;
    let provider_clone = provider.clone();

    // let ws = WsConnect::new("ws://localhost:8545");
    // let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Subscribe to pending transactions.
    // Alteratively use `subscribe_full_pending_transactions` to get the full transaction details
    // directly if supported by the RPC provider.
    let sub = provider.subscribe_pending_transactions().await?;

    // Wait and take the next 3 transactions.
    let mut stream = sub.into_stream().take(20);

    println!("Awaiting pending transactions...");

    let mut calculator = SandwichCalculator {
        provider,
        token_decimals: HashMap::new(),
    };


    // Take the stream and print the pending transaction.
    let handle = tokio::spawn(async move {
        // while let Some(tx_hash) = stream.next().await {
        //     // Get the transaction details.
        //     if let Ok(tx) = provider_clone.get_transaction_by_hash(tx_hash).await {
        //         // println!("Transaction details: {tx:#?}");
        //         calculator.analyze_transaction(tx.unwrap()).await;
        //     }
        // }

        let tx_hash = TxHash::from_str("0x7e7833a807c4df20da51c8c99ddebe7d6f4b85b6b2927f554f407b101ea1b997").unwrap();

        if let Ok(tx) = provider_clone.get_transaction_by_hash(tx_hash).await {
            calculator.analyze_transaction(tx.unwrap()).await;
        }
    });

    handle.await?;

    Ok(())
}
