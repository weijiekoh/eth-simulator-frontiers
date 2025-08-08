# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an Ethereum simulator built in Rust that provides EVM functionality for testing and development. It consists of two main components:
- **eth-simulator**: Main binary application that demonstrates ETH transfers using a forked EVM
- **simular-core-fork**: A fork of the simular-core library that provides EVM simulation capabilities

## Development Commands

### Building
```bash
# Build the main project
cargo build

# Build with release optimizations
cargo build --release

# Build the simular-core library specifically
cd simular-core-fork && cargo build
```

### Running
```bash
# Run the ETH transfer simulation (requires sender, recipient, and ETH amount)
cargo run --bin simple_eth_transfer -- \
    --sender 0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045 \
    --recipient 0x1111000000000000000000000000000000000001 \
    --eth-amt 1
```

### Testing

TODO

## Architecture

### Core Components

**simular-core-fork/src/**
- `evm.rs`: Main EVM wrapper (`BaseEvm`) that provides high-level API for EVM operations
- `db/`: Database abstraction layer supporting both in-memory and forked storage
  - `fork.rs`: Handles state forking from remote Ethereum nodes
  - `in_memory_db.rs`: In-memory EVM state storage
  - `fork_backend.rs`: Backend for forked database operations
- `abi.rs`: Contract ABI handling and function encoding/decoding
- `snapshot.rs`: EVM state serialization and restoration

**Main Application**
- `src/simple_eth_transfer.rs`: CLI application demonstrating ETH transfers on forked state

### Key Design Patterns

1. **Storage Backend Abstraction**: The `StorageBackend` in `db/mod.rs` abstracts over in-memory and forked storage, allowing seamless switching between local simulation and mainnet forking.

2. **EVM State Management**: The `BaseEvm` struct provides a high-level API that wraps REVM with additional functionality like proof verification, serialization, and balance management.

3. **Fork-First Design**: The system is designed to easily fork from live Ethereum networks, pulling remote contract state into local simulation environment.

## Key Features

- **State Forking**: Fork from any Ethereum node (mainnet, testnet) at a specific block
- **EVM Simulation**: Full EVM compatibility using REVM under the hood
- **Proof Verification**: Verify account proofs from forked state
- **State Serialization**: Save/restore EVM state via JSON serialization
- **Contract Interaction**: Support for both raw calls and typed contract interactions

## Dependencies

- **REVM**: Core EVM implementation
- **Alloy**: Modern Ethereum types and utilities
- **Ethers**: Legacy Ethereum library (being migrated from)
- **Tokio**: Async runtime for network operations
- **Clap**: CLI argument parsing

## Network Configuration

The simulator defaults to using `https://eth.llamarpc.com` as the RPC endpoint for forking. Other endpoints, like Infura, may not work due to different responses.
