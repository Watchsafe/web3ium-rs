use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, thiserror::Error)]
pub enum KyberSwapError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid status code {status}: {message}")]
    InvalidStatus {
        status: reqwest::StatusCode,
        message: String,
    },
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, KyberSwapError>;

const BASE_URL: &str = "https://aggregator-api.kyberswap.com";

#[derive(Serialize, Deserialize)]
pub struct RouteResponse {
    pub code: i64,
    pub message: String,
    pub data: RouteData,
    #[serde(rename = "requestId")]
    pub request_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RouteData {
    #[serde(rename = "routeSummary")]
    pub route_summary: RouteSummary,
    #[serde(rename = "routerAddress")]
    pub router_address: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RouteSummary {
    #[serde(rename = "tokenIn")]
    pub token_in: String,
    #[serde(rename = "amountIn")]
    pub amount_in: String,
    #[serde(rename = "amountInUsd")]
    pub amount_in_usd: String,
    #[serde(rename = "tokenInMarketPriceAvailable")]
    pub token_in_market_price_available: bool,
    #[serde(rename = "tokenOut")]
    pub token_out: String,
    #[serde(rename = "amountOut")]
    pub amount_out: String,
    #[serde(rename = "amountOutUsd")]
    pub amount_out_usd: String,
    #[serde(rename = "tokenOutMarketPriceAvailable")]
    pub token_out_market_price_available: bool,
    pub gas: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    #[serde(rename = "gasUsd")]
    pub gas_usd: String,
    #[serde(rename = "extraFee")]
    pub extra_fee: ExtraFee,
    pub route: Vec<Vec<Route>>,
    #[serde(default)]
    pub checksum: String,
    #[serde(default)]
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExtraFee {
    #[serde(rename = "feeAmount")]
    pub fee_amount: String,
    #[serde(rename = "chargeFeeBy")]
    pub charge_fee_by: String,
    #[serde(rename = "isInBps")]
    pub is_in_bps: bool,
    #[serde(rename = "feeReceiver")]
    pub fee_receiver: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Route {
    pub pool: String,
    #[serde(rename = "tokenIn")]
    pub token_in: String,
    #[serde(rename = "tokenOut")]
    pub token_out: String,
    #[serde(rename = "limitReturnAmount")]
    pub limit_return_amount: String,
    #[serde(rename = "swapAmount")]
    pub swap_amount: String,
    #[serde(rename = "amountOut")]
    pub amount_out: String,
    pub exchange: String,
    #[serde(rename = "poolLength")]
    pub pool_length: i32,
    #[serde(rename = "poolType")]
    pub pool_type: String,
    #[serde(rename = "poolExtra")]
    pub pool_extra: OuterPoolExtra,
    pub extra: serde_json::Value,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OuterPoolExtra {
    #[serde(default)]
    #[serde(rename = "blockNumber")]
    pub block_number: i64,
    #[serde(default)]
    #[serde(rename = "tokenInIndex")]
    pub token_in_index: i32,
    #[serde(default)]
    #[serde(rename = "tokenOutIndex")]
    pub token_out_index: i32,
    #[serde(default)]
    pub underlying: bool,
    #[serde(default)]
    #[serde(rename = "TokenInIsNative")]
    pub token_in_is_native: bool,
    #[serde(default)]
    #[serde(rename = "TokenOutIsNative")]
    pub token_out_is_native: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_limit: Option<String>,
}

#[derive(Serialize)]
pub struct BuildRouteRequest {
    #[serde(rename = "routeSummary")]
    pub route_summary: RouteSummary,
    pub sender: String,
    pub recipient: String,
    pub deadline: i64,
    #[serde(rename = "slippageTolerance")]
    pub slippage_tolerance: i64,
    #[serde(rename = "enableGasEstimation")]
    pub enable_gas_estimation: bool,
    #[serde(rename = "ignoreCappedSlippage")]
    pub ignore_capped_slippage: bool,
}

#[derive(Serialize, Deserialize)]
pub struct BuildRouteResponse {
    pub code: i64,
    pub message: String,
    pub data: BuildRouteData,
    // pub request_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct BuildRouteData {
    #[serde(rename = "amountIn")]
    pub amount_in: String,
    #[serde(rename = "amountInUsd")]
    pub amount_in_usd: String,
    #[serde(rename = "amountOut")]
    pub amount_out: String,
    #[serde(rename = "amountOutUsd")]
    pub amount_out_usd: String,
    #[serde(rename = "gas")]
    pub gas: String,
    #[serde(rename = "gasUsd")]
    pub gas_usd: String,
    #[serde(rename = "outputChange")]
    pub output_change: OutputChange,
    #[serde(rename = "data")]
    pub data: String,
    #[serde(rename = "routerAddress")]
    pub router_address: String,
    #[serde(rename = "transactionValue")]
    pub transaction_value: String,
}

#[derive(Serialize, Deserialize)]
pub struct OutputChange {
    pub amount: String,
    pub percent: f64,
    pub level: i32,
}

pub struct KyberSwapClient {
    pub http_client: Client,
    pub base_url: String,
}

impl KyberSwapClient {
    /// Creates a new KyberSwap client.
    ///
    /// # Arguments
    ///
    /// * `base_url` - Optional base URL. If None, uses default BASE_URL
    /// * `chain` - Optional chain name. If None, uses "ethereum"
    pub fn new(base_url: Option<String>, chain: Option<String>) -> Self {
        let base_url = base_url.unwrap_or(BASE_URL.to_string());
        let chain = chain.unwrap_or("ethereum".to_string());

        // Create a client that behaves more like curl
        let http_client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")  // 设置为 curl 的 UA
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            base_url: format!("{}/{}", base_url, chain),
        }
    }

    /// Fetches routes for token swap
    pub async fn get_routes(
        &self,
        token_in: &str,
        token_out: &str,
        amount_in: &str,
    ) -> Result<RouteResponse> {
        let url = format!(
            "{}/api/v1/routes?tokenIn={}&tokenOut={}&amountIn={}",
            self.base_url, token_in, token_out, amount_in
        );

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(KyberSwapError::RequestError)?;

        if !response.status().is_success() {
            return Err(KyberSwapError::InvalidStatus {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        response
            .json::<RouteResponse>()
            .await
            .map_err(KyberSwapError::RequestError)
    }

    /// Builds a route for token swap
    pub async fn build_route(
        &self,
        route_summary: RouteSummary,
        sender: &str,
        recipient: &str,
        slippage_tolerance: i64,
        enable_gas_estimation: bool,
    ) -> Result<BuildRouteResponse> {
        let deadline = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 20 * 3600;

        let req_body = BuildRouteRequest {
            route_summary,
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            deadline: deadline as i64,
            slippage_tolerance: slippage_tolerance,
            enable_gas_estimation: enable_gas_estimation,
            ignore_capped_slippage: false,
        };

        let url = format!("{}/api/v1/route/build", self.base_url);

        let response = self
            .http_client
            .post(&url)
            .json(&req_body)
            .send()
            .await
            .map_err(KyberSwapError::RequestError)?;

        if !response.status().is_success() {
            return Err(KyberSwapError::InvalidStatus {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        response
            .json::<BuildRouteResponse>()
            .await
            .map_err(KyberSwapError::RequestError)
    }

    /// Sets a custom timeout for the HTTP client
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.http_client = ClientBuilder::new()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHAIN: &str = "ethereum";
    const SUSDE: &str = "0x9D39A5DE30e57443BfF2A8307A4256c8797A3497";
    const USDT: &str = "0xdac17f958d2ee523a2206206994597c13d831ec7";
    const DAI: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";
    const WST_ETH: &str = "0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0";
    const EZ_ETH: &str = "0xbf5495Efe5DB9ce00f80364C8B423567e58d2110";

    struct TestCase {
        name: &'static str,
        token_in: &'static str,
        token_out: &'static str,
        amount_in: &'static str,
        slippage_tolerance: i64,
        enable_gas_estimation: bool,
    }

    impl TestCase {
        fn new(
            name: &'static str,
            token_in: &'static str,
            token_out: &'static str,
            amount_in: &'static str,
            slippage_tolerance: i64,
            enable_gas_estimation: bool,
        ) -> Self {
            Self {
                name,
                token_in,
                token_out,
                amount_in,
                slippage_tolerance,
                enable_gas_estimation,
            }
        }
    }

    #[tokio::test]
    async fn test_kyber_swap_client_get_routes() -> Result<()> {
        let test_cases = vec![
            TestCase::new(
                "test get router by USDT",
                USDT,
                SUSDE,
                "2238451467827",
                10,
                false,
            ),
            TestCase::new("test get router by DAI", DAI, SUSDE, "100", 10, false),
            TestCase::new(
                "test get router by wstETH",
                WST_ETH,
                EZ_ETH,
                "10000000000000",
                10,
                false,
            ),
        ];

        let sender = "0xd46B96d15ffF9b2B17e9c788086f3159bD0e8355";
        let client = KyberSwapClient::new(None, Some(CHAIN.to_string()));

        for test_case in test_cases {
            println!("\nRunning test case: {}", test_case.name);

            // Get routes
            let route_response = client
                .get_routes(test_case.token_in, test_case.token_out, test_case.amount_in)
                .await?;

            println!("Route response:");
            println!("{}", serde_json::to_string_pretty(&route_response)?);
            println!("********************************************************");

            // Build route
            let build_response = client
                .build_route(
                    route_response.data.route_summary,
                    sender,
                    sender,
                    test_case.slippage_tolerance,
                    test_case.enable_gas_estimation,
                )
                .await?;

            println!("Build response:");
            println!("{}", serde_json::to_string_pretty(&build_response)?);
            println!("--------------------------------------------------------");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_kyber_swap_client_with_timeout() {
        let client = KyberSwapClient::new(None, Some(CHAIN.to_string()))
            .with_timeout(Duration::from_secs(5));

        let result = client.get_routes(USDT, SUSDE, "1000").await;
        assert!(
            result.is_ok(),
            "Request with custom timeout failed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_invalid_token_address() {
        let client = KyberSwapClient::new(None, Some(CHAIN.to_string()));
        let result = client.get_routes("invalid_address", SUSDE, "1000").await;
        assert!(result.is_err(), "Expected error for invalid token address");
    }
}
