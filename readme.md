# How to use

1. Make sure you have installed rust https://doc.rust-lang.org/cargo/getting-started/installation.html

2. Clone this project

3. Add alchemy api key to .env

```bash
mv .env.default .env
vim .env
```

4. Build
   
```bash
cargo build -p cli
mv ./target/release/cli .
./cli --help
```

5. Monitor

Monior new pairs in the 100 pending txs
```bash
./cli -c 100 pair
```

Monior token transfers
```bash
./cli -c 100 token
```
![Demo GIF](demo.gif)

# Test

There are some other functions under development
```
cargo run --example deploy_erc20 -p core
```

# Functions

- [x] Monitor new pair: Uniswap(Ethereum)
- [x] Monitor token transfer: USDT, USDC, PEPE(Ethereum)
- [ ] Monitor new pair: SuiShiSwap(Ethereum)
- [ ] Monitor new pair: PancakeSwap(BSC)
- [ ] Monitor new pair for Base network
- [ ] Monitor new pair for Raydium, Orca on Solana
- [ ] Monitor creating nft or order on OpenSea, Blur
- [ ] Monitor target wallet's txs
- [ ] Monitor token price
- [ ] Monitor token volume
- [ ] Frontend UI