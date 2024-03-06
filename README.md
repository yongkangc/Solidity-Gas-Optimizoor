# Solidity-Gas-Optimizoor

An automated CLI tool that optimizes gas usage in Solidity smart contracts, focusing on storage and function call efficiency.

For more information, see the [research section](research.md)

**Problem Statement:**

Solidity developers need tools to help them write gas-efficient code to minimize the execution cost of smart contracts on the blockchain. While there are some linters and optimizers available, there's a lack of tools specifically designed to analyze and suggest optimizations at both the source code and intermediate representation levels.

**Project Objective:**

The goal of this project is to design and implement a Rust-based tool that analyzes Solidity smart contracts, identifies patterns that lead to high gas usage, and suggests or automatically applies optimizations to improve gas efficiency.

## How does our tool optimize the gas of your smart contracts?

Automated Rust-based CLI tool that optimizes gas of solidity code by

- [struct packing](https://github.com/beskay/gas-guide/blob/main/OPTIMIZATIONS.md#storage-packing)
  - We can save storage by ordering variables that use less than 32 bytes next to each other.
  - Storage packing is particularly useful when reading or writing multiple values in the same storage slot. In such cases, only a single SLOAD or SSTORE operation is needed, significantly reducing the cost of accessing storage variables by half or more. This situation commonly occurs with structs:
  - parses Solidity structs and packs the fields efficiently to reduce the number of storage slots they use. It also adds struct packing comments to clearly indicate how the fields are packed.
  - It can deal with comments and whitespace in the struct definition, and will preserve them in the output. It handles unknown types by assuming they cannot be packed, treating they as `bytes32`.
- [tight packing](https://fravoll.github.io/solidity-patterns/tight_variable_packing.html)
  - Save gas by using smaller data types (e.g. `bytes16`, `uint32`) when possible, as the EVM can then pack them together in one single 32 byte slot and therefore use less storage. Gas is then saved because the EVM can combine multiple reads or writes into one single operation. The underlying behavior is also referred to as “tight packing” and is unfortunately, until the time of writing, not automatically achieved by the optimizer.
- Fixed size variables are cheaper than dynamic size variables - As a general rule, use bytes for arbitrary-length raw byte data and string for arbitrary-length string (UTF-8) data. If you can limit the length to a certain number of bytes, always use one of the value types (bytes1 to bytes32) because they are much cheaper. - The same applies for arrays: If you know that you will have at most a certain number of elements, always use a fixed array instead of a dynamic one. The reason is that a fixed array does not need a length parameter in storage and thus saves one storage slot. -[ Calldata instead of memory for external functions](https://github.com/beskay/gas-guide/blob/main/OPTIMIZATIONS.md#calldata-instead-of-memory-for-external-functions) - Calldata is cheaper than memory. If the input argument does not need to be modified, consider using calldata in external functions
- Function order matters
  - When calling a function, the EVM jumps through the list of function selectors until it finds a match. The function selectors are ordered in hexadecimal order and each jump costs 22 gas. If you have a lot of functions, you can save gas by ordering them in a way that the most commonly called functions are at the top.
- [Caching Storage Variable](https://www.rareskills.io/post/gas-optimization#viewer-8lubg)
  - Cache   

## Delivables of the project

### Core Deliverables:

- A **Rust-based CLI tool** that can be run on various operating systems.
- **Source code analysis** and **optimization algorithms** that:
  - Apply tight variable packing and efficient data type usage.
  - **Struct packing optimization** feature that reorders struct fields to use storage slots efficiently.
  - **Function selector optimization** that reorders function definitions based on call frequency to save gas.
- **Automated code rewriting** capabilities that apply the suggested optimizations directly to the Solidity source code.
- **Documentation** on how to install, configure, and use the tool.
- **Test suite** that covers various Solidity contracts and ensures the reliability of the optimization process.
- **Benchmarking reports** that demonstrate the gas savings achieved by the tool on sample contracts.

### Reach Goals

- **Calldata optimization** that identifies and updates external function parameters to use `calldata` when beneficial.
- **Dry-run mode** that outputs potential optimizations without altering the original code, for manual review.
- **Smart contract metrics dashboard** that visualizes gas usage before and after optimizations.

## Roadmap

### Week 8:
- [ ]  Start on lexer and parser
    - look at the optimisations u need to do and look at the tokens we neeed
        - struct packing
            - struct, parathesis, variable name, type identifier
        - storage cache
            - identifying read and write
                - what can trigger a read / write
            - function
            - limit it to value types only
                - start with integer
                    - we can start with value types (pass by value)
