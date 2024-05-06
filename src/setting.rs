use getset::Getters;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{client::client_setting::ClientSetting, explorer::explorer_setting::ExplorerSetting};

#[derive(Debug, Serialize, Deserialize, Getters, Default)]
#[get = "pub with_prefix"]
pub struct RetrieverSetting {
    pub bitcoincore_rpc_url: Option<String>,
    pub bitcoincore_rpc_port: Option<String>,
    pub bitcoincore_rpc_cookie_path: String,
    pub bitcoincore_rpc_timeout_seconds: Option<u64>,
    pub mnemonic: Option<String>,
    pub passphrase: Option<String>,
    pub base_derivation_paths: Option<Vec<String>>,
    pub exploration_path: Option<String>,
    pub sweep: Option<bool>,
    pub exploration_depth: Option<u32>,
    pub network: Option<bitcoin::Network>,
    pub data_dir: String,
}

impl Zeroize for RetrieverSetting {
    fn zeroize(&mut self) {
        self.bitcoincore_rpc_url.zeroize();
        self.bitcoincore_rpc_port.zeroize();
        self.bitcoincore_rpc_cookie_path.zeroize();
        self.bitcoincore_rpc_timeout_seconds.zeroize();
        self.mnemonic.zeroize();
        self.passphrase.zeroize();
        self.base_derivation_paths.zeroize();
        self.exploration_path.zeroize();
        self.sweep.zeroize();
        self.exploration_depth.zeroize();
        self.network = Some(bitcoin::Network::Signet);
    }
}

impl ZeroizeOnDrop for RetrieverSetting {}

impl RetrieverSetting {
    pub fn get_client_setting(&self) -> ClientSetting {
        ClientSetting {
            rpc_url: self.bitcoincore_rpc_url.clone().unwrap(),
            rpc_port: self.bitcoincore_rpc_port.clone().unwrap(),
            cookie_path: self.bitcoincore_rpc_cookie_path.clone(),
            timeout_seconds: self.bitcoincore_rpc_timeout_seconds.unwrap(),
        }
    }

    pub fn get_explorer_setting(&self) -> ExplorerSetting {
        ExplorerSetting {
            mnemonic: self.mnemonic.clone().unwrap(),
            passphrase: self.passphrase.clone().unwrap(),
            base_derivation_paths: self.base_derivation_paths.clone().unwrap(),
            exploration_path: self.exploration_path.clone().unwrap(),
            exploration_depth: self.exploration_depth.unwrap(),
            network: self.network.unwrap(),
            sweep: self.sweep.unwrap(),
        }
    }
}
