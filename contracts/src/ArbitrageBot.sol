// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@uniswap/v2-periphery/contracts/interfaces/IUniswapV2Router02.sol";

contract ArbitrageBot {
    address public owner;
    IUniswapV2Router02 public dex1;
    IUniswapV2Router02 public dex2;

    modifier onlyOwner() {
        require(msg.sender == owner, "Not the owner");
        _;
    }

    constructor(address _dex1, address _dex2) {
        owner = msg.sender;
        dex1 = IUniswapV2Router02(_dex1);
        dex2 = IUniswapV2Router02(_dex2);
    }

    function executeArbitrage(
        address tokenA,
        address tokenB,
        uint256 amountIn
    ) external onlyOwner {
        IERC20(tokenA).transferFrom(msg.sender, address(this), amountIn);
        IERC20(tokenA).approve(address(dex1), amountIn);

        address[] memory path = new address[](2);
        path[0] = tokenA;
        path[1] = tokenB;

        uint256[] memory amounts = dex1.swapExactTokensForTokens(
            amountIn,
            0,
            path,
            address(this),
            block.timestamp
        );

        uint256 amountReceived = amounts[1];
        IERC20(tokenB).approve(address(dex2), amountReceived);

        address[] memory reversePath = new address[](2);
        reversePath[0] = tokenB;
        reversePath[1] = tokenA;

        uint256[] memory finalAmounts = dex2.swapExactTokensForTokens(
            amountReceived,
            0,
            reversePath,
            address(this),
            block.timestamp
        );

        require(finalAmounts[1] > amountIn, "Arbitrage failed");
        IERC20(tokenA).transfer(owner, finalAmounts[1]);
    }
}
