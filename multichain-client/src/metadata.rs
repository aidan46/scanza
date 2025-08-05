//! Chain metadata loader and token registry utilities.
//!
//! This module provides functionality to:
//! - Load token metadata from a local JSON file
//! - Load a list of chain metadata from a JSON file
//!
//! The format for chain metadata matches the format used by chainlist.org:
//! ```json
//! {
//!   "name": "Ethereum Mainnet",
//!   "chainId": 1,
//!   "shortName": "eth",
//!   "networkId": 1,
//!   "nativeCurrency": { "name": "Ether", "symbol": "ETH", "decimals": 18 },
//!   "rpc": ["https://cloudflare-eth.com"]
//! }
//! ```

mod chain;
mod token;

pub use chain::{ChainMetaData, NativeCurrency};
pub use token::{TokenBalance, TokenMetadata};
