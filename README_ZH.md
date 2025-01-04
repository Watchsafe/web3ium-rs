# Web3ium
[![EN](https://img.shields.io/badge/language-EN-blue.svg)](README.md) [![CN](https://img.shields.io/badge/语言-中文-red.svg)](README_ZH.md)


Web3ium 是一个多链开发工具包，支持以太坊(EVM)、Solana 和 Bitcoin 等多条公链。该项目提供了统一的接口来处理不同链上的账户创建、签名验证等基础功能。

## 功能特性 ✨ 
Web3ium 目前支持以太坊 (EVM)、Solana 和 Bitcoin 三大主流公链。其中以太坊部分功能最为完备, 未来我也将持续拓展对更多链的支持。链与链之间的功能是独立的。

### 通用功能 (Common) 🌐
- 助记词生成与验证 (BIP39)
- 私钥管理
- ED25519 签名算法支持

### EVM
- 账户管理
  - 通过助记词创建账户
  - 通过私钥创建账户
- 交易签名
  - 支持 Legacy 交易
  - 支持 EIP1559 交易
- 消息签名
  - EIP191 签名
  - EIP712 类型化数据签名
- DEX 接口支持
  - Uniswap V2
  - Kyber
  - Odos

### Solana & Bitcoin
- 基础账户管理功能

## 项目结构
```
web3ium/
├── crates/
│ ├── common/ # 通用功能模块 🧩
│ ├── evm/ # 以太坊相关功能 ⟠
│ ├── solana/ # Solana 相关功能 ◎
│ └── bitcoin/ # Bitcoin 相关功能 ₿
```

## 开发状态 🚧

- [x] 通用功能模块
    - [x] 助记词生成
    - [x] 不同链的私钥生成
- [x] 以太坊基础功能
    - [x] EIP-191，EIP-712 签名
    - [x] 交易签名
    - [x] 交易解析
    - [ ] 模拟交易
    - [ ] 基于 cobo argus 的交易模块
    - [x] MEV(flashbot) 封装
    - [ ] DEXES
        - [x] kyberswap
        - [x] ODOS
        - [ ] uniswapV2/V3
        - [ ] curve
        - [ ] balancer
- [ ] Solana 功能开发
    - [x] message 签名
    - [x] 交易签名，模拟
    - [ ] MEV(jito) 封装
    - [ ] DEXES
        - [ ] Jup
        - [ ] Raydium
- [ ] Bitcoin 功能开发
    - [ ] message 签名
    - [ ] 交易签名
    - [ ] PSBT
    - [ ] DEXES
      - [ ] DotSwap
      - [ ] pizzaSwap
