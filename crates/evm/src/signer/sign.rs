use std::str::FromStr;

use crate::signer::account::EvmAccount;
use alloy_consensus::{TxEip1559, TxEip2930, TxEip4844, TxEip7702, TxLegacy};
use alloy_dyn_abi::eip712::TypedData;
use alloy_network::{EthereumWallet, TransactionBuilder};
use alloy_primitives::Address;
use alloy_primitives::{hex, PrimitiveSignature, TxKind};
use alloy_rlp::Encodable;
use alloy_rpc_types::TransactionRequest;
use alloy_signer::SignerSync;
use alloy_sol_types::SolStruct;
use thiserror::Error;

use serde::Serialize;

pub enum Transaction<'a> {
    Legacy(&'a mut TxLegacy),
    Eip1559(&'a mut TxEip1559),
    Eip2930(&'a mut TxEip2930),
    Eip4844(&'a mut TxEip4844),
    Eip7702(&'a mut TxEip7702),
}

#[derive(Error, Debug)]
pub enum EvmSignerError {
    #[error("Signature error: {0}")]
    SignatureError(String),
    #[error("Invalid address format: {0}")]
    InvalidAddress(String),
}

pub struct EvmSigner<'a> {
    account: &'a EvmAccount,
}

pub fn parse_address(address: &str) -> Result<Address, EvmSignerError> {
    if !address.starts_with("0x") {
        return Err(EvmSignerError::InvalidAddress(
            "Address must start with 0x".into(),
        ));
    }

    Address::from_str(address)
        .map_err(|_| EvmSignerError::InvalidAddress("Invalid address format".into()))
}

impl<'a> EvmSigner<'a> {
    pub fn new(account: &'a EvmAccount) -> Self {
        Self { account }
    }

    pub fn sign_eip191(&self, message: String) -> Result<String, EvmSignerError> {
        let signature = self
            .account
            .signer
            .sign_message_sync(message.as_bytes())
            .map_err(|e| EvmSignerError::SignatureError(e.to_string()))?;
        Ok(format!("0x{}", hex::encode(signature.as_bytes())))
    }

    pub fn recover_address_from_msg(
        message: &[u8],
        signature: &str,
    ) -> Result<Address, EvmSignerError> {
        // if signature starts with 0x, remove the 0x prefix
        let signature_bytes = if signature.starts_with("0x") {
            hex::decode(&signature[2..])
        } else {
            hex::decode(signature)
        }
        .map_err(|e| EvmSignerError::SignatureError(e.to_string()))?;

        let signature = PrimitiveSignature::try_from(signature_bytes.as_slice())
            .map_err(|e| EvmSignerError::SignatureError(e.to_string()))?;
        signature
            .recover_address_from_msg(message)
            .map_err(|e| EvmSignerError::SignatureError(e.to_string()))
    }

    pub fn sign_eip712<T: SolStruct + Serialize>(
        &self,
        domain: alloy_dyn_abi::Eip712Domain,
        data: &T,
    ) -> Result<String, EvmSignerError> {
        // check address
        if let Some(contract) = domain.verifying_contract {
            if contract.is_zero() || contract.len() != 20 {
                return Err(EvmSignerError::InvalidAddress(
                    "Verifying contract address is zero".into(),
                ));
            }
        }

        let typed_data = TypedData::from_struct(data, Some(domain));
        let signature = self
            .account
            .signer
            .sign_dynamic_typed_data_sync(&typed_data)
            .expect("Failed to sign dynamic typed data");
        Ok(format!("0x{}", hex::encode(signature.as_bytes())))
    }

    pub fn recover_eip712_address<T: SolStruct + Serialize>(
        domain: alloy_dyn_abi::Eip712Domain,
        data: &T,
        signature: &str,
    ) -> Result<Address, EvmSignerError> {
        let signature_bytes = hex::decode(&signature[2..])
            .map_err(|e| EvmSignerError::SignatureError(e.to_string()))?;

        let signature = PrimitiveSignature::try_from(signature_bytes.as_slice())
            .map_err(|e| EvmSignerError::SignatureError(e.to_string()))?;

        let typed_data = TypedData::from_struct(data, Some(domain));
        let hash = typed_data
            .eip712_signing_hash()
            .map_err(|e| EvmSignerError::SignatureError(e.to_string()))?;

        signature
            .recover_address_from_prehash(&hash)
            .map_err(|e| EvmSignerError::SignatureError(e.to_string()))
    }

    pub fn parse_address(address: &str) -> Result<Address, EvmSignerError> {
        if !address.starts_with("0x") {
            return Err(EvmSignerError::InvalidAddress(
                "Address must start with 0x".into(),
            ));
        }

        if address.len() != 42 {
            return Err(EvmSignerError::InvalidAddress(
                "Invalid address length, must be 42".into(),
            ));
        }

        Address::from_str(address)
            .map_err(|_| EvmSignerError::InvalidAddress("Invalid address format".into()))
    }

    pub async fn sign_transaction(&self, tx: Transaction<'_>) -> Result<String, EvmSignerError> {
        let signer = self.account.signer.clone();
        let wallet = EthereumWallet::from(signer);
        match tx {
            Transaction::Legacy(tx) => {
                let to_address = match tx.to {
                    TxKind::Call(addr) => addr,
                    TxKind::Create => Address::ZERO,
                };

                let tx_envelope = TransactionRequest::default()
                    .with_to(to_address)
                    .with_nonce(tx.nonce)
                    .with_chain_id(tx.chain_id.unwrap_or(1))
                    .with_value(tx.value)
                    .with_gas_limit(tx.gas_limit)
                    .with_gas_price(tx.gas_price)
                    .with_input(tx.input.clone())
                    .build(&wallet)
                    .await
                    .unwrap();

                let mut raw_data = Vec::new();
                tx_envelope.encode(&mut raw_data);
                Ok(format!("0x{}", hex::encode(raw_data)))
            }
            Transaction::Eip1559(tx) => {
                let to_address = match tx.to {
                    TxKind::Call(addr) => addr,
                    TxKind::Create => Address::ZERO,
                };

                let tx_envelope = TransactionRequest::default()
                    .with_to(to_address)
                    .with_nonce(tx.nonce)
                    .with_chain_id(tx.chain_id)
                    .with_value(tx.value)
                    .with_gas_limit(tx.gas_limit)
                    .with_max_priority_fee_per_gas(tx.max_priority_fee_per_gas)
                    .with_max_fee_per_gas(tx.max_fee_per_gas)
                    .with_input(tx.input.clone())
                    .with_access_list(tx.access_list.clone()).build(&wallet).await.unwrap();

                let mut raw_data = Vec::new();
                tx_envelope.encode(&mut raw_data);
                // remove the first two bytes, rlp encoded length and type flag
                if raw_data.len() > 2 {
                    raw_data = raw_data[2..].to_vec();
                }
                Ok(format!("0x{}", hex::encode(raw_data)))
            }
            _ => Err(EvmSignerError::SignatureError(
                "Unsupported transaction type".into(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use alloy_primitives::{Address, Bytes, U256};
    use alloy_sol_types::{sol, SolCall};
    use serde::Serialize;

    // sample struct
    sol! {
        #[derive(Debug, Serialize)]
        struct Message {
            address to;
            string contents;
        }
    }

    //  complex type
    sol! {
        #[derive(Debug, Serialize)]
        struct Fee {
            uint16 fee;
            address recipient;
        }

        #[derive(Debug, Serialize)]
        struct Order {
            address trader;
            uint8 side;
            address matchingPolicy;
            address collection;
            uint256 tokenId;
            uint256 amount;
            address paymentToken;
            uint256 price;
            uint256 listingTime;
            uint256 expirationTime;
            Fee[] fees;
            uint256 salt;
            bytes extraParams;
            uint256 nonce;
        }
    }

    sol! {
        function approve(address spender, uint256 amount) external returns (bool);
    }

    #[test]
    fn test_sign_and_recover() {
        let account = EvmAccount::from_private_key_hex(
            "c277f46a9cab407af9ac3cdf517b33f1d6e3615faf4a52a57ecc7b7d187a075d",
        )
        .unwrap();

        let address = account.signer.address();
        println!("private key address: {:?}", address);

        let signer = EvmSigner::new(&account);

        let message = "Hello, EIP-191!".to_string();
        let signature = signer.sign_eip191(message.clone()).unwrap();
        println!("Signature: {}", signature);

        let recovered_address =
            EvmSigner::recover_address_from_msg(message.as_bytes(), &signature).unwrap();

        println!("Recovered Address: {}", recovered_address);
        println!("Signer Address: {}", account.signer.address());
        assert_eq!(recovered_address, account.signer.address());
    }

    #[test]
    fn test_invalid_contract_address() {
        let account = EvmAccount::from_private_key_hex(
            "c277f46a9cab407af9ac3cdf517b33f1d6e3615faf4a52a57ecc7b7d187a075d",
        )
        .unwrap();
        let signer = EvmSigner::new(&account);

        let message = Message {
            to: Address::ZERO,
            contents: "Test".into(),
        };

        let result = (|| -> Result<_, EvmSignerError> {
            let domain = alloy_dyn_abi::Eip712Domain::new(
                Some("Test".into()),
                Some("1".into()),
                Some(U256::from(1)),
                Some(EvmSigner::parse_address("invalid_address")?),
                None,
            );

            signer.sign_eip712(domain, &message)
        })();

        assert!(matches!(result, Err(EvmSignerError::InvalidAddress(_))));

        if let Err(EvmSignerError::InvalidAddress(msg)) = result {
            println!("Caught expected error: {}", msg);
        }
    }

    #[test]
    fn test_invalid_contract_address_2() {
        let account = EvmAccount::from_private_key_hex(
            "c277f46a9cab407af9ac3cdf517b33f1d6e3615faf4a52a57ecc7b7d187a075d",
        )
        .unwrap();
        let signer = EvmSigner::new(&account);

        let message = Message {
            to: Address::ZERO,
            contents: "Test".into(),
        };

        let result = (|| -> Result<_, EvmSignerError> {
            let domain = alloy_dyn_abi::Eip712Domain::new(
                Some("Test".into()),
                Some("1".into()),
                Some(U256::from(1)),
                Some(EvmSigner::parse_address(
                    "0xc277f46a9cab407af9ac3cdf517b33f1d6e3615faf4a52a57ecc7b7d187a075d",
                )?),
                None,
            );

            signer.sign_eip712(domain, &message)
        })();

        assert!(matches!(result, Err(EvmSignerError::InvalidAddress(_))));

        if let Err(EvmSignerError::InvalidAddress(msg)) = result {
            println!("Caught expected error: {}", msg);
        }
    }

    #[test]
    fn test_sign_eip712_message() {
        let account = EvmAccount::from_private_key_hex(
            "c277f46a9cab407af9ac3cdf517b33f1d6e3615faf4a52a57ecc7b7d187a075d",
        )
        .unwrap();
        let signer = EvmSigner::new(&account);

        let _message: Message = Message {
            to: Address::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e").unwrap(),
            contents: "Hello, EIP-712!".into(),
        };

        let domain = alloy_dyn_abi::Eip712Domain::new(
            Some("Test".into()),
            Some("1".into()),
            Some(U256::from(1)),
            Some(Address::from_str("0x0000000000000000000000000000000000000001").unwrap()),
            None,
        );

        let signature = signer.sign_eip712(domain, &_message).unwrap();

        println!("Mail signature: {}", signature);
        assert!(signature.starts_with("0x"));
        assert_eq!(signature.len(), 132); // 0x + 130 chars
    }

    #[test]
    fn test_blur_order_sign() {
        let account = EvmAccount::from_private_key_hex(
            "c277f46a9cab407af9ac3cdf517b33f1d6e3615faf4a52a57ecc7b7d187a075d",
        )
        .unwrap();
        let signer = EvmSigner::new(&account);

        let order = Order {
            trader: Address::from_str("0x96ae51d5bed1071de4ff4a37390773bc51601da5").unwrap(),
            side: 0,
            matchingPolicy: Address::from_str("0x0000000000b92d5d043faf7cecf7e2ee6aaed232")
                .unwrap(),
            collection: Address::from_str("0xed5af388653567af2f388e6224dc7c4b3241c544").unwrap(),
            tokenId: U256::ZERO,
            amount: U256::from(1u64),
            paymentToken: Address::from_str("0x0000000000a39bb272e79075ade125fd351887ac").unwrap(),
            price: U256::from(100000000000000000u64),
            listingTime: U256::from(1679396503u64),
            expirationTime: U256::from(1710932502u64),
            fees: vec![],
            salt: U256::from_str_radix("6dc73d5dbea25abd6a3347e5e9886df6", 16).unwrap(),
            extraParams: Bytes::from_str("0x01").unwrap(),
            nonce: U256::ZERO,
        };

        let domain = alloy_dyn_abi::Eip712Domain::new(
            Some("Blur Exchange".into()),
            Some("1.0".into()),
            Some(U256::from(1)),
            Some(Address::from_str("0x000000000000ad05ccc4f10045630fb830b95127").unwrap()),
            None,
        );

        let signature = signer.sign_eip712(domain.clone(), &order).unwrap();

        println!("Blur Order EIP712 signature: {}", signature);

        let recovered_address =
            EvmSigner::recover_eip712_address(domain.clone(), &order, &signature).unwrap();

        println!("Signer address: {}", account.signer.address());
        println!("Recovered address: {}", recovered_address);
        assert_eq!(recovered_address, account.signer.address());
    }

    #[tokio::test]
    async fn test_legacy_approve_tx() {
        let account = EvmAccount::from_private_key_hex(
            "c277f46a9cab407af9ac3cdf517b33f1d6e3615faf4a52a57ecc7b7d187a075d",
        )
        .unwrap();
        let signer = EvmSigner::new(&account);

        let token_address =
            Address::from_str("0xec53bf9167f50cdeb3ae105f56099aaab9061f83").unwrap();
        let spender = Address::from_str("0x163a5ec5e9c32238d075e2d829fe9fa87451e3b7").unwrap();
        let amount = U256::from_str("1000000000000000000").unwrap();
        let approve_call = approveCall { spender, amount };

        let approve_data = approve_call.abi_encode();
        let mut legacy_tx = TxLegacy {
            nonce: 0u64,
            gas_price: 13_500_000_000u128,
            gas_limit: 54_250u64,
            to: TxKind::Call(token_address),
            value: U256::ZERO,
            input: approve_data.into(),
            chain_id: Some(1),
        };

        let raw_tx = signer
            .sign_transaction(Transaction::Legacy(&mut legacy_tx))
            .await
            .unwrap();

        println!("Signed approve transaction: {}", raw_tx);
        assert!(raw_tx.starts_with("0x"));
    }

    #[tokio::test]
    async fn test_eip1559_approve_tx() {
        let account = EvmAccount::from_private_key_hex(
            "c277f46a9cab407af9ac3cdf517b33f1d6e3615faf4a52a57ecc7b7d187a075d",
        )
        .unwrap();
        let signer = EvmSigner::new(&account);

        let token_address =
            Address::from_str("0xec53bf9167f50cdeb3ae105f56099aaab9061f83").unwrap();
        let spender = Address::from_str("0x163a5ec5e9c32238d075e2d829fe9fa87451e3b7").unwrap();
        let amount: alloy_primitives::Uint<256, 4> = U256::from_str("1000000000000000000").unwrap();
        let approve_call = approveCall { spender, amount };

        let approve_data = approve_call.abi_encode();
        let mut eip1559_tx: TxEip1559 = TxEip1559 {
            nonce: 1u64,
            max_fee_per_gas: 13_500_000_000u128,
            max_priority_fee_per_gas: 13_500_000_00u128,
            gas_limit: 54_250u64,
            to: TxKind::Call(token_address),
            value: U256::ZERO,
            input: approve_data.into(),
            chain_id: 1,
            access_list: vec![].into(),
        };

        let raw_tx = signer
            .sign_transaction(Transaction::Eip1559(&mut eip1559_tx))
            .await
            .unwrap();

        println!("Signed approve transaction: {}", raw_tx);
        assert!(raw_tx.starts_with("0x"));
    }
}
