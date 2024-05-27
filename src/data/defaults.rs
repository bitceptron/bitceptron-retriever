use crate::covered_descriptors::CoveredDescriptors::{self, P2pk, P2pkh, P2shwpkh, P2tr, P2wpkh};

pub const DEFAULT_EXPLORATION_DEPTH: u32 = 100;
pub const DEFAULT_EXPLORATION_PATH: &str = "*";
pub const DEFAULT_BITCOINCORE_RPC_URL: &str = "127.0.0.1";
pub const DEFAULT_BITCOINCORE_RPC_PORT: &str = "8332";
pub const DEFAULT_BITCOINCORE_RPC_TIMEOUT_SECONDS: u64 = 6800;
pub const DEFAULT_SWEEP: bool = false;
pub const DEFAULT_NETWORK: bitcoin::Network = bitcoin::Network::Bitcoin;
pub const DEFAULT_SELECTED_DESCRIPTORS: [CoveredDescriptors; 5] =
    [P2pk, P2pkh, P2shwpkh, P2tr, P2wpkh];

