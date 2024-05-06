use config::Config;
use getset::Getters;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    client::client_setting::ClientSetting, error::RetrieverError,
    explorer::explorer_setting::ExplorerSetting,
};

#[derive(Debug, Serialize, Deserialize, Getters, Default)]
#[get = "pub with_prefix"]
pub struct RetrieverSetting {
    bitcoincore_rpc_url: Option<String>,
    bitcoincore_rpc_port: Option<String>,
    bitcoincore_rpc_cookie_path: String,
    bitcoincore_rpc_timeout_seconds: Option<u64>,
    mnemonic: Option<String>,
    passphrase: Option<String>,
    base_derivation_paths: Option<Vec<String>>,
    exploration_path: Option<String>,
    sweep: Option<bool>,
    exploration_depth: Option<u32>,
    network: Option<bitcoin::Network>,
    data_dir: String,
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
    pub fn new(
        bitcoincore_rpc_url: Option<String>,
        bitcoincore_rpc_port: Option<String>,
        bitcoincore_rpc_cookie_path: String,
        bitcoincore_rpc_timeout_seconds: Option<u64>,
        mnemonic: Option<String>,
        passphrase: Option<String>,
        base_derivation_paths: Option<Vec<String>>,
        exploration_path: Option<String>,
        sweep: Option<bool>,
        exploration_depth: Option<u32>,
        network: Option<bitcoin::Network>,
        data_dir: String,
    ) -> Self {
        RetrieverSetting {
            bitcoincore_rpc_url,
            bitcoincore_rpc_port,
            bitcoincore_rpc_cookie_path,
            bitcoincore_rpc_timeout_seconds,
            mnemonic,
            passphrase,
            base_derivation_paths,
            exploration_path,
            sweep,
            exploration_depth,
            network,
            data_dir,
        }
    }

    pub fn from_config_file(config_file_path: &str) -> Result<Self, RetrieverError> {
        Ok(Config::builder()
            .add_source(config::File::with_name(&config_file_path))
            .build()?
            .try_deserialize::<RetrieverSetting>()?)
    }

    pub fn get_client_setting(&self) -> ClientSetting {
        ClientSetting {
            rpc_url: self.get_bitcoincore_rpc_url().clone().unwrap(),
            rpc_port: self.get_bitcoincore_rpc_port().clone().unwrap(),
            cookie_path: self.get_bitcoincore_rpc_cookie_path().clone(),
            timeout_seconds: self.get_bitcoincore_rpc_timeout_seconds().unwrap(),
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
