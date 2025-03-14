// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IUniswapV2Router {
    function swapExactTokensForTokens(
        uint amountIn,
        uint amountOutMin,
        address[] calldata path,
        address to,
        uint deadline
    ) external returns (uint[] memory amounts);
}

contract SwapTestScript is Script {
    // Uniswap V2 Router Address (以太坊主网，测试用需换成Anvil环境)
    address private constant UNISWAP_V2_ROUTER = 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D;

    // WETH & USDC 地址（以太坊主网，需要改成本地 Anvil 的测试代币地址）
    address private constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
    address private constant USDT = 0xdAC17F958D2ee523a2206206994597C13D831ec7;

    function run() external {
        // 读取私钥
        uint256 privateKey = vm.envUint("PRIVATE_KEY");

        // 从私钥推导出地址
        address derivedAddress = vm.addr(privateKey);

        // 输出地址
        console.log("Derived Address:", derivedAddress);
        vm.startBroadcast(privateKey);

        IUniswapV2Router uniswap = IUniswapV2Router(UNISWAP_V2_ROUTER);
        IERC20(WETH).approve(UNISWAP_V2_ROUTER, type(uint256).max);

        address[] memory path;
        path[0] = WETH;
        path[1] = USDT;

        // uniswap.swapExactTokensForTokens(
        //     1 ether,        // 1 WETH
        //     0,              // 最小接受 USDC 数量
        //     path,
        //     msg.sender,     // 收款地址
        //     block.timestamp + 60 // 交易截止时间
        // );

        uint256 balanceOfUsdt = IERC20(USDT).balanceOf(derivedAddress); 

        console.log("USDT balance ", balanceOfUsdt);

        vm.stopBroadcast();
    }
}
