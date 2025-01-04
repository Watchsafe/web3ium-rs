use crate::signer::account::SolanaAccount;
use solana_sdk::{
    signature::{Signature, Signer},
    transaction::Transaction,
};
use std::str::FromStr;
pub struct SolanaSigner<'a> {
    account: &'a SolanaAccount,
}

impl<'a> SolanaSigner<'a> {
    pub fn new(account: &'a SolanaAccount) -> Self {
        Self { account }
    }

    pub fn sign_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let message_bytes = message.as_bytes();
        let signature = self.account.signer.sign_message(message_bytes);
        Ok(signature.to_string())
    }

    pub fn verify_signature(
        &self,
        message: &str,
        signature_str: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let pubkey = self.account.signer.pubkey();
        let signature = Signature::from_str(signature_str)?;
        let message_bytes = message.as_bytes();
        Ok(signature.verify(pubkey.as_ref(), message_bytes))
    }

    pub fn sign_transaction(
        &self,
        unsigned_tx: Transaction,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut tx = unsigned_tx;
        tx.try_sign(&[&self.account.signer], tx.message.recent_blockhash)?;

        let serialized = bincode::serialize(&tx)?;
        Ok(bs58::encode(serialized).into_string())
    }

    pub fn deserialize_transaction(
        raw_tx: &str,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        let tx_data = bs58::decode(raw_tx)
            .into_vec()
            .map_err(|e| format!("Failed to decode base58: {}", e))?;

        let tx: Transaction = bincode::deserialize(&tx_data)
            .map_err(|e| format!("Failed to deserialize transaction: {}", e))?;

        Ok(tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{message::Message, system_instruction};

    #[test]
    fn test_message_sign_and_verify() -> Result<(), Box<dyn std::error::Error>> {
        let hex = "sPKbmNCtAUifiQs4R4CAuWfFZM7CJ8wBvkVioehLpjwpDcoSySU6Jtmw6ZiuG6Jx72yWB8A6LzN5jia5JkiHLHf";
        let account = SolanaAccount::from_private_key_hex(hex).unwrap();
        let signer = SolanaSigner::new(&account);

        let message = "Hello Solana!";

        let signature = signer.sign_message(message)?;
        println!("Message: {}", message);
        println!("Public Key: {}", account.signer.pubkey());
        println!("Signature: {}", signature);

        let is_valid = signer.verify_signature(message, &signature)?;
        println!("Signature Valid: {}", is_valid);

        assert!(is_valid);
        Ok(())
    }

    #[test]
    fn test_transaction_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let hex = "sPKbmNCtAUifiQs4R4CAuWfFZM7CJ8wBvkVioehLpjwpDcoSySU6Jtmw6ZiuG6Jx72yWB8A6LzN5jia5JkiHLHf";
        let account = SolanaAccount::from_private_key_hex(hex).unwrap();
        let signer = SolanaSigner::new(&account);

        // let to_pubkey = Pubkey::new_unique();
        let instruction = system_instruction::transfer(
            &account.signer.pubkey(),
            &account.signer.pubkey(),
            1000000,
        );

        let message = Message::new(&[instruction], Some(&account.signer.pubkey()));

        let tx = Transaction::new_unsigned(message);
        println!("Original Transaction: {:?}", tx);

        // 签名并序列化
        let serialized = signer.sign_transaction(tx)?;
        println!("Serialized Transaction (base58): {} \n", serialized);

        // 反序列化
        let deserialized = SolanaSigner::deserialize_transaction(&serialized)?;
        println!("Deserialized Transaction: {:?} \n", deserialized);
        println!(
            "Deserialized Transaction Signatures: {:?} \n",
            deserialized.signatures
        );
        println!(
            "Deserialized Transaction Message: {:?} \n",
            deserialized.message
        );

        // 验证签名
        assert!(deserialized.verify().is_ok());
        Ok(())
    }

    #[test]
    fn test_transaction_on_devnet() -> Result<(), Box<dyn std::error::Error>> {
        let rpc_client = RpcClient::new("https://api.devnet.solana.com");

        let hex = "sPKbmNCtAUifiQs4R4CAuWfFZM7CJ8wBvkVioehLpjwpDcoSySU6Jtmw6ZiuG6Jx72yWB8A6LzN5jia5JkiHLHf";
        let account = SolanaAccount::from_private_key_hex(hex)?;
        let signer = SolanaSigner::new(&account);

        println!("Signer Public Key: {}", account.signer.pubkey());

        let instruction = system_instruction::transfer(
            &account.signer.pubkey(),
            &account.signer.pubkey(),
            1000000,
        );

        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        println!("Recent Blockhash: {:?}", recent_blockhash);

        let message = Message::new_with_blockhash(
            &[instruction],
            Some(&account.signer.pubkey()),
            &recent_blockhash,
        );

        let tx = Transaction::new_unsigned(message);
        println!("Original Transaction: {:?}", tx);

        let serialized = signer.sign_transaction(tx)?;
        println!("Serialized Transaction (base58): {}\n", serialized);

        let deserialized = SolanaSigner::deserialize_transaction(&serialized)?;
        println!("Deserialized Transaction: {:?}\n", deserialized);
        println!(
            "Deserialized Transaction Signatures: {:?}\n",
            deserialized.signatures
        );
        println!(
            "Deserialized Transaction Message: {:?}\n",
            deserialized.message
        );

        assert!(deserialized.verify().is_ok());

        println!("Sending transaction to Devnet...");
        let signature = rpc_client.send_and_confirm_transaction(&deserialized)?;

        println!("Transaction successful!");
        println!("Transaction Signature: {}", signature);
        println!(
            "View transaction on Explorer: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );

        Ok(())
    }
}
