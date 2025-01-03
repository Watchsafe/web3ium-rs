use alloy_primitives::U256;
use serde::{Deserialize, Serialize};
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum OdosError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid status code {status}: {message}")]
    InvalidStatus {
        status: reqwest::StatusCode,
        message: String,
    },
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, OdosError>;

const BASE_URL: &str = "https://api.odos.xyz";

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceResponse {
    #[serde(rename = "currencyId")]
    pub currency_id: String,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputToken {
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputToken {
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    pub proportion: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteRequest {
    #[serde(rename = "chainId")]
    pub chain_id: i32,
    #[serde(rename = "inputTokens")]
    pub input_tokens: Vec<InputToken>,
    #[serde(rename = "outputTokens")]
    pub output_tokens: Vec<OutputToken>,
    #[serde(rename = "gasPrice")]
    pub gas_price: f64,
    #[serde(rename = "userAddr")]
    pub user_addr: String,
    #[serde(rename = "slippageLimitPercent")]
    pub slippage_limit_percent: f64,
    #[serde(rename = "sourceBlacklist")]
    pub source_blacklist: Vec<String>,
    #[serde(rename = "sourceWhitelist")]
    pub source_whitelist: Vec<String>,
    #[serde(rename = "poolBlacklist")]
    pub pool_blacklist: Vec<String>,
    #[serde(rename = "pathViz")]
    pub path_viz: bool,
    #[serde(rename = "referralCode")]
    pub referral_code: i32,
    pub compact: bool,
    #[serde(rename = "likeAsset")]
    pub like_asset: bool,
    #[serde(rename = "disableRFQs")]
    pub disable_rfqs: bool,
    pub simple: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub visible: bool,
    pub width: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub asset_id: String,
    pub asset_type: String,
    pub is_rebasing: bool,
    pub cgid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathLink {
    pub source: i32,
    pub target: i32,
    #[serde(rename = "sourceExtend")]
    pub source_extend: bool,
    #[serde(rename = "targetExtend")]
    pub target_extend: bool,
    pub label: String,
    pub value: f64,
    #[serde(rename = "nextValue")]
    pub next_value: f64,
    #[serde(rename = "stepValue")]
    pub step_value: f64,
    pub in_value: f64,
    pub out_value: f64,
    pub edge_len: i32,
    #[serde(rename = "sourceToken")]
    pub source_token: TokenInfo,
    #[serde(rename = "targetToken")]
    pub target_token: TokenInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathViz {
    pub nodes: Vec<Token>,
    pub links: Vec<PathLink>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteResponse {
    #[serde(rename = "inTokens")]
    pub in_tokens: Vec<String>,
    #[serde(rename = "outTokens")]
    pub out_tokens: Vec<String>,
    #[serde(rename = "inAmounts")]
    pub in_amounts: Vec<String>,
    #[serde(rename = "outAmounts")]
    pub out_amounts: Vec<String>,
    #[serde(rename = "gasEstimate")]
    pub gas_estimate: f64,
    #[serde(rename = "dataGasEstimate")]
    pub data_gas_estimate: i32,
    #[serde(rename = "gweiPerGas")]
    pub gwei_per_gas: f64,
    #[serde(rename = "gasEstimateValue")]
    pub gas_estimate_value: f64,
    #[serde(rename = "inValues")]
    pub in_values: Vec<f64>,
    #[serde(rename = "outValues")]
    pub out_values: Vec<f64>,
    #[serde(rename = "netOutValue")]
    pub net_out_value: f64,
    #[serde(rename = "priceImpact")]
    pub price_impact: f64,
    #[serde(rename = "percentDiff")]
    pub percent_diff: f64,
    #[serde(rename = "partnerFeePercent")]
    pub partner_fee_percent: f64,
    #[serde(rename = "pathId")]
    pub path_id: String,
    #[serde(rename = "pathViz")]
    pub path_viz: PathViz,
    #[serde(rename = "blockNumber")]
    pub block_number: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssembleRequest {
    #[serde(rename = "userAddr")]
    pub user_addr: String,
    #[serde(rename = "pathId")]
    pub path_id: String,
    pub simulate: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(default)]
    pub gas: i64,
    #[serde(default)]
    #[serde(rename = "gasPrice")]
    pub gas_price: i64,
    #[serde(default)]
    pub value: String,
    pub to: String,
    pub from: String,
    pub data: String,
    #[serde(default)]
    pub nonce: i64,
    #[serde(default)]
    #[serde(rename = "chainId")]
    pub chain_id: i32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Simulation {
    #[serde(default)]
    #[serde(rename = "isSuccess")]
    pub is_success: bool,
    #[serde(default)]
    #[serde(rename = "amountsOut")]
    pub amounts_out: Vec<U256>,
    #[serde(default)]
    #[serde(rename = "gasEstimate")]
    pub gas_estimate: i64,
    #[serde(default)]
    #[serde(rename = "simulationError")]
    pub simulation_error:  Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AssembleResponse {
    #[serde(default)]
    pub deprecated: Option<String>,
    #[serde(default)]
    #[serde(rename = "blockNumber")]
    pub block_number: i64,
    #[serde(default)]
    #[serde(rename = "gasEstimate")]
    pub gas_estimate: i64,
    #[serde(default)]
    #[serde(rename = "gasEstimateValue")]
    pub gas_estimate_value: f64,
    #[serde(default)]
    #[serde(rename = "inputTokens")]
    pub input_tokens: Vec<InputToken>,
    #[serde(default)]
    #[serde(rename = "outputTokens")]
    pub output_tokens: Vec<OutputTokenAssemble>,
    #[serde(default)]
    #[serde(rename = "netOutValue")]
    pub net_out_value: f64,
    #[serde(default)]
    #[serde(rename = "outValues")]
    pub out_values: Vec<String>,
    #[serde(rename = "transaction")]
    pub transaction: Transaction,
    #[serde(default)]
    pub simulation: Option<Simulation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputTokenAssemble {
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    pub amount: String,
}

pub struct OdosClient {
    http_client: Client,
    base_url: String,
}

impl OdosClient {
    pub fn new(base_url: Option<String>) -> Self {
        let base_url = base_url.unwrap_or(BASE_URL.to_string());
        
        let http_client = ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                headers.insert("Accept", "*/*".parse().unwrap());
                headers.insert("Origin", "https://app.odos.xyz".parse().unwrap());
                headers.insert("Referer", "https://app.odos.xyz/".parse().unwrap());
                headers
            })
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            base_url,
        }
    }

    pub async fn get_token_price(&self, chain_id: &str, token_addr: &str) -> Result<PriceResponse> {
        let url = format!(
            "{}/pricing/token/{}/{}",
            self.base_url, chain_id, token_addr
        );
        
        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(OdosError::RequestError)?;

        if !response.status().is_success() {
            return Err(OdosError::InvalidStatus {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        response.json::<PriceResponse>()
            .await
            .map_err(OdosError::RequestError)
    }

    pub async fn quote(&self, req: &QuoteRequest) -> Result<QuoteResponse> {
        let url = format!("{}/sor/quote/v2", self.base_url);

        let response = self.http_client
            .post(&url)
            .json(req)
            .send()
            .await
            .map_err(OdosError::RequestError)?;

        if !response.status().is_success() {
            return Err(OdosError::InvalidStatus {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        response.json::<QuoteResponse>()
            .await
            .map_err(OdosError::RequestError)
    }

    pub async fn assemble(&self, user_addr: &str, path_id: &str, is_simulate: bool) -> Result<AssembleResponse> {
        let url = format!("{}/sor/assemble", self.base_url);

        let req = AssembleRequest {
            user_addr: user_addr.to_string(),
            path_id: path_id.to_string(),
            simulate: is_simulate,
        };

        println!("\nAssemble request:");
        println!("URL: {}", url);
        println!("Request body: {}\n", serde_json::to_string_pretty(&req).unwrap());

        let response = self.http_client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(OdosError::RequestError)?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            println!("Error response: {}", error_text);
            return Err(OdosError::InvalidStatus {
                status,
                message: error_text,
            });
        }

        let response_text = response.text().await.map_err(OdosError::RequestError)?;
        println!("\n============== API Response Begin ==============");
        println!("{}", response_text);
        println!("============== API Response End ================\n");

        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json) => {
                println!("Parsed JSON:");
                println!("{}", serde_json::to_string_pretty(&json).unwrap());
            }
            Err(e) => {
                println!("Failed to parse response as JSON: {}", e);
            }
        }

        match serde_json::from_str::<AssembleResponse>(&response_text) {
            Ok(response) => Ok(response),
            Err(e) => {
                println!("\nDeserialization error: {}", e);
                Err(OdosError::JsonError(e))
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    const CHAIN_ID: &str = "1";
    const SUSDE: &str = "0x9D39A5DE30e57443BfF2A8307A4256c8797A3497";
    const DAI: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";
    const WST_ETH: &str = "0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0";
    const EZ_ETH: &str = "0xbf5495Efe5DB9ce00f80364C8B423567e58d2110";

    struct TokenPriceTestCase {
        name: &'static str,
        chain_id: &'static str,
        token_addr: &'static str,
    }

    #[tokio::test]
    async fn test_get_token_price() {
        let client = OdosClient::new(None);

        let test_cases = vec![
            TokenPriceTestCase {
                name: "test get router by DAI",
                chain_id: CHAIN_ID,
                token_addr: DAI,
            },
            TokenPriceTestCase {
                name: "test get router by USDC",
                chain_id: CHAIN_ID,
                token_addr: SUSDE,
            },
            TokenPriceTestCase {
                name: "test get router by wstETH",
                chain_id: CHAIN_ID,
                token_addr: WST_ETH,
            },
        ];

        for test in test_cases {
            println!("\nRunning test case: {}", test.name);
            
            match client.get_token_price(test.chain_id, test.token_addr).await {
                Ok(response) => {
                    println!("GetTokenPrice response: {}", serde_json::to_string_pretty(&response).unwrap());
                }
                Err(e) => {
                    panic!("Failed to get token price: {:?}", e);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_quote() {
        let client = OdosClient::new(None);

        let test_cases = vec![
            (
                "test get router by DAI",
                QuoteRequest {
                    chain_id: 1,
                    input_tokens: vec![InputToken {
                        token_address: DAI.to_string(),
                        amount: "1000000000000000000".to_string(),
                    }],
                    output_tokens: vec![OutputToken {
                        token_address: SUSDE.to_string(),
                        proportion: 1.0,
                    }],
                    gas_price: 6.27,
                    user_addr: "0x163A5EC5e9C32238d075E2D829fE9fA87451e3b7".to_string(),
                    slippage_limit_percent: 0.1,
                    source_blacklist: vec![],
                    source_whitelist: vec![],
                    pool_blacklist: vec![],
                    path_viz: true,
                    referral_code: 1,
                    compact: true,
                    like_asset: true,
                    disable_rfqs: false,
                    simple: false,
                }
            ),
            (
                "test get router by USDC",
                QuoteRequest {
                    chain_id: 1,
                    input_tokens: vec![InputToken {
                        token_address: SUSDE.to_string(),
                        amount: "1000000000000000000".to_string(),
                    }],
                    output_tokens: vec![OutputToken {
                        token_address: DAI.to_string(),
                        proportion: 1.0,
                    }],
                    gas_price: 6.27,
                    user_addr: "0x163A5EC5e9C32238d075E2D829fE9fA87451e3b7".to_string(),
                    slippage_limit_percent: 0.1,
                    source_blacklist: vec![],
                    source_whitelist: vec![],
                    pool_blacklist: vec![],
                    path_viz: true,
                    referral_code: 1,
                    compact: true,
                    like_asset: true,
                    disable_rfqs: false,
                    simple: false,
                }
            ),
            (
                "test get router by wstETH",
                QuoteRequest {
                    chain_id: 1,
                    input_tokens: vec![InputToken {
                        token_address: WST_ETH.to_string(),
                        amount: "1000000000000000000".to_string(),
                    }],
                    output_tokens: vec![OutputToken {
                        token_address: EZ_ETH.to_string(),
                        proportion: 1.0,
                    }],
                    gas_price: 6.27,
                    user_addr: "0x163A5EC5e9C32238d075E2D829fE9fA87451e3b7".to_string(),
                    slippage_limit_percent: 0.1,
                    source_blacklist: vec![],
                    source_whitelist: vec![],
                    pool_blacklist: vec![],
                    path_viz: true,
                    referral_code: 1,
                    compact: true,
                    like_asset: true,
                    disable_rfqs: false,
                    simple: false,
                }
            ),
        ];

        for (name, request) in test_cases {
            println!("\nRunning test case: {}", name);

            match client.quote(&request).await {
                Ok(response) => {
                    println!("Quote response: {}", serde_json::to_string_pretty(&response).unwrap());
                }
                Err(e) => {
                    panic!("Failed to get quote: {:?}", e);
                }
            }
        }
    }

    struct AssembleTestCase {
        name: &'static str,
        user_addr: &'static str,
        path_id: &'static str,
        simulate: bool,
    }

    #[tokio::test]
    async fn test_assemble() {
        let client = OdosClient::new(None);

        let test_cases = vec![
            AssembleTestCase {
                name: "test assemble ETH -> sUSDe",
                user_addr: "0x163A5EC5e9C32238d075E2D829fE9fA87451e3b7",
                path_id: "6ace6e4f6028d0103b5df5f6d78cf7f8",
                simulate: false,
            },
            AssembleTestCase {
                name: "test assemble DAI -> sUSDe",
                user_addr: "0x163A5EC5e9C32238d075E2D829fE9fA87451e3b7",
                path_id: "1af07a2ff47096a38da76949da5bf130",
                simulate: true,
            },
        ];

        for test in test_cases {
            println!("\nRunning test case: {}", test.name);

            match client.assemble(test.user_addr, test.path_id, test.simulate).await {
                Ok(response) => {
                    println!("Assemble response: {}", serde_json::to_string_pretty(&response).unwrap());
                }
                Err(e) => {
                    panic!("Failed to assemble: {:?}", e);
                }
            }
        }
    }
}