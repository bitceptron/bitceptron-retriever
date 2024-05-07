use config::Config;
use getset::Getters;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    client::client_setting::ClientSetting,
    data::{
        defaults::{
            DEFAULT_BITCOINCORE_RPC_PORT, DEFAULT_BITCOINCORE_RPC_TIMEOUT_SECONDS,
            DEFAULT_BITCOINCORE_RPC_URL, DEFAULT_EXPLORATION_DEPTH, DEFAULT_EXPLORATION_PATH,
            DEFAULT_NETWORK, DEFAULT_SWEEP,
        },
        wallets_info::WalletsInfo,
    },
    error::RetrieverError,
    explorer::explorer_setting::ExplorerSetting,
};

#[derive(Debug, Serialize, Deserialize, Getters, Default)]
#[get = "pub with_prefix"]
pub struct RetrieverSetting {
    bitcoincore_rpc_url: Option<String>,
    bitcoincore_rpc_port: Option<String>,
    // Must be entered.
    bitcoincore_rpc_cookie_path: String,
    bitcoincore_rpc_timeout_seconds: Option<u64>,
    // Must be entered.
    mnemonic: String,
    // Must be entered.
    passphrase: String,
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
        // Must be entered.
        bitcoincore_rpc_cookie_path: String,
        bitcoincore_rpc_timeout_seconds: Option<u64>,
        // Must be entered.
        mnemonic: String,
        // Must be entered.
        passphrase: String,
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
        let rpc_url = match self.get_bitcoincore_rpc_url() {
            Some(rpc_url) => rpc_url,
            None => DEFAULT_BITCOINCORE_RPC_URL,
        };
        let rpc_port = match self.get_bitcoincore_rpc_port() {
            Some(rpc_port) => rpc_port,
            None => DEFAULT_BITCOINCORE_RPC_PORT,
        };
        let cookie_path = self.get_bitcoincore_rpc_cookie_path();
        let timeout_seconds = match self.get_bitcoincore_rpc_timeout_seconds() {
            Some(timeout_seconds) => *timeout_seconds,
            None => DEFAULT_BITCOINCORE_RPC_TIMEOUT_SECONDS,
        };
        ClientSetting::new(rpc_url, rpc_port, cookie_path, timeout_seconds)
    }

    pub fn get_explorer_setting(&self) -> ExplorerSetting {
        let mnemonic = self.get_mnemonic().to_owned();
        let passphrase = self.get_passphrase().to_owned();
        let base_derivation_paths = match self.get_base_derivation_paths() {
            Some(base_derivation_paths) => base_derivation_paths.to_owned(),
            None => WalletsInfo::get_all_unique_preset_wallet_base_paths().to_owned(),
        };

        let exploration_path = match self.get_exploration_path() {
            Some(exploration_path) => exploration_path.to_owned(),
            None => DEFAULT_EXPLORATION_PATH.to_string(),
        };

        let exploration_depth = match self.get_exploration_depth() {
            Some(exploration_depth) => *exploration_depth,
            None => DEFAULT_EXPLORATION_DEPTH,
        };

        let network = match self.get_network() {
            Some(network) => *network,
            None => DEFAULT_NETWORK,
        };
        let sweep = match self.get_sweep() {
            Some(sweep) => *sweep,
            None => DEFAULT_SWEEP,
        };
        ExplorerSetting::new(
            mnemonic,
            passphrase,
            base_derivation_paths,
            exploration_path,
            exploration_depth,
            network,
            sweep,
        )
    }
}
