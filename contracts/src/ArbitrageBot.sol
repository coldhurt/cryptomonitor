// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@uniswap/v2-periphery/contracts/interfaces/IUniswapV2Router02.sol";

contract ArbitrageBot {
    address public owner;
    IUniswapV2Router02 public dex1;
    IUniswapV2Router02 public dex2;
    
    constructor(address _dex1, address _dex2) {
        owner = msg.sender;
        dex1 = IUniswapV2Router02(_dex1);
        dex2 = IUniswapV2Router02(_dex2);
    }
    
    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }
    
    function checkProfitability(
        address tokenA, 
        address tokenB,
        uint256 amountIn
    ) public view returns (bool profitable, uint256 profit) {
        uint256 amountOutDex1 = getAmountOut(dex1, tokenA, tokenB, amountIn);
        uint256 amountOutDex2 = getAmountOut(dex2, tokenB, tokenA, amountOutDex1);

        if (amountOutDex2 > amountIn) {
            return (true, amountOutDex2 - amountIn);
        } else {
            return (false, 0);
        }
    }
    
    function executeArbitrage(
        address tokenA, 
        address tokenB, 
        uint256 amountIn
    ) external onlyOwner {
        (bool profitable, ) = checkProfitability(tokenA, tokenB, amountIn);
        require(profitable, "No profit detected");
        
        IERC20(tokenA).approve(address(dex1), amountIn);
        uint256 amountOutDex1 = swap(dex1, tokenA, tokenB, amountIn);
        IERC20(tokenB).approve(address(dex2), amountOutDex1);
        uint256 finalAmount = swap(dex2, tokenB, tokenA, amountOutDex1);
        
        require(finalAmount > amountIn, "Arbitrage failed");
    }
    
    function getAmountOut(
        IUniswapV2Router02 dex, 
        address tokenIn,
        address tokenOut, 
        uint256 amountIn
    ) public view returns (uint256) {
        address[] memory path = new address[](2);
        path[0] = tokenIn;
        path[1] = tokenOut;
        uint256[] memory amounts = dex.getAmountsOut(amountIn, path);
        return amounts[1];
    }
    
    function swap(
        IUniswapV2Router02 dex,
        address tokenIn,
        address tokenOut,
        uint256 amountIn
    ) internal returns (uint256) {
        address[] memory path = new address[](2);
        path[0] = tokenIn;
        path[1] = tokenOut;
        uint256[] memory amounts = dex.swapExactTokensForTokens(
            amountIn, 0, path, address(this), block.timestamp + 300
        );
        return amounts[1];
    }
}