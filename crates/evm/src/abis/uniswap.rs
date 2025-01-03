use alloy_sol_types::sol;

// https://docs.uniswap.org/contracts/v2/reference/smart-contracts/v2-deployments
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IUniswapV2Factory,
    "src/abis/protocols/uniswapV2/factory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IUniswapV2Router,
    "src/abis/protocols/uniswapV2/router.json"
);

// https://docs.uniswap.org/contracts/v3/reference/deployments/ethereum-deployments
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IUniswapV3Factory,
    "src/abis/protocols/uniswapV3/factory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IUniswapV3Router,
    "src/abis/protocols/uniswapV3/router.json"
);
