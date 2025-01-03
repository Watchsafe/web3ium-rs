use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use alloy_primitives::{hex::ToHexExt, keccak256};
use alloy_signer::{Signer, SignerSync};
use alloy_signer_local::PrivateKeySigner;

use reqwest::Client;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlashbotError {
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("Response error: {0}")]
    ResponseError(String),
    #[error("Signing error: {0}")]
    SigningError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Timeout error: {0}")]
    TimeoutError(String),
}

#[derive(Debug, Serialize, Clone)]
pub struct RequestConfig {
    pub timeout: Duration,
}

impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
struct FlashbotRequest {
    jsonrpc: String,
    id: i64,
    method: String,
    params: Vec<FlashbotRequestParams>,
}

#[derive(Debug, Serialize, Clone)]
struct FlashbotRequestParams {
    txs: Vec<String>,
    #[serde(rename = "blockNumber")]
    block_number: String,
    #[serde(rename = "minTimestamp")]
    min_timestamp: u64,
    #[serde(rename = "maxTimestamp")]
    max_timestamp: u64,
    #[serde(rename = "revertingTxHashes")]
    reverting_tx_hashes: Vec<String>,
    #[serde(rename = "replacementUuid")]
    replacement_uuid: String,
    builders: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PrivateTransactionPreferences {
    pub fast: bool,
    pub privacy: Option<PrivacyPreference>,
    pub validity: Option<ValidityPreference>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PrivacyPreference {
    pub hints: Option<Vec<String>>,
    pub builders: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ValidityPreference {
    pub refund: Option<Vec<RefundPreference>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RefundPreference {
    pub address: String,
    pub percent: i32,
}

#[derive(Debug, Serialize)]
pub(crate) struct PrivateTransactionParams {
    tx: String,
    #[serde(rename = "maxBlockNumber")]
    max_block_number: Option<String>,
    preferences: Option<PrivateTransactionPreferences>,
}

#[derive(Debug, Serialize)]
struct PrivateTransactionRequest {
    jsonrpc: String,
    id: i64,
    method: String,
    params: Vec<PrivateTransactionParams>,
}

#[derive(Debug, Deserialize)]
pub struct PrivateTransactionResponse {
    pub jsonrpc: String,
    pub id: i64,
    pub result: Option<String>,
    pub error: Option<ResponseError>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct FlashbotConfig {
    pub relay_url: String,
    pub builders: Vec<String>,
    pub request_config: RequestConfig,
}

impl Default for FlashbotConfig {
    fn default() -> Self {
        Self {
            relay_url: "https://relay.flashbots.net".to_string(),
            builders: vec![
                "flashbots".to_string(),
                "f1b.io".to_string(),
                "rsync".to_string(),
                "beaverbuild.org".to_string(),
                "builder0x69".to_string(),
                "Titan".to_string(),
                "EigenPhi".to_string(),
                "boba-builder".to_string(),
                "Gambit Labs".to_string(),
                "payload".to_string(),
                "Loki".to_string(),
                "BuildAI".to_string(),
                "JetBuilder".to_string(),
                "tbuilder".to_string(),
                "penguinbuild".to_string(),
                "bobthebuilder".to_string(),
                "BTCS".to_string(),
                "bloXroute".to_string(),
            ],
            request_config: RequestConfig::default(),
        }
    }
}

impl FlashbotConfig {
    pub fn validate(&self) -> Result<(), FlashbotError> {
        if self.relay_url.is_empty() {
            return Err(FlashbotError::RequestError("Empty relay URL".to_string()));
        }
        if self.builders.is_empty() {
            return Err(FlashbotError::RequestError(
                "No builders configured".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Flashbot {
    client: Client,
    config: FlashbotConfig,
}

impl Default for Flashbot {
    fn default() -> Self {
        Self::new()
    }
}

impl Flashbot {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(RequestConfig::default().timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            config: FlashbotConfig::default(),
        }
    }

    pub fn with_config(config: FlashbotConfig) -> Result<Self, FlashbotError> {
        config.validate()?;
        let client = Client::builder()
            .timeout(config.request_config.timeout)
            .build()
            .map_err(|e| FlashbotError::RequestError(e.to_string()))?;

        Ok(Self { client, config })
    }

    pub fn get_config(&self) -> FlashbotConfig {
        self.config.clone()
    }

    pub fn set_config(&mut self, config: FlashbotConfig) {
        self.config = config;
    }

    pub fn append_builder(&mut self, builder: String) {
        if !self.config.builders.contains(&builder) {
            self.config.builders.push(builder);
        }
    }

    pub fn remove_builder(&mut self, builder: &str) {
        self.config.builders.retain(|b| b != builder);
    }

    fn sign_request(&self, data: &str) -> Result<String, FlashbotError> {
        let mut signer = PrivateKeySigner::random();
        signer.set_chain_id(Some(1));
        let msg_hash = keccak256(data.as_bytes()).as_slice().encode_hex_with_prefix();
        let sig = signer
            .sign_message_sync(msg_hash.as_bytes())
            .map_err(|e| FlashbotError::SigningError(e.to_string()))?;

        Ok(format!(
            "{}:{}",
            signer.address(),
            sig.as_bytes().encode_hex_with_prefix()
        ))
    }

    pub async fn send_bundle(
        &self,
        bundle: Vec<String>,
        block: u64,
    ) -> Result<String, FlashbotError> {
        let id: i64 = { rand::thread_rng().gen() };
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let block_number = format!("0x{:x}", block);
        let body = FlashbotRequest {
            jsonrpc: "2.0".to_string(),
            id,
            method: "eth_sendBundle".to_string(),
            params: vec![FlashbotRequestParams {
                txs: bundle,
                block_number,
                min_timestamp: 0,
                max_timestamp: ts,
                reverting_tx_hashes: vec![],
                replacement_uuid: "".to_string(),
                builders: self.config.builders.clone(),
            }],
        };

        let data = serde_json::to_string(&body)
            .map_err(|e| FlashbotError::SerializationError(e.to_string()))?;
        let header = self.sign_request(&data)?;
        let response = self
            .client
            .post(&self.config.relay_url)
            .json(&body)
            .header("X-Flashbots-Signature", header)
            .send()
            .await
            .map_err(|e| FlashbotError::RequestError(e.to_string()))?;

        let response_text = response
            .text()
            .await
            .map_err(|e| FlashbotError::ResponseError(e.to_string()))?;

        Ok(response_text)
    }

    pub async fn send_private_transaction(
        &self,
        raw_tx_hex: String,
        max_block_number: Option<u64>,
        preferences: Option<PrivateTransactionPreferences>,
    ) -> Result<String, FlashbotError> {
        let max_block_number = max_block_number.map(|num| format!("0x{:x}", num));

        let params = PrivateTransactionParams {
            tx: raw_tx_hex,
            max_block_number,
            preferences,
        };

        let body = PrivateTransactionRequest {
            jsonrpc: "2.0".to_string(),
            id: rand::thread_rng().gen::<i64>(),
            method: "eth_sendPrivateTransaction".to_string(),
            params: vec![params],
        };

        let data = serde_json::to_string(&body)
            .map_err(|e| FlashbotError::SerializationError(e.to_string()))?;
        let header = self.sign_request(&data)?;

        let response: PrivateTransactionResponse = self
            .client
            .post(&self.config.relay_url)
            .header("X-Flashbots-Signature", header)
            .json(&body)
            .send()
            .await
            .map_err(|e| FlashbotError::RequestError(e.to_string()))?
            .json()
            .await
            .map_err(|e| FlashbotError::ResponseError(e.to_string()))?;

        match response.error {
            Some(err) => Err(FlashbotError::ResponseError(err.message)),
            None => Ok(response.result.unwrap_or_default()),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_bundle() {
        let config = FlashbotConfig {
            request_config: RequestConfig {
                timeout: Duration::from_secs(5),
            },
            ..Default::default()
        };

        let flashbot = Flashbot::with_config(config).unwrap();
        let bundle = vec![
            "0x02f86f0102843b9aca0085029e7822d68298f094d9e1459a7a482635700cbc20bbaf52d495ab9c9680841b55ba3ac080a0c199674fcb29f353693dd779c017823b954b3c69dffa3cd6b2a6ff7888798039a028ca912de909e7e6cdef9cdcaf24c54dd8c1032946dfa1d85c206b32a9064fe8".to_string(),
            "0xf8a91e850174f35da582d3ea94ec53bf9167f50cdeb3ae105f56099aaab9061f8380b844095ea7b3000000000000000000000000163a5ec5e9c32238d075e2d829fe9fa87451e3b70000000000000000000000000000000000000000000000000de0b6b3a764000025a03b59bc434bc3e660969a0d414352dfc0fac09f68ed259b8c2a9a2140aa5fbdcaa00cdcbfc8150ecc0b321b903dca58081915fe97ad625f4ad4d8fdf04cc33c9660".to_string()
        ];
        
        let mut block = 21541615u64;
        for i in 0..10 {
            println!("Sending bundle {} for block {}", i+1, block);
            
            match flashbot.send_bundle(bundle.clone(), block).await {
                Ok(response) => println!("Bundle {} result: {}", i+1, response),
                Err(e) => println!("Bundle {} error: {:?}", i+1, e),
            }
            
            block += 1;
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    }

    #[tokio::test]
    async fn test_send_private_transaction() {
        let config = FlashbotConfig {
            request_config: RequestConfig {
                timeout: Duration::from_secs(5),
            },
            ..Default::default()
        };

        let flashbot = Flashbot::with_config(config).unwrap();
        let tx = "0xf8a91e850174f35da582d3ea94ec53bf9167f50cdeb3ae105f56099aaab9061f8380b844095ea7b3000000000000000000000000163a5ec5e9c32238d075e2d829fe9fa87451e3b70000000000000000000000000000000000000000000000000de0b6b3a764000025a03b59bc434bc3e660969a0d414352dfc0fac09f68ed259b8c2a9a2140aa5fbdcaa00cdcbfc8150ecc0b321b903dca58081915fe97ad625f4ad4d8fdf04cc33c9660";
        
        let preferences = PrivateTransactionPreferences {
            fast: true,
            privacy: Some(PrivacyPreference {
                hints: None,
                builders: Some(vec!["flashbots".to_string()]),
            }),
            validity: None,
        };

        let result = flashbot.send_private_transaction(
            tx.to_string(),
            Some(21541639),
            Some(preferences)
        ).await;
        
        println!("Private tx result: {:?}", result);
    }

    #[test]
    fn test_timeout_config() {
        let config = FlashbotConfig {
            request_config: RequestConfig {
                timeout: Duration::from_secs(1),
            },
            ..Default::default()
        };

        let flashbot = Flashbot::with_config(config.clone()).unwrap();
        assert_eq!(flashbot.config.request_config.timeout, Duration::from_secs(1));
    }
}