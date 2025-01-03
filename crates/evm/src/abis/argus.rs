use alloy_sol_types::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ICoboArgus,
    "src/abis/protocols/argus/cobo_argus.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ISafe,
    "src/abis/protocols/argus/safe.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IRoleManager,
    "src/abis/protocols/argus/roleManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IAuthorizer,
    "src/abis/protocols/argus/authorizer.json"
);
