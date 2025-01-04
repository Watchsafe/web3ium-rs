use alloy_signer_local::{coins_bip39::English, MnemonicBuilder, LocalSigner, LocalSignerError, PrivateKeySigner};
use alloy_signer::Signer;
use alloy_primitives::hex;
use k256::ecdsa::SigningKey;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvmAccountError {
    #[error("Invalid mnemonic phrase")]
    InvalidMnemonic,
    #[error("Signer error: {0}")]
    SignerError(String),
    #[error("Invalid private key hex")]
    InvalidPrivateKeyHex,
}

impl From<LocalSignerError> for EvmAccountError {
    fn from(err: LocalSignerError) -> Self {
        EvmAccountError::SignerError(err.to_string())
    }
}

pub struct EvmAccount {
    pub signer: LocalSigner<SigningKey>,
}

impl EvmAccount {
    pub fn from_mnemonic(
        phrase: &str, 
        password: &str, 
        index: u32
    ) -> Result<Self, EvmAccountError> {
        let signer = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .password(password)
            .index(index)?
            .build()?;
        Ok(Self { signer })
    }

    pub fn random_private_key() -> Result<Self, EvmAccountError> {
        let signer = PrivateKeySigner::random();
        // let private_key = signer.credential().to_bytes();
        // let private_key_hex = hex::encode(private_key);
        // println!("private_key_hex: {:?}", private_key_hex);
        Ok(Self { signer })
    }

    pub fn from_private_key_hex(
        private_key_hex: &str
    ) -> Result<Self, EvmAccountError> {
        // if has 0x prefix, remove it
        let private_key_hex = if private_key_hex.starts_with("0x") {
            &private_key_hex[2..]
        } else {
            private_key_hex
        };

        let signer: PrivateKeySigner = private_key_hex.parse().unwrap();
        
        // signer.set_chain_id(Some(1));

        // let signer = signer.set_chain_id(Some(1));
        Ok(Self { signer })
    }

    // from keystore


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_mnemonic() {
        let signer = EvmAccount::from_mnemonic("work man father plunge mystery proud hollow address reunion sauce theory bonus", "", 0).unwrap();
        let chain_id = signer.signer.chain_id();
        let address = signer.signer.address();
        println!("chain_id: {:?}", chain_id);
        println!("address: {:?}", address);
    }

    #[test]
    fn test_random_private_key() {
        let signer = EvmAccount::random_private_key().unwrap();
        let address = signer.signer.address();
        println!("address: {:?}", address);
    }

    #[test]
    fn test_from_private_key_hex() {
        let signer = EvmAccount::from_private_key_hex("c277f46a9cab407af9ac3cdf517b33f1d6e3615faf4a52a57ecc7b7d187a075d").unwrap();
        let address = signer.signer.address();
        println!("address: {:?}", address);
    }

    #[test]
    fn test_from_private_key_hex_prefix_0x  () {
        let signer = EvmAccount::from_private_key_hex("0xc277f46a9cab407af9ac3cdf517b33f1d6e3615faf4a52a57ecc7b7d187a075d").unwrap();
        let address = signer.signer.address();
        println!("address: {:?}", address);
    }
}

