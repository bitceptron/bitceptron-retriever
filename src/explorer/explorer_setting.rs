use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerSetting {
    pub(crate) mnemonic: String,
    pub(crate) passphrase: String,
    pub(crate) base_derivation_paths: Vec<String>,
    pub(crate) exploration_path: String,
    pub(crate) exploration_depth: u32,
    pub(crate) network: bitcoin::Network,
    pub(crate) sweep: bool,
}

impl Zeroize for ExplorerSetting {
    fn zeroize(&mut self) {
        self.mnemonic = String::new();
        self.passphrase = String::new();
        self.exploration_path = String::new();
        self.exploration_depth = 0;
        self.network = bitcoin::Network::Regtest;
        self.sweep = false;
    }
}

impl ZeroizeOnDrop for ExplorerSetting {}
