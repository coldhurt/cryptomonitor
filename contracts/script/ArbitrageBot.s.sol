// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {ArbitrageBot} from "../src/ArbitrageBot.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@uniswap/v2-periphery/contracts/interfaces/IUniswapV2Router02.sol";

contract ArbitrageBotScript is Script {
    ArbitrageBot public arbitrageBot;

    address private constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
    address private constant USDT = 0xdAC17F958D2ee523a2206206994597C13D831ec7;
    address private constant PEPE = 0x6982508145454Ce325dDbE47a25d4ec3d2311933;

    address private constant UNISWAP_V2_ROUTER = 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D;
    address private constant SUSHISWAP_V2_ROUTER = 0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F;

    function run() public {

        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        address derivedAddress = vm.addr(privateKey);
        console.log("Derived Address:", derivedAddress);
        vm.startBroadcast(privateKey);

        ERC20 weth = ERC20(WETH);
        ERC20 usdt = ERC20(USDT);

        uint256 wethDecimals = weth.decimals();
        uint256 usdtDecimals = usdt.decimals();

        console.log("WETH decimals", wethDecimals);
        console.log("USDT decimals", usdtDecimals);

        // uniswap, sushiswap
        arbitrageBot = new ArbitrageBot(
            SUSHISWAP_V2_ROUTER,
            UNISWAP_V2_ROUTER
        );

        // uint256 amountIn = 0.01 ether;
        // console.log("Amount in:", amountIn);

        // uint256 amount1 = arbitrageBot.getAmountOut(IUniswapV2Router02(uniswapRouterV2), WETH, USDT, amountIn);
        // console.log("Amount middle:", amount1);

        // uint256 amount2 = arbitrageBot.getAmountOut(IUniswapV2Router02(sushiswapRouterV2), USDT, WETH, amount1);
        // console.log("Amount out:", amount2);

        (bool profitable, uint256 profit) = (arbitrageBot.checkProfitability(
            WETH,
            USDT,
            1 ether
        ));

        console.log(profitable, profit);


        // IUniswapV2Router02 uniswap = IUniswapV2Router02(UNISWAP_V2_ROUTER);
        // address[] memory path = new address[](2);
        // path[0] = WETH;
        // path[1] = USDT;
        // uniswap.swapExactETHForTokens{value: 1 ether}(
        //     0,              // 最小接受 USDT 数量
        //     path,
        //     msg.sender,     // 收款地址
        //     block.timestamp + 60 // 交易截止时间
        // );

        // arbitrageBot.executeArbitrage(USDT, WETH, 10 * 10 ** usdtDecimals);
        vm.stopBroadcast();
    }
}
