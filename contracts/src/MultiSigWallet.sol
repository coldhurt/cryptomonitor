// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract MultiSigWallet {
    struct Transaction {
        address to;
        uint256 value;
        bytes data;
        bool executed;
        uint256 numConfirmations;
        bytes32 txHash;
    }

    address[] public owners;
    mapping(address => bool) public isOwner;
    uint256 public requiredConfirmations;
    uint256 public nonce;
    uint256 public maxPendingTransactions = 100;

    Transaction[] public transactions;
    mapping(uint256 => mapping(address => bool)) public confirmations;

    event Deposit(address indexed sender, uint256 value);
    event SubmitTransaction(uint256 indexed txIndex, address indexed owner, address indexed to, uint256 value, bytes data);
    event ConfirmTransaction(uint256 indexed txIndex, address indexed owner);
    event RevokeConfirmation(uint256 indexed txIndex, address indexed owner);
    event ExecuteTransaction(uint256 indexed txIndex, address indexed owner);

    modifier onlyOwner() {
        require(isOwner[msg.sender], "Not an owner");
        _;
    }

    modifier txExists(uint256 _txIndex) {
        require(_txIndex < transactions.length, "Transaction does not exist");
        _;
    }

    modifier notExecuted(uint256 _txIndex) {
        require(!transactions[_txIndex].executed, "Transaction already executed");
        _;
    }

    modifier notConfirmed(uint256 _txIndex) {
        require(!confirmations[_txIndex][msg.sender], "Transaction already confirmed");
        _;
    }

    constructor(address[] memory _owners, uint256 _requiredConfirmations) {
        require(_owners.length > 0, "Owners required");
        require(
            _requiredConfirmations > 0 && _requiredConfirmations <= _owners.length,
            "Invalid number of required confirmations"
        );
        
        for (uint256 i = 0; i < _owners.length; i++) {
            address owner = _owners[i];
            require(owner != address(0), "Invalid owner");
            require(!isOwner[owner], "Owner not unique");
            isOwner[owner] = true;
            owners.push(owner);
        }
        requiredConfirmations = _requiredConfirmations;
    }

    receive() external payable {
        emit Deposit(msg.sender, msg.value);
    }

    function submitTransaction(address _to, uint256 _value, bytes memory _data) public onlyOwner {
        require(_to != address(this), "Cannot call itself");
        require(transactions.length < maxPendingTransactions, "Too many pending transactions");

        bytes32 txHash = keccak256(abi.encodePacked(_to, _value, _data, nonce));
        transactions.push(Transaction({
            to: _to,
            value: _value,
            data: _data,
            executed: false,
            numConfirmations: 0,
            txHash: txHash
        }));

        emit SubmitTransaction(transactions.length - 1, msg.sender, _to, _value, _data);
        nonce++;
    }

    function confirmTransaction(uint256 _txIndex) public onlyOwner txExists(_txIndex) notExecuted(_txIndex) notConfirmed(_txIndex) {
        Transaction storage transaction = transactions[_txIndex];
        confirmations[_txIndex][msg.sender] = true;
        transaction.numConfirmations++;
        emit ConfirmTransaction(_txIndex, msg.sender);
    }

    function revokeConfirmation(uint256 _txIndex) public onlyOwner txExists(_txIndex) notExecuted(_txIndex) {
        require(confirmations[_txIndex][msg.sender], "Transaction not confirmed");
        require(transactions[_txIndex].numConfirmations < requiredConfirmations, "Cannot revoke after reaching required confirmations");
        
        Transaction storage transaction = transactions[_txIndex];
        confirmations[_txIndex][msg.sender] = false;
        transaction.numConfirmations--;
        emit RevokeConfirmation(_txIndex, msg.sender);
    }

    function executeTransaction(uint256 _txIndex) public onlyOwner txExists(_txIndex) notExecuted(_txIndex) {
        Transaction storage transaction = transactions[_txIndex];
        require(transaction.numConfirmations >= requiredConfirmations, "Not enough confirmations");
        
        transaction.executed = true;
        (bool success, ) = transaction.to.call{value: transaction.value}(transaction.data);
        require(success, "Transaction failed");

        emit ExecuteTransaction(_txIndex, msg.sender);
    }

    function getTransactionCount() public view returns (uint256) {
        return transactions.length;
    }
}