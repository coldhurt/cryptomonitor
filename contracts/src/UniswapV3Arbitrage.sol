// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol";
import "@uniswap/v3-periphery/contracts/interfaces/IQuoter.sol";
import "@uniswap/v2-periphery/contracts/interfaces/IUniswapV2Router02.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import {Script, console} from "forge-std/Script.sol";

contract UniswapV3Arbitrage is Ownable{
    ISwapRouter public uniswapRouter;
    IUniswapV2Router02 public sushiswapRouter;
    IQuoter public uniswapQuoter;

    constructor(
        address _uniswapRouter,
        address _sushiswapRouter,
        address _uniswapQuoter
    ) Ownable(msg.sender) {
        uniswapRouter = ISwapRouter(_uniswapRouter);
        sushiswapRouter = IUniswapV2Router02(_sushiswapRouter);
        uniswapQuoter = IQuoter(_uniswapQuoter);
    }

    /// @dev 获取 Uniswap V3 的报价
    function getUniswapV3Price(
        address tokenIn,
        address tokenOut,
        uint256 amountIn
    ) public returns (uint256 amountOut) {
        amountOut = uniswapQuoter.quoteExactInputSingle(
            tokenIn,
            tokenOut,
            500,
            amountIn,
            0
        );
    }

    /// @dev 获取 Sushiswap 价格（基于 Router）
    function getSushiswapPrice(
        address tokenIn,
        address tokenOut,
        uint256 amountIn
    ) public view returns (uint256 amountOut) {
        address[] memory path = new address[](2);
        path[0] = tokenIn;
        path[1] = tokenOut;
        uint256[] memory amounts = sushiswapRouter.getAmountsOut(
            amountIn,
            path
        );
        return amounts[1];
    }

    /// @dev 检查套利机会
    function checkArbitrageProfit(
        address tokenA,
        address tokenB,
        uint256 amountIn
    ) public returns (bool hasProfit, uint256 profit) {
        uint256 priceA = getUniswapV3Price(tokenA, tokenB, amountIn);
        console.log("price A", priceA);
        uint256 priceB = getSushiswapPrice(tokenB, tokenA, priceA);
        console.log("price B", priceB);

        hasProfit = priceB > amountIn;
        profit = hasProfit ? priceB - amountIn : 0;
    }

    /// @dev 执行套利交易
    function executeArbitrage(
        address tokenA,
        address tokenB,
        uint256 amountIn
    ) external onlyOwner() {
        // 1. 确保有套利利润
        (bool hasProfit, ) = checkArbitrageProfit(tokenA, tokenB, amountIn);
        require(hasProfit, "No arbitrage opportunity");

        // 2. 先从 Uniswap 买入 tokenB
        IERC20(tokenA).approve(address(uniswapRouter), amountIn);
        uint256 amountOut = uniswapRouter.exactInputSingle(
            ISwapRouter.ExactInputSingleParams({
                tokenIn: tokenA,
                tokenOut: tokenB,
                fee: 500,
                recipient: address(this),
                deadline: block.timestamp + 60,
                amountIn: amountIn,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: 0
            })
        );

        // 3. 在 Sushiswap 卖回 tokenA
        IERC20(tokenB).approve(address(sushiswapRouter), amountOut);
        address[] memory path = new address[](2);
        path[0] = tokenB;
        path[1] = tokenA;
        sushiswapRouter.swapExactTokensForTokens(
            amountOut,
            0,
            path,
            msg.sender,
            block.timestamp + 60
        );
    }
}
