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

#[derive(Debug, Clone)]
pub struct BitcoincoreRpcClient {
    client: Arc<bitcoincore_rpc::Client>,
}

impl Default for BitcoincoreRpcClient {
    fn default() -> Self {
        Self { client: Arc::new(bitcoincore_rpc::Client::new("0.0.0.0", Auth::None).unwrap()) }
    }
}

impl BitcoincoreRpcClient {
    pub async fn new(setting: ClientSetting) -> Result<Self, RetrieverError> {
        info!("Creation of bitcoincore rpc client started.");
        let (client_result_sender, mut client_result_receiver) =
            tokio::sync::mpsc::unbounded_channel();
        let (user, pass) = Auth::CookieFile(PathBuf::from_str(setting.get_cookie_path()).unwrap())
            .get_user_pass()?;
        tokio::task::spawn_blocking(move || {
            let jsonrpc_build = bitcoincore_rpc::jsonrpc::simple_http::Builder::new()
                .timeout(Duration::from_secs(*setting.get_timeout_seconds()))
                .auth(user.unwrap(), pass)
                .url(format!("{}:{}", setting.get_rpc_url(), setting.get_rpc_port()).as_str())
                .map_err(|err| client_result_sender.send(Err(RetrieverError::from(err))))
                .unwrap()
                .build();
            let jsonrpc_client = bitcoincore_rpc::jsonrpc::Client::from(jsonrpc_build);
            let client = bitcoincore_rpc::Client::from_jsonrpc(jsonrpc_client);
            info!("Creation of bitcoincore rpc client finished successfully.");
            match client.ping() {
                Ok(_) => {
                    info!("Bitcoincore rpc client responded successfully to ping.");
                    let _ = client_result_sender.send(Ok(BitcoincoreRpcClient {
                        client: Arc::new(client),
                    }));
                }
                Err(_) => {
                    error!("Bitcoincore rpc client did not respond to the ping.");
                    let _ =
                        client_result_sender.send(Err(RetrieverError::BitcoincoreRpcUnreachable));
                }
            };
        });

        client_result_receiver.recv().await.unwrap()
    }

    pub async fn dump_utxo_set(
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
        fs::create_dir_all(&dir_path)?;
        let client = self.client.clone();
        let (response_sender, response_receiver) = tokio::sync::oneshot::channel();
        tokio::task::spawn_blocking(move || {
            info!("Requesting the utxo dump file from bitcoincore.");
            let response = client.call::<DumpTxoutSetResult>(
                "dumptxoutset",
                &[Value::String(file_path.to_str().unwrap().to_string())],
            );
            info!("Utxo dump file fetched from bitcoincore successfully.");
            let _ = response_sender.send(response);
        });

        Ok(response_receiver.await.unwrap()?)
    }

    pub async fn scan_utxo_set(
        &self,
        scan_requests: Vec<PathScanRequestDescriptorTrio>,
    ) -> Result<Vec<PathScanResultDescriptorTrio>, RetrieverError> {
        info!("Scanning the utxo set for details of non-empty ScriptPubKeys.");
        let (results_sender, mut results_receiver) = tokio::sync::mpsc::unbounded_channel();
        let client = self.client.clone();
        tokio::task::spawn_blocking(move || {
            let mut results = vec![];
            for PathScanRequestDescriptorTrio(path, request, descriptor) in scan_requests {
                info!("Scan request sent to bitcoincore.");
                results.push(PathScanResultDescriptorTrio::new(
                    path,
                    client
                        .scan_tx_out_set_blocking(&[request])
                        .map_err(|err| results_sender.send(Err(RetrieverError::from(err))))
                        .unwrap(),
                    descriptor,
                ));
                info!("Scan result received from bitcoincore.");
            }
            info!("Bitcoincore scan for details completed.");
            let _ = results_sender.send(Ok(results));
        });

        results_receiver.recv().await.unwrap()
    }
}

impl Zeroize for BitcoincoreRpcClient {
    fn zeroize(&mut self) {
        let client = bitcoincore_rpc::Client::new(
            "0.0.0.0:0000",
            Auth::CookieFile(PathBuf::from_str("/cookie/jar/obviously").unwrap()),
        )
        .unwrap();
        self.client = Arc::new(client);
    }
}

impl ZeroizeOnDrop for BitcoincoreRpcClient {}