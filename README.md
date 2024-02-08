# Solidity-Gas-Optimizoor
An automated CLI tool that optimizes gas usage in Solidity smart contracts, focusing on storage and function call efficiency. 

## How does our tool optimize the gas of your smart contracts?
Automated Rust-based CLI tool that optimizes gas of solidity code by 
- struct packing
	- We can save storage by ordering variables that use less than 32 bytes next to each other.
	- Storage packing is particularly useful when reading or writing multiple values in the same storage slot. In such cases, only a single SLOAD or SSTORE operation is needed, significantly reducing the cost of accessing storage variables by half or more. This situation commonly occurs with structs:
	- parses Solidity structs and packs the fields efficiently to reduce the number of storage slots they use. It also adds struct packing comments to clearly indicate how the fields are packed.
	- It can deal with comments and whitespace in the struct definition, and will preserve them in the output. It handles unknown types by assuming they cannot be packed, treating they as `bytes32`.
- [tight packing](https://fravoll.github.io/solidity-patterns/tight_variable_packing.html)
	- Save gas by using smaller data types (e.g. `bytes16`, `uint32`) when possible, as the EVM can then pack them together in one single 32 byte slot and therefore use less storage. Gas is then saved because the EVM can combine multiple reads or writes into one single operation. The underlying behavior is also referred to as “tight packing” and is unfortunately, until the time of writing, not automatically achieved by the optimizer.
- Fixed size variables are cheaper than dynamic size variables
	- As a general rule, use bytes for arbitrary-length raw byte data and string for arbitrary-length string (UTF-8) data. If you can limit the length to a certain number of bytes, always use one of the value types (bytes1 to bytes32) because they are much cheaper.
	- The same applies for arrays: If you know that you will have at most a certain number of elements, always use a fixed array instead of a dynamic one. The reason is that a fixed array does not need a length parameter in storage and thus saves one storage slot.
-[ Calldata instead of memory for external functions](https://github.com/beskay/gas-guide/blob/main/OPTIMIZATIONS.md#calldata-instead-of-memory-for-external-functions)
	- Calldata is cheaper than memory. If the input argument does not need to be modified, consider using calldata in external functions
- Function order matters
	- When calling a function, the EVM jumps through the list of function selectors until it finds a match. The function selectors are ordered in hexadecimal order and each jump costs 22 gas. If you have a lot of functions, you can save gas by ordering them in a way that the most commonly called functions are at the top.

## Delivables of the project

Core Deliverables:
    - A **Rust-based CLI tool** that can be run on various operating systems.
    - **Source code analysis** and **optimization algorithms** that 
	    - apply tight variable packing and efficient data type usage.
	    - **Struct packing optimization** feature that reorders struct fields to use storage slots efficiently.
	    - **Function selector optimization** that reorders function definitions based on call frequency to save gas.
    - **Automated code rewriting** capabilities that apply the suggested optimizations directly to the Solidity source code.
    - **Documentation** on how to install, configure, and use the tool.
    - **Test suite** that covers various Solidity contracts and ensures the reliability of the optimization process.
    - **Benchmarking reports** that demonstrate the gas savings achieved by the tool on sample contracts.
- Reach Goals
	- **Calldata optimization** that identifies and updates external function parameters to use `calldata` when beneficial.
	- **Dry-run mode** that outputs potential optimizations without altering the original code, for manual review.
	- **Smart contract metrics dashboard** that visualizes gas usage before and after optimizations.

## Domain Knowledge
### What are smart contracts? What is solidity?
Smart contracts are self-executing contracts with the terms of the agreement between buyer and seller being directly written into lines of code. These contracts are stored on a blockchain and automatically enforce the terms of the agreement. Solidity is the primary programming language used for developing smart contracts on the Ethereum blockchain. It is an object-oriented, high-level language designed to target the Ethereum Virtual Machine (EVM) and is influenced by C++, Python, and JavaScript.

Solidity is used to write programs that govern the behavior of accounts within the Ethereum state. It supports features such as inheritance, libraries, and complex user-defined types. Smart contracts, on the other hand, are programs that automate processes and govern the behavior of accounts within the Ethereum state. 

EVM Blockchain includes Ethereum, Polygon, Avalanche, Mantle ...
### What is gas and why is optimising gas necessary for blockchain?
Gas in the context of blockchain, particularly Ethereum, refers to the "fuel" that powers smart contract execution and transactions. It is a unit that measures the amount of computational effort required to perform operations on the blockchain network.

Gas is necessary for several reasons:

1. Preventing infinite computations: Gas ensures that computations on the blockchain are not infinite and can be completed within a reasonable amount of time. Without gas, malicious actors could create smart contracts or transactions that run forever, potentially causing the network to grind to a halt.

2. Mitigating spam and abuse: By requiring users to pay for gas, the blockchain network discourages spamming and abuse. Users need to spend a certain amount of cryptocurrency (Ether in the case of Ethereum) to execute actions on the network. This economic cost helps prevent malicious actors from overwhelming the network with frivolous or malicious transactions.

3. Resource allocation: Gas plays a role in resource allocation on the blockchain. Miners or validators prioritize transactions with higher gas fees because they earn those fees as a reward for including transactions in a block. This mechanism incentivizes users to set appropriate gas fees and helps allocate network resources efficiently.

Gas optimization is necessary for several reasons:

1. Cost reduction: Optimizing gas usage helps reduce the cost of executing smart contracts and transactions. Gas fees are paid in cryptocurrency, so minimizing gas consumption can make blockchain interactions more affordable for users.

2. Efficiency: Gas optimization improves the efficiency of blockchain networks. By reducing the amount of computational effort required for operations, transactions can be processed more quickly, and smart contracts can execute more efficiently.

3. Scalability: Gas optimization is crucial for scalability. By minimizing gas consumption, more transactions can be processed within a given block limit, leading to increased network throughput and improved scalability.

4. Enhanced user experience: Gas optimization can improve the user experience by reducing wait times and costs associated with executing transactions. It enables smoother and more cost-effective interactions with the blockchain.

To optimize gas usage, developers and users employ various techniques such as minimizing on-chain data, reducing complex computations, limiting external calls, using memory efficiently, applying looping carefully, implementing lazy evaluation, and optimizing data structures. These techniques help reduce gas consumption, improve efficiency, and make blockchain networks more scalable and cost-effective.

### Why Rust?
- Blazing Fast
- Strongly Typed
- Just for fun

