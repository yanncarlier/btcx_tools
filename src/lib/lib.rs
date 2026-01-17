//! Bitcoin Tools Library
//! Common types and utilities for Bitcoin-related command-line tools.

pub mod error;
pub mod types;

// Re-exports
pub use bitcoin::{
    absolute, hashes, Address, Amount, Block, BlockHash, OutPoint, Script, Sequence,
    Transaction, TxIn, TxOut, Txid, Witness, consensus, hashes::hex::FromHex, secp256k1,
    Network, PublicKey,
};
pub use error::{Error, Result};
pub use types::*;

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
    type Err = error::Error;

    fn from_str(s: &str) -> std::result::Result<Self, error::Error> {
        match s.to_lowercase().as_str() {
            "mainnet" | "bitcoin" => Ok(BtcNetwork::Bitcoin),
            "testnet" => Ok(BtcNetwork::Testnet),
            "signet" => Ok(BtcNetwork::Signet),
            "regtest" => Ok(BtcNetwork::Regtest),
            _ => Err(error::Error::InvalidNetwork(s.to_string())),
        }
    }
}
