// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
// import "../src/MyUpgradeableContract.sol";
import "../src/MyProxy.sol";

contract DeployScript is Script {
    function run() external {
        vm.startBroadcast();

        // 1️⃣ 部署逻辑合约
        MyUpgradeableContract logic = new MyUpgradeableContract();

        // 2️⃣ 计算初始化调用数据
        bytes memory data = abi.encodeWithSignature("initialize(uint256)", 42);

        // 3️⃣ 部署代理合约
        MyProxy proxy = new MyProxy(address(logic), msg.sender, data);

        console.log("Proxy deployed at:", address(proxy));

        vm.stopBroadcast();
    }
}
