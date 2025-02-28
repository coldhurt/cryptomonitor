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

# Done

New pair monitor: Uniswap(Ethereum)
Token Transfer: USDT(Ethereum)

# TODO

## New Pair Monitor

EVM: Uniswap(Base), SushiSwap(Ethereum,Base), PancakeSwap(BSC)

Solana: Raydium, Orca

## NFT Monitor

create and mint and order in OpenSea, Blur

## Price & Volumn Monitor
## Whale Monitor

Monitor target wallet's activities

## Frontend