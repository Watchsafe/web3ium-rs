use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use rand::rngs::OsRng;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Ed25519Error {
    #[error("Signing error: {0}")]
    SigningError(String),
    #[error("Verification error: {0}")]
    VerificationError(String),
}

pub struct Ed25519Signer {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl Ed25519Signer {
    pub fn new() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        Self { 
            signing_key,
            verifying_key,
        }
    }

    pub fn from_private_key(private_key: &[u8]) -> Result<Self, Ed25519Error> {
        let signing_key = SigningKey::try_from(private_key).map_err(|e| {
            Ed25519Error::SigningError(format!("Invalid private key: {}", e))
        })?;
        let verifying_key = signing_key.verifying_key();
        Ok(Self { 
            signing_key,
            verifying_key,
        })
    }

    /// 获取公钥（hex格式）
    pub fn public_key(&self) -> String {
        hex::encode(self.verifying_key.to_bytes())
    }

    /// 获取私钥（hex格式）
    pub fn private_key(&self) -> String {
        hex::encode(self.signing_key.to_bytes())
    }

    /// 签名消息
    pub fn sign(&self, message: &[u8]) -> String {
        let signature = self.signing_key.sign(message);
        hex::encode(signature.to_bytes())
    }

    /// 验证签名
    pub fn verify(&self, message: &[u8], signature: &str) -> Result<bool, Ed25519Error> {
        let sig_bytes = hex::decode(signature).map_err(|e| {
            Ed25519Error::VerificationError(format!("Invalid signature hex: {}", e))
        })?;
        
        let signature = Signature::try_from(sig_bytes.as_slice()).map_err(|e| {
            Ed25519Error::VerificationError(format!("Invalid signature bytes: {}", e))
        })?;

        self.verifying_key.verify_strict(message, &signature)
            .map_err(|e| Ed25519Error::VerificationError(format!("Signature verification failed: {}", e)))
            .map(|_| true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_sign() {
        let signer = Ed25519Signer::new();
        
        println!("Private Key: {}", signer.private_key());
        println!("Public Key: {}", signer.public_key());

        let message = b"Hello, Ed25519!";
        let signature = signer.sign(message);
        println!("Signature: {}", signature);

        let is_valid = signer.verify(message, &signature).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_sign_different_messages() {
        let signer = Ed25519Signer::new();

        let messages = vec![
            "Short message",
            "A longer message that needs to be signed",
            "测试中文消息",
            "あいうえお",
            "هذا نص تجريبي باللغة العربية لتجربة التنسيق",
            "1234567890!@#$%^&*()",
        ];

        for msg in messages {
            let signature = signer.sign(msg.as_bytes());
            println!("Message: {}", msg);
            println!("Signature: {}", signature);
            
            let is_valid = signer.verify(msg.as_bytes(), &signature).unwrap();
            assert!(is_valid);
        }
    }

    #[test]
    fn test_from_private_key() {
        let original_signer = Ed25519Signer::new();
        let private_key_hex = original_signer.private_key();
        
        let private_key_bytes = hex::decode(&private_key_hex).unwrap();
        let restored_signer = Ed25519Signer::from_private_key(&private_key_bytes).unwrap();
        
        assert_eq!(original_signer.public_key(), restored_signer.public_key());
        
        let message = b"Test message";
        let signature = restored_signer.sign(message);
        assert!(restored_signer.verify(message, &signature).unwrap());
    }
}