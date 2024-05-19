pub mod client_setting;
pub mod dump_utxout_set_result;

use std::{fs, path::PathBuf, str::FromStr, sync::Arc, time::Duration};

use bitcoincore_rpc::{jsonrpc::serde_json::Value, Auth, RpcApi};
use tracing::{error, info};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    error::RetrieverError,
    path_pairs::{PathScanRequestDescriptorTrio, PathScanResultDescriptorTrio},
};

use self::{client_setting::ClientSetting, dump_utxout_set_result::DumpTxoutSetResult};

#[derive(Debug)]
pub struct BitcoincoreRpcClient {
    client: Arc<bitcoincore_rpc::Client>,
}

impl BitcoincoreRpcClient {
    pub fn new(setting: ClientSetting) -> Result<Self, RetrieverError> {
        info!("Creation of bitcoincore rpc client started.");
        let (user, pass) = Auth::CookieFile(PathBuf::from_str(&setting.get_cookie_path()).unwrap())
            .get_user_pass()?;

        let jsonrpc_build = bitcoincore_rpc::jsonrpc::simple_http::Builder::new()
            .timeout(Duration::from_secs(*setting.get_timeout_seconds()))
            .auth(user.unwrap(), pass)
            .url(format!("{}:{}", setting.get_rpc_url(), setting.get_rpc_port()).as_str())?
            .build();
        let jsonrpc_client = bitcoincore_rpc::jsonrpc::Client::from(jsonrpc_build);
        let client = bitcoincore_rpc::Client::from_jsonrpc(jsonrpc_client);
        info!("Creation of bitcoincore rpc client finished successfully.");
        match client.ping() {
            Ok(_) => {
                info!("Bitcoincore rpc client responded successfully to ping.");
                Ok(BitcoincoreRpcClient {
                    client: Arc::new(client),
                })
            }
            Err(_) => {
                error!("Bitcoincore rpc client did not respond to the ping.");
                Err(RetrieverError::BitcoincoreRpcUnreachable)
            }
        }
    }

    pub fn dump_utxo_set(
        &self,
        data_dump_dir_path: &str,
    ) -> Result<DumpTxoutSetResult, RetrieverError> {
        let dir_path = PathBuf::from_str(data_dump_dir_path).unwrap();
        let mut file_path = dir_path.clone();
        file_path.extend(["utxo_dump.dat"]);
        if file_path.exists() {
            error!("Dump file already exists in datadir.");
            return Err(RetrieverError::DumpFileAlreadyExistsInPath);
        }
        let _ = fs::create_dir_all(&dir_path)?;
        info!("Requesting the utxo dump file from bitcoincore.");
        let response = self.client.call::<DumpTxoutSetResult>(
            "dumptxoutset",
            &[Value::String(file_path.to_str().unwrap().to_string())],
        )?;
        info!("Utxo dump file fetched from bitcoincore successfully.");
        Ok(response)
    }

    pub fn scan_utxo_set(
        &self,
        scan_requests: Vec<PathScanRequestDescriptorTrio>,
    ) -> Result<Vec<PathScanResultDescriptorTrio>, RetrieverError> {
        info!("Scanning the utxo set for details of non-empty ScriptPubKeys.");
        let mut results = vec![];
        for PathScanRequestDescriptorTrio(path, request, descriptor) in scan_requests {
            info!("Scan request sent to bitcoincore.");
            results.push(PathScanResultDescriptorTrio::new(
                path,
                self.client.scan_tx_out_set_blocking(&[request])?,
                descriptor,
            ));
            info!("Scan result received from bitcoincore.");
        }
        info!("Bitcoincore scan for details completed.");
        Ok(results)
    }
}

impl Zeroize for BitcoincoreRpcClient {
    fn zeroize(&mut self) {
        info!("Zeroizing bitcoincore client initialized.");
        let client = bitcoincore_rpc::Client::new(
            "0.0.0.0:0000",
            Auth::CookieFile(PathBuf::from_str("/cookie/jar/obviously").unwrap()),
        )
        .unwrap();
        self.client = Arc::new(client);
        info!("Zeroizing bitcoincore client finished.");
    }
}

impl ZeroizeOnDrop for BitcoincoreRpcClient {}
