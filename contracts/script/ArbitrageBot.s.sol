// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {ArbitrageBot} from "../src/ArbitrageBot.sol";

contract ArbitrageBotScript is Script {
    ArbitrageBot public arbitrageBot;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        // uniswap, sushiswap
        arbitrageBot = new ArbitrageBot(
            0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D,
            0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F
        );

        // pepe, weth
        // (bool profitable, uint256 profit) = (arbitrageBot.checkProfitability(
        //     0x6982508145454Ce325dDbE47a25d4ec3d2311933,
        //     0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2,
        //     100000
        // ));

        // usdt, weth
        (bool profitable, uint256 profit) = (arbitrageBot.checkProfitability(
            0xdAC17F958D2ee523a2206206994597C13D831ec7,
            0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2,
            100000
        ));

        console.log(profitable, profit);


        vm.stopBroadcast();
    }
}
