// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Script, console} from "forge-std/Script.sol";
import {UniswapV3Arbitrage} from "../src/UniswapV3Arbitrage.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@uniswap/v3-periphery/contracts/interfaces/IQuoter.sol";
import "@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol";

contract ArbitrageBotScript is Script {
    UniswapV3Arbitrage public arbitrageBot;

    address private constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
    address private constant USDT = 0xdAC17F958D2ee523a2206206994597C13D831ec7;
    address private constant PEPE = 0x6982508145454Ce325dDbE47a25d4ec3d2311933;

    address private constant UNISWAP_V3_ROUTER =
        0xE592427A0AEce92De3Edee1F18E0157C05861564;
    address private constant SUSHISWAP_V2_ROUTER =
        0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F;

    address private constant UNISWAP_V3_QUOTER =
        0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6;

    function run() public {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        address derivedAddress = vm.addr(privateKey);
        console.log("Derived Address:", derivedAddress);
        vm.startBroadcast(privateKey);

        ERC20 weth = ERC20(WETH);
        ERC20 usdt = ERC20(USDT);

        uint256 wethDecimals = weth.decimals();
        uint256 usdtDecimals = usdt.decimals();

        console.log("WETH decimals:", wethDecimals);
        console.log("USDT decimals:", usdtDecimals);

        IQuoter quoter = IQuoter(UNISWAP_V3_QUOTER);

        uint256 ethToUsdt = quoter.quoteExactInputSingle(
            WETH,
            USDT,
            3000,
            1 ether,
            0
        );
        console.log("ethToUsdt:", ethToUsdt / (10 ** usdtDecimals));

        // swapEthToUSDT();

        getUSDTBalance(derivedAddress);

        arbitrageBot = new UniswapV3Arbitrage(
            UNISWAP_V3_ROUTER,
            SUSHISWAP_V2_ROUTER,
            UNISWAP_V3_QUOTER
        );
        address owner = arbitrageBot.owner();

        console.log("Bot owner is", owner);
        require(owner == derivedAddress, "owner is not the creator");

        uint256 amountIn = 1 ether;
        console.log("Amount in", amountIn);
        (bool hasProfit, ) = arbitrageBot.checkArbitrageProfit(WETH, USDT, amountIn);

        console.log("has profit USDT => WETH?", hasProfit);

        vm.stopBroadcast();
    }

    function swapEthToUSDT() internal {
        ISwapRouter swapRouter = ISwapRouter(UNISWAP_V3_ROUTER);
        ISwapRouter.ExactInputSingleParams memory params = ISwapRouter
            .ExactInputSingleParams({
                tokenIn: WETH,
                tokenOut: USDT,
                fee: 500,
                recipient: msg.sender,
                deadline: block.timestamp + 60,
                amountIn: 1 ether,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: 0
            });

        uint256 amountOut = swapRouter.exactInputSingle{value: 1 ether}(params);
        console.log("Swapped 1 ETH to USDT, received:", amountOut);
    }

    function getUSDTBalance(address derivedAddress) public view {
        uint256 balanceOfUsdt = IERC20(USDT).balanceOf(derivedAddress);
        console.log("USDT balance:", balanceOfUsdt / 10 ** 6);
    }
}
