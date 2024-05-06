pub mod client_setting;
pub mod dump_utxout_set_result;

use std::{
    fs,
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use bitcoincore_rpc::{jsonrpc::serde_json::Value, Auth, RpcApi};
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
        let (user, pass) =
            Auth::CookieFile(PathBuf::from_str(&setting.cookie_path).unwrap()).get_user_pass()?;

        let jsonrpc_build = bitcoincore_rpc::jsonrpc::simple_http::Builder::new()
            .timeout(Duration::from_secs(setting.timeout_seconds))
            .auth(user.unwrap(), pass)
            .url(format!("{}:{}", setting.rpc_url, setting.rpc_port).as_str())?
            .build();
        let jsonrpc_client = bitcoincore_rpc::jsonrpc::Client::from(jsonrpc_build);
        let client = bitcoincore_rpc::Client::from_jsonrpc(jsonrpc_client);
        match client.ping() {
            Ok(_) => Ok(BitcoincoreRpcClient {
                client: Arc::new(client),
            }),
            Err(_) => Err(RetrieverError::BitcoincoreRpcUnreachable),
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
            return Err(RetrieverError::DumpFileAlreadyExistsInPath);
        }
        let _ = fs::create_dir_all(&dir_path)?;
        println!("Dumping the utxout set snapshot...");
        let start = Instant::now();
        let response = self.client.call::<DumpTxoutSetResult>(
            "dumptxoutset",
            &[Value::String(file_path.to_str().unwrap().to_string())],
        )?;
        let duration = start.elapsed().as_secs();
        println!(
            "Finished dumping the utxout set snapshot in ~{} mins.",
            duration / 60
        );
        Ok(response)
    }

    pub fn scan_utxo_set(
        &self,
        scan_requests: Vec<PathScanRequestDescriptorTrio>,
    ) -> Result<Vec<PathScanResultDescriptorTrio>, RetrieverError> {
        println!("Scanning the UTXO set for details of found bitcoins...");
        let mut results = vec![];
        for PathScanRequestDescriptorTrio(path, request, descriptor) in scan_requests {
            results.push(PathScanResultDescriptorTrio::new(
                path,
                self.client.scan_tx_out_set_blocking(&[request])?,
                descriptor,
            ));
        }
        Ok(results)
    }
}

impl Zeroize for BitcoincoreRpcClient {
    fn zeroize(&mut self) {
        let client = bitcoincore_rpc::Client::new(
            "0.0.0.0:0000",
            Auth::CookieFile(PathBuf::from_str("/cookie/jar/obviously").unwrap()),
        )
        .unwrap();
        self.client = Arc::new(client)
    }
}

impl ZeroizeOnDrop for BitcoincoreRpcClient {}
