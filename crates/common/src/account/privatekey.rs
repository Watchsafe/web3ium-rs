use alloy_signer_local::PrivateKeySigner;
use alloy_primitives::hex;
use solana_sdk::signature::Keypair;
use bitcoin::PrivateKey as BitcoinPrivateKey;

#[derive(Debug, Clone)]
pub struct PrivateKey {
    pub pk: String,
}

impl PrivateKey {
    pub fn evm_private_key() -> Result<Self, PrivateKey> {
        let signer = PrivateKeySigner::random();
        let private_key = signer.credential().to_bytes();
        let private_key_hex = hex::encode(private_key);
        Ok(Self { pk: private_key_hex})
    }


    pub fn random_solana() -> Self {
        Self {
            pk: Keypair::new().to_base58_string(),
        }
    }

    pub fn random_bitcoin(network: bitcoin::network::Network) -> Self {
        let private_key = BitcoinPrivateKey::generate(network);
        Self {
            pk: private_key.to_wif(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_private_key() {
        let private_key = PrivateKey::evm_private_key();
        println!("private_key: {:?}", private_key);
    }

    #[test]
    fn test_solana_private_key() {
        let private_key = PrivateKey::random_solana();
        println!("private_key: {:?}", private_key);
    }

    #[test]
    fn test_bitcoin_private_key() {
        let private_key = PrivateKey::random_bitcoin(bitcoin::network::Network::Bitcoin);
        println!("private_key: {:?}", private_key);
    }
}
