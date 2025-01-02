use bip39::{Mnemonic as Bip39Mnemonic, Language};
use thiserror::Error;
use std::fmt;

pub const DEFAULT_WORD_COUNT: u32 = 24;
pub const DEFAULT_LANGUAGE: Language = Language::English;

#[derive(Error, Debug)]
pub enum MnemonicError {
    #[error("Invalid word count: {0}, must be one of [12, 15, 18, 21, 24]")]
    InvalidWordCount(u32),
    #[error("Invalid mnemonic phrase")]
    InvalidPhrase,
    #[error("Bip39 error: {0}")]
    Bip39Error(#[from] bip39::Error),
}

pub struct Mnemonic {
    inner: Bip39Mnemonic,
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Mnemonic {
    pub fn new() -> Result<Self, MnemonicError> {
        Self::with_word_count(DEFAULT_LANGUAGE, DEFAULT_WORD_COUNT)
    }

    pub fn with_word_count(language: Language, word_count: u32) -> Result<Self, MnemonicError> {
        let inner = Bip39Mnemonic::generate_in(language, word_count as usize).unwrap();
        Ok(Self { inner })
    }

    pub fn from_phrase(phrase: &str, language: Language) -> Result<Self, MnemonicError> {
        let inner = Bip39Mnemonic::parse_in(language, phrase)
            .map_err(MnemonicError::Bip39Error)?;
        Ok(Self { inner })
    }

    pub fn is_valid(phrase: &str, language: Language) -> bool {
        Bip39Mnemonic::parse_in(language, phrase).is_ok()
    }

    pub fn to_seed(&self, passphrase: Option<&str>) -> [u8; 64] {
        self.inner.to_seed(passphrase.unwrap_or(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_new_mnemonic() {
        let mnemonic = Mnemonic::new().unwrap();
        let phrase = mnemonic.to_string();
        let words: Vec<&str> = phrase.split_whitespace().collect();
        println!("Generated phrase: {}", phrase);
        println!("Words count: {}", words.len());
        assert_eq!(words.len(), DEFAULT_WORD_COUNT as usize);
    }

    #[test]
    fn test_custom_word_count() {
        let word_counts = [12, 15, 18, 21, 24];
        for count in word_counts {
            let mnemonic = Mnemonic::with_word_count(Language::English, count).unwrap();
            let phrase = mnemonic.to_string();  // 先绑定到变量
            let words: Vec<&str> = phrase.split_whitespace().collect();
            assert_eq!(words.len(), count as usize);
        }
    }

    #[test]
    fn test_invalid_word_count() {
        let invalid_counts = [0, 11, 13, 25, 30];
        for count in invalid_counts {
            let result = Mnemonic::with_word_count(Language::English, count);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_from_phrase() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic: Mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
        assert_eq!(mnemonic.to_string(), phrase);
    }

    #[test]
    fn test_invalid_phrase() {
        let invalid_phrase = "invalid phrase that is not a valid mnemonic";
        let result = Mnemonic::from_phrase(invalid_phrase, Language::English);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid() {
        let valid_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        assert!(Mnemonic::is_valid(valid_phrase, Language::English));

        let invalid_phrase = "invalid phrase that is not a valid mnemonic";
        assert!(!Mnemonic::is_valid(invalid_phrase, Language::English));
    }

    #[test]
    fn test_to_seed() {
        let mnemonic = Mnemonic::new().unwrap();
        
        let seed1 = mnemonic.to_seed(None);
        assert_eq!(seed1.len(), 64);

        let seed2 = mnemonic.to_seed(Some("password"));
        assert_eq!(seed2.len(), 64);
        
        assert_ne!(seed1, seed2);
    }
}