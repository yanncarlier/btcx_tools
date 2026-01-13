//! Bitcoin Tools Library
//! Common types and utilities for Bitcoin-related command-line tools.

pub mod error;
pub mod types;
pub mod network;
pub mod utils;

// Re-exports
pub use bitcoin::{
    absolute, hashes, Address, Amount, Block, BlockHash, BlockHeader, EcdsaSighashType,
    OutPoint, PackedLockTime, Script, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
    consensus, hashes::hex::FromHex, secp256k1, Network, PublicKey,
};
pub use error::{Error, Result};
pub use types::*;

/// Common result type for the library
pub type Result<T> = std::result::Result<T, Error>;

/// Network configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BtcNetwork {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

impl From<BtcNetwork> for Network {
    fn from(network: BtcNetwork) -> Self {
        match network {
            BtcNetwork::Bitcoin => Network::Bitcoin,
            BtcNetwork::Testnet => Network::Testnet,
            BtcNetwork::Signet => Network::Signet,
            BtcNetwork::Regtest => Network::Regtest,
        }
    }
}

impl std::str::FromStr for BtcNetwork {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "mainnet" | "bitcoin" => Ok(BtcNetwork::Bitcoin),
            "testnet" => Ok(BtcNetwork::Testnet),
            "signet" => Ok(BtcNetwork::Signet),
            "regtest" => Ok(BtcNetwork::Regtest),
            _ => Err(Error::InvalidNetwork(s.to_string())),
        }
    }
}

/// Common error type for the library
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Bitcoin error: {0}")]
    Bitcoin(#[from] bitcoin::consensus::encode::Error),
    
    #[error("Hex decode error: {0}")]
    Hex(#[from] hex::FromHexError),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Invalid network: {0}")]
    InvalidNetwork(String),
    
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Transaction signing failed: {0}")]
    SigningError(String),
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("{0}")]
    Custom(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Custom(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Custom(s.to_string())
    }
}

/// Common utility functions
pub mod utils {
    use super::*;
    
    /// Parse a Bitcoin amount string (e.g., "0.001 BTC", "100000 sat")
    pub fn parse_amount(s: &str) -> Result<Amount> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.is_empty() {
            return Err(Error::Custom("Empty amount string".into()));
        }
        
        let value = parts[0].parse::<f64>().map_err(|_| {
            Error::Custom(format!("Invalid amount: {}", parts[0]))
        })?;
        
        let unit = parts.get(1).map(|s| s.to_lowercase()).unwrap_or_else(|| "btc".to_string());
        
        match unit.as_str() {
            "sat" | "sats" => Ok(Amount::from_sat(value as u64)),
            "btc" => Ok(Amount::from_btc(value).map_err(|_| {
                Error::Custom("Invalid BTC amount".into())
            })?),
            _ => Err(Error::Custom(format!("Unknown unit: {}", unit))),
        }
    }
    
    /// Format a Bitcoin amount as a string
    pub fn format_amount(amount: Amount, include_unit: bool) -> String {
        if include_unit {
            if amount.as_sat() < 10_000 {
                format!("{} sat", amount.as_sat())
            } else {
                format!("{:.8} BTC", amount.as_btc())
            }
        } else {
            amount.to_string()
        }
    }
    
    /// Parse a transaction ID string
    pub fn parse_txid(s: &str) -> Result<Txid> {
        Txid::from_hex(s).map_err(|e| {
            Error::InvalidTransaction(format!("Invalid transaction ID: {}", e))
        })
    }
    
    /// Parse a Bitcoin address
    pub fn parse_address(s: &str, network: Network) -> Result<Address> {
        s.parse::<Address>()
            .map_err(|_| Error::InvalidAddress(s.into()))
            .and_then(|addr| {
                if addr.network == network {
                    Ok(addr)
                } else {
                    Err(Error::InvalidAddress(format!(
                        "Address network mismatch: expected {:?}, got {:?}",
                        network, addr.network
                    )))
                }
            })
    }
}

/// Network-related functionality
pub mod network {
    use super::*;
    use reqwest::blocking::Client;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;
    
    /// Default timeout for network requests (in seconds)
    const DEFAULT_TIMEOUT: u64 = 30;
    
    /// Blockstream API client
    pub struct BlockstreamClient {
        base_url: String,
        client: Client,
    }
    
    impl BlockstreamClient {
        /// Create a new Blockstream API client
        pub fn new(network: BtcNetwork) -> Self {
            let base_url = match network {
                BtcNetwork::Bitcoin => "https://blockstream.info/api".to_string(),
                BtcNetwork::Testnet => "https://blockstream.info/testnet/api".to_string(),
                BtcNetwork::Signet => "https://mempool.space/signet/api".to_string(),
                BtcNetwork::Regtest => "http://localhost:3002/api".to_string(),
            };
            
            let client = Client::builder()
                .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
                .build()
                .expect("Failed to create HTTP client");
                
            BlockstreamClient { base_url, client }
        }
        
        /// Get UTXOs for an address
        pub fn get_utxos(&self, address: &str) -> Result<Vec<Utxo>> {
            let url = format!("{}/address/{}/utxo", self.base_url, address);
            let response = self.client.get(&url).send()?;
            
            if !response.status().is_success() {
                return Err(Error::Network(response.error_for_status().unwrap_err()));
            }
            
            let utxos: Vec<Utxo> = response.json()?;
            Ok(utxos)
        }
        
        /// Get transaction details
        pub fn get_transaction(&self, txid: &Txid) -> Result<TransactionInfo> {
            let url = format!("{}/tx/{}", self.base_url, txid);
            let response = self.client.get(&url).send()?;
            
            if !response.status().is_success() {
                return Err(Error::Network(response.error_for_status().unwrap_err()));
            }
            
            let tx_info: TransactionInfo = response.json()?;
            Ok(tx_info)
        }
        
        /// Broadcast a raw transaction
        pub fn broadcast_transaction(&self, tx_hex: &str) -> Result<Txid> {
            let url = format!("{}/tx", self.base_url);
            let response = self.client.post(&url).body(tx_hex.to_string()).send()?;
            
            if !response.status().is_success() {
                return Err(Error::Network(response.error_for_status().unwrap_err()));
            }
            
            let txid_str = response.text()?;
            Txid::from_hex(&txid_str).map_err(|e| {
                Error::InvalidTransaction(format!("Invalid transaction ID: {}", e))
            })
        }
        
        /// Get fee estimates
        pub fn get_fee_estimates(&self) -> Result<FeeEstimate> {
            let url = format!("{}/fee-estimates", self.base_url);
            let response = self.client.get(&url).send()?;
            
            if !response.status().is_success() {
                return Err(Error::Network(response.error_for_status().unwrap_err()));
            }
            
            let estimates: std::collections::HashMap<String, f64> = response.json()?;
            
            // Convert to sat/vB and find the best estimate
            let mut fee_estimate = FeeEstimate::default();
            
            for (blocks, fee_rate) in estimates {
                if let Ok(blocks) = blocks.parse::<u32>() {
                    let fee_rate = (fee_rate * 1000.0).round() as u64; // Convert to sat/kvB
                    
                    if blocks <= 6 {
                        fee_estimate.high_priority = fee_rate;
                    } else if blocks <= 12 {
                        fee_estimate.medium_priority = fee_rate;
                    } else if blocks <= 25 {
                        fee_estimate.low_priority = fee_rate;
                    }
                }
            }
            
            Ok(fee_estimate)
        }
    }
    
    /// UTXO information
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Utxo {
        pub txid: String,
        pub vout: u32,
        pub status: UtxoStatus,
        pub value: u64,
        #[serde(default)]
        pub address: Option<String>,
        #[serde(default)]
        pub script_pubkey: Option<String>,
    }
    
    /// UTXO status information
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct UtxoStatus {
        pub confirmed: bool,
        #[serde(default)]
        pub block_height: Option<u64>,
        #[serde(default)]
        pub block_hash: Option<String>,
        #[serde(default)]
        pub block_time: Option<u64>,
    }
    
    /// Transaction information
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct TransactionInfo {
        pub txid: String,
        pub version: i32,
        pub locktime: u32,
        pub size: usize,
        pub weight: usize,
        pub fee: u64,
        pub status: TransactionStatus,
        pub vin: Vec<TxInput>,
        pub vout: Vec<TxOutput>,
    }
    
    /// Transaction status
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct TransactionStatus {
        pub confirmed: bool,
        #[serde(default)]
        pub block_height: Option<u64>,
        #[serde(default)]
        pub block_hash: Option<String>,
        #[serde(default)]
        pub block_time: Option<u64>,
    }
    
    /// Transaction input
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct TxInput {
        pub txid: String,
        pub vout: u32,
        #[serde(default)]
        pub script_sig: Option<ScriptInfo>,
        #[serde(default)]
        pub witness: Option<Vec<String>>,
        #[serde(default)]
        pub sequence: u32,
        #[serde(default)]
        pub prevout: Option<TxOutput>,
    }
    
    /// Transaction output
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct TxOutput {
        pub scriptpubkey: String,
        #[serde(default)]
        pub scriptpubkey_asm: Option<String>,
        #[serde(default)]
        pub scriptpubkey_type: Option<String>,
        #[serde(default)]
        pub scriptpubkey_address: Option<String>,
        pub value: u64,
    }
    
    /// Script information
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct ScriptInfo {
        pub asm: String,
        pub hex: String,
    }
    
    /// Fee estimates
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct FeeEstimate {
        pub high_priority: u64,  // sat/kvB for next block
        pub medium_priority: u64, // sat/kvB for 3-6 blocks
        pub low_priority: u64,   // sat/kvB for 6+ blocks
    }
}

/// Common types used across the library
pub mod types {
    use super::*;
    
    /// UTXO information
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Utxo {
        pub txid: Txid,
        pub vout: u32,
        pub amount: Amount,
        pub script_pubkey: Script,
        #[serde(default)]
        pub address: Option<Address>,
        #[serde(default)]
        pub confirmations: Option<u32>,
    }
    
    /// Transaction input for signing
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SigningInput {
        pub txid: Txid,
        pub vout: u32,
        pub amount: Amount,
        pub script_pubkey: Script,
        pub redeem_script: Option<Script>,
        pub witness_script: Option<Script>,
    }
    
    /// Coin selection strategy
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum CoinSelectionStrategy {
        /// Select smallest UTXOs first (maximizes privacy)
        SmallestFirst,
        /// Select largest UTXOs first (minimizes fees)
        LargestFirst,
        /// Random selection (good for privacy)
        Random,
        /// Branch and bound algorithm for exact matches
        BranchAndBound,
    }
    
    impl Default for CoinSelectionStrategy {
        fn default() -> Self {
            CoinSelectionStrategy::BranchAndBound
        }
    }
    
    /// Fee estimation
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct FeeEstimate {
        pub sat_per_vbyte: f32,
        pub blocks: u32,
    }
    
    impl Default for FeeEstimate {
        fn default() -> Self {
            FeeEstimate {
                sat_per_vbyte: 1.0,
                blocks: 6,
            }
        }
    }
}
