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
