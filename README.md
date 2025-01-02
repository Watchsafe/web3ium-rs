# Web3ium 🚀
[![EN](https://img.shields.io/badge/language-EN-blue.svg)](README.md) [![CN](https://img.shields.io/badge/语言-中文-red.svg)](README_zh.md)


Web3ium is a multi-chain development toolkit that supports Ethereum (EVM), Solana, Bitcoin, and other major public chains. The project provides unified interfaces to handle basic functions such as account creation and signature verification on different chains.

## Features ✨ 
Web3ium currently supports three mainstream public chains: Ethereum (EVM), Solana, and Bitcoin. Among them, Ethereum functions are the most complete. In the future, I will continue to expand support for more chains. The functions between chains are independent.

### Common Functions 🌐
- Mnemonic phrase generation and verification (BIP39)
- Private key management
- ED25519 signature algorithm support

### EVM
- Account management
  - Create an account with a mnemonic phrase
  - Create an account with a private key
- Transaction signing
  - Support for Legacy transactions
  - Support for EIP1559 transactions
- Message signing
  - EIP191 signing
  - EIP712 typed data signing
- DEX interface support
  - Uniswap V2
  - Kyber
  - Odos

### Solana & Bitcoin
- Basic account management functions

## Project Structure
```
web3ium/
├── crates/
│   ├── common/ # Common function modules 🧩
│   ├── evm/ # Ethereum-related functions ⟠
│   ├── solana/ # Solana-related functions ◎
│   └── bitcoin/ # Bitcoin-related functions ₿
```

## Development Status 🚧

- [x] Common function modules
    - [x] Mnemonic phrase generation
    - [x] Private key generation for different chains
- [x] Ethereum basic functions
    - [x] EIP-191, EIP-712 signing
    - [x] Transaction signing
    - [ ] Transaction parsing
    - [ ] Simulate transactions
    - [ ] MEV (flashbot) encapsulation
    - [ ] DEX
        - [ ] kyberswap
        - [ ] ODOS
        - [ ] uniswapV2/V3
        - [ ] curve
        - [ ] balancer
- [ ] Solana function development
    - [ ] Message signing
    - [ ] Transaction signing and simulation
    - [ ] MEV (jito) encapsulation
    - [ ] DEX
        - [ ] Jup
        - [ ] Raydium
- [ ] Bitcoin function development
    - [ ] Message signing
    - [ ] Transaction signing
    - [ ] PSBT
    - [ ] DEX