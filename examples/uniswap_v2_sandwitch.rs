use alloy::{
    network::Ethereum,
    primitives::{Address, Bytes, address, U256, keccak256},
    providers::{Provider, ProviderBuilder},
    sol,
    hex,
};
use std::collections::HashMap;

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

    interface IUniswapV2Pair {
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
    }

    interface IERC20 {
        function decimals() external view returns (uint8);
    }
}

// 利润计算器
struct SandwichCalculator {
    provider: Provider<Ethereum>,
    gas_price: f64,                       // 单位：Gwei
    min_profit: f64,                      // 最小盈利阈值（美元）
    token_decimals: HashMap<Address, u8>, // 代币精度缓存
}

impl SandwichCalculator {
    /// 核心验证逻辑
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

        // 4. 计算预期利润
        let profit = self.estimate_profit(amount_in, price_impact);

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
        let dec = erc20.decimals().call().await.unwrap_or(18);
        self.token_decimals.insert(token, dec);
        dec
    }

    /// 获取池子储备量
    async fn get_pool_reserves(&self, token_a: Address, token_b: Address) -> (U256, U256) {
        let pair_address = self.compute_pair_address(token_a, token_b);
        let pair = IUniswapV2Pair::new(pair_address, &self.provider);

        match pair.getReserves().call().await {
            Ok((reserve0, reserve1, _)) => {
                // 确保储备量顺序正确
                if token_a < token_b {
                    (reserve0.into(), reserve1.into())
                } else {
                    (reserve1.into(), reserve0.into())
                }
            }
            Err(_) => (U256::ZERO, U256::ZERO),
        }
    }

    /// 计算价格影响（简化版）
    fn calculate_price_impact(&self, amount_in: U256, reserve_in: U256, reserve_out: U256) -> f64 {
        let k = reserve_in * reserve_out;
        let new_reserve_in = reserve_in + amount_in;
        let new_reserve_out = k / new_reserve_in;

        let price_before = reserve_out.to::<f64>() / reserve_in.to::<f64>();
        let price_after = new_reserve_out.to::<f64>() / new_reserve_in.to::<f64>();

        (price_before - price_after) / price_before
    }

    /// 估算利润（单位：美元）
    fn estimate_profit(&mut self, amount_in: U256, price_impact: f64) -> f64 {
        // 简化模型：利润 = 输入金额 * 价格影响 * 0.5
        // 实际需要接入价格预言机
        let dec_in = self.get_decimals(token_in).await;
        let amount_usd = amount_in.to::<f64>() / 10f64.powi(dec_in as i32) * token_price;

        amount_usd * price_impact * 0.5
    }

    /// 估算 Gas 成本（单位：美元）
    async fn estimate_gas_cost(&self) -> f64 {
        let gas_price = self.provider.get_gas_price().await.unwrap();
        let eth_price = 3000.0; // 需要接入预言机获取实时价格

        // 三明治交易总 Gas（前置 + 目标 + 后置）
        let total_gas = 400_000u64;
        gas_price.to::<f64>() * total_gas as f64 * 1e-9 * eth_price
    }

    /// 计算 V2 配对合约地址
    fn compute_pair_address(&self, token_a: Address, token_b: Address) -> Address {
        // 使用 Uniswap V2 官方工厂地址
        let factory = address!("5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");
        let init_code_hash =
            hex!("96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f");

        // 排序代币地址
        let (token0, token1) = if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        };

        // 使用 create2 计算地址
        let salt = keccak256(abi::encode(&[token0, token1]));
        let mut hasher = keccak256::new();
        hasher.update(hex!("ff"));
        hasher.update(factory);
        hasher.update(salt);
        hasher.update(init_code_hash);
        Address::from_slice(&hasher.finalize()[12..])
    }
}

// 使用示例
#[tokio::main]
async fn main() {
    let provider = ProviderBuilder::new()
        .on_ws("wss://mainnet.infura.io/ws/v3/YOUR_KEY".parse().unwrap())
        .await
        .unwrap();

    let mut calculator = SandwichCalculator {
        provider,
        gas_price: 20.0,  // 20 Gwei
        min_profit: 50.0, // 最少50美元利润
        token_decimals: HashMap::new(),
    };

    // 检测到交易时
    if let Ok(decoded) = IUniswapV2Router::swapExactTokensForTokensCall::abi_decode(&input, true) {
        if calculator.is_profitable(&decoded).await {
            println!("可执行三明治攻击！");
        }
    }
}
