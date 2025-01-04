#![allow(unused_imports)]


use bip39::{Mnemonic, Language};
use solana_sdk::derivation_path::DerivationPath;
use solana_sdk::signature::{Keypair, keypair_from_seed_and_derivation_path};
use solana_sdk::signer::Signer;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolanaAccountError {
    #[error("Invalid mnemonic phrase")]
    InvalidMnemonic,
    #[error("Signer error: {0}")]
    SignerError(String),
    #[error("Invalid private key hex")]
    InvalidPrivateKeyHex,
}

pub struct SolanaAccount {
    pub signer: Keypair,
}

impl SolanaAccount {
    pub fn from_mnemonic(
        phrase: &str, 
        password: &str, 
        index: u32
    ) -> Result<Self, SolanaAccountError> {
        let mnemonic = Mnemonic::parse_in(Language::English, phrase).unwrap();
        let seed = mnemonic.to_seed(password);
        
        let derivation_path = DerivationPath::new_bip44(Some(index), Some(0));
        let keypair = keypair_from_seed_and_derivation_path(&seed, Some(derivation_path)).unwrap();
        Ok(Self { signer: keypair })
    }

    pub fn random_private_key() -> Result<Self, SolanaAccountError> {
        let keypair = Keypair::new();
        Ok(Self { signer: keypair })
    }

    pub fn from_private_key_hex(hex: &str) -> Result<Self, SolanaAccountError> {
        let keypair = Keypair::from_base58_string(hex);
        Ok(Self { signer: keypair })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_mnemonic() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let password = "";
        let index = 5;
        let account = SolanaAccount::from_mnemonic(mnemonic, password, index).unwrap();
        assert_eq!(account.signer.pubkey().to_string(), "2EUrWmf5xMmWER9BtDbXbGbZjoL7R3eTDMXYR6H6cKPj");
    }

    #[test]
    fn test_from_private_key_hex() {
        let hex = "2yj1p1pVstUJ3iVVJt4NjqYf6ikb3mK2ZAkxwYiZNUc5QECNhBxmvoRMpyzoRgyYMpYGbS8tcPmwriSTZ6nUd81B";
        let account = SolanaAccount::from_private_key_hex(hex).unwrap();
        assert_eq!(account.signer.pubkey().to_string(), "2EUrWmf5xMmWER9BtDbXbGbZjoL7R3eTDMXYR6H6cKPj");
    }
}