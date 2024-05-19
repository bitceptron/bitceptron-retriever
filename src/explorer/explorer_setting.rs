use getset::Getters;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Getters)]
#[get = "pub with_prefix"]
pub struct ExplorerSetting {
    mnemonic: String,
    passphrase: String,
    base_derivation_paths: Vec<String>,
    exploration_path: String,
    exploration_depth: u32,
    network: bitcoin::Network,
    sweep: bool,
}

impl ExplorerSetting {
    pub fn new(
        mnemonic: String,
        passphrase: String,
        base_derivation_paths: Vec<String>,
        exploration_path: String,
        exploration_depth: u32,
        network: bitcoin::Network,
        sweep: bool,
    ) -> Self {
        ExplorerSetting {
            mnemonic,
            passphrase,
            base_derivation_paths,
            exploration_path,
            exploration_depth,
            network,
            sweep,
        }
    }
}

impl Zeroize for ExplorerSetting {
    fn zeroize(&mut self) {
        self.mnemonic.zeroize();
        self.passphrase.zeroize();
        self.base_derivation_paths.zeroize();
        self.exploration_path.zeroize();
        self.exploration_depth.zeroize();
        self.network = bitcoin::Network::Regtest;
        self.sweep.zeroize();
    }
}

impl ZeroizeOnDrop for ExplorerSetting {}
