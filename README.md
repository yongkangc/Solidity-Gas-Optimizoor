# Solidity-Gas-Optimizoor

An high performance automated CLI tool that optimizes gas usage in Solidity smart contracts, focusing on storage and function call efficiency.

For more information on architecture and implementation, see the [docs](docs.md)

Disclaimer:

This code is a work in progress and can contain bugs. Use it at your own risk. Feature request and bug reports are welcome.

**Problem Statement:**

Solidity developers need tools to help them write gas-efficient code to minimize the execution cost of smart contracts on the blockchain. While there are some linters and optimizers available, there's a lack of tools specifically designed to analyze and suggest optimizations at both the source code and intermediate representation levels.

**Project Objective:**

The goal of this project is to design and implement a Rust-based tool that analyzes Solidity smart contracts, identifies patterns that lead to high gas usage, and suggests or automatically applies optimizations to improve gas efficiency.

## How does our tool optimize the gas of your smart contracts?

### Structured Data Packing

- **Concept**: Aligning struct members under 32 bytes together optimizes storage usage on the EVM.
- **Advantages**: This technique minimizes the number of `SLOAD` or `SSTORE` operations, slashing storage interaction costs by 50% or more when dealing with multiple struct values within a single slot.
- **Tooling**: A Rust CLI application scrutinizes Solidity struct layouts, reorganizing fields to use fewer storage slots. It respects existing comments and assumes `bytes32` for unrecognized types.
- **Documentation**: [Structured Data Packing Guidance](https://github.com/beskay/gas-guide/blob/main/OPTIMIZATIONS.md#storage-packing)

### Caching Storage Variables

- **Approach**: Utilize local variables to cache frequently accessed storage variables, reducing the number of expensive storage reads and writes.
- **Details**: Create a temporary local variable to store the value of a storage variable if it's accessed multiple times.
- **Source**: [Caching Storage Variables](https://www.rareskills.io/post/gas-optimization#viewer-8lubg)

### Calldata Efficiency

- **Gas Savings**: Leveraging calldata for unaltered external function inputs is more cost-effective than utilizing memory.
- **Implementation**: Analyze functions to ensure that inputs declared as `memory` are not modified. If unmodified, convert to `calldata`.
- **Reference**: [Calldata Efficiency Tips](https://github.com/beskay/gas-guide/blob/main/OPTIMIZATIONS.md#calldata-instead-of-memory-for-external-functions)

---

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

## Roadmap

### Week 8:

- [ ] Start on lexer and parser
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
