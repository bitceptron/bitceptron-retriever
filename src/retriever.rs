use std::{
    fs,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use bitcoin::{bip32::DerivationPath, key::Secp256k1};
use getset::Getters;
use itertools::Itertools;
use miniscript::Descriptor;
use num_format::{Locale, ToFormattedString};
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    client::BitcoincoreRpcClient,
    covered_descriptors::CoveredDescriptors,
    data::defaults::DEFAULT_SELECTED_DESCRIPTORS,
    error::RetrieverError,
    explorer::Explorer,
    path_pairs::{PathDescriptorPair, PathScanResultDescriptorTrio},
    setting::RetrieverSetting,
    uspk_set::{UnspentScriptPubKeysSet, UspkSetStatus},
};

#[derive(Debug, Clone, Default, Getters)]
#[get = "pub"]
pub struct Retriever {
    client: BitcoincoreRpcClient,
    explorer: Arc<Explorer>,
    uspk_set: UnspentScriptPubKeysSet,
    data_dir: String,
    finds: Arc<Mutex<Vec<PathDescriptorPair>>>,
    detailed_finds: Option<Vec<PathScanResultDescriptorTrio>>,
    select_descriptors: hashbrown::HashSet<CoveredDescriptors>,
}

impl Retriever {
    pub async fn new(setting: RetrieverSetting) -> Result<Self, RetrieverError> {
        info!("Creation of retriever started.");
        let client_setting = setting.get_client_setting();
        let explorer_setting = setting.get_explorer_setting();
        let client = BitcoincoreRpcClient::new(client_setting).await?;
        let explorer = Arc::new(Explorer::new(explorer_setting)?);
        let uspk_set = UnspentScriptPubKeysSet::new();
        let data_dir = fs::canonicalize(setting.get_data_dir())?
            .to_string_lossy()
            .to_string();
        let finds = Arc::new(Mutex::new(vec![]));
        let select_descriptors = match setting.get_selected_descriptors() {
            Some(select_descriptors) => hashbrown::HashSet::from_iter(select_descriptors.clone()),
            None => hashbrown::HashSet::from_iter(DEFAULT_SELECTED_DESCRIPTORS.to_vec()),
        };
        info!("Creation of retriever finished successfully.");
        Ok(Retriever {
            client,
            explorer,
            uspk_set,
            data_dir,
            finds,
            detailed_finds: None,
            select_descriptors,
        })
    }

    pub async fn check_for_dump_in_data_dir_or_create_dump_file(
        &mut self,
    ) -> Result<(), RetrieverError> {
        let data_dir_path = PathBuf::from_str(&self.data_dir).unwrap();
        let mut dump_file_path = data_dir_path.clone();
        dump_file_path.extend(["utxo_dump.dat"]);
        info!("Searching for the dump file in datadir.");
        if dump_file_path.exists() {
            info!("Dump file found in datadir.");
            Ok(())
        } else {
            info!("Dump file was not found in datadir.");
            if !data_dir_path.exists() {
                info!("Creating the full datadir path.");
                fs::create_dir_all(data_dir_path)?;
            }
            let _dump_result = self.client.dump_utxo_set(&self.data_dir).await?;
            Ok(())
        }
    }

    pub async fn populate_uspk_set(&mut self) -> Result<(), RetrieverError> {
        if self.uspk_set.get_status() == UspkSetStatus::Empty {
            info!("Searching for the dump file to populate the Unspent ScriptPubKey set.");
            let dump_file_path_str = format!("{}/utxo_dump.dat", self.data_dir);
            let dump_file_path = PathBuf::from_str(&dump_file_path_str).unwrap();
            if !dump_file_path.exists() {
                error!("Dump file (utxo_dump.dat) does not exist in data dir.");
                return Err(RetrieverError::NoDumpFileInDataDir);
            }
            info!("Dump file found.");
            let _ = tokio::join!({ self.uspk_set.populate_with_dump_file(&dump_file_path_str) });
            Ok(())
        } else if self.uspk_set.get_status() == UspkSetStatus::Populating {
            Err(RetrieverError::PopulatingUSPKSetInProgress)
        } else {
            Err(RetrieverError::USPKSetAlreadyPopulated)
        }
    }

    pub async fn create_derivation_path_stream(
        &self,
        sender: mpsc::Sender<DerivationPath>,
    ) -> Result<(), RetrieverError> {
        let explorer = self.explorer.clone();
        let bases = explorer.get_exploration_path().get_base_paths().to_owned();
        let num_explore_paths = self.explorer.get_exploration_path().size();
        let total_paths = num_explore_paths;
        let mut sent_paths = 0;
        tokio::spawn(async move {
            info!(
                "Creation of an iterator for total {} paths started.",
                total_paths.to_formatted_string(&Locale::en)
            );
            let explore_paths_iter = explorer
                .get_exploration_path()
                .clone()
                .get_explore()
                .to_owned()
                .iter()
                .map(|step| step.to_owned())
                .multi_cartesian_product();
            for explore_path in explore_paths_iter {
                for base in bases.iter() {
                    sender
                        .send(
                            base.extend(
                                DerivationPath::from_str(&format!("m/{}", explore_path.join("/")))
                                    .unwrap(),
                            ),
                        )
                        .await
                        .unwrap();
                    sent_paths += 1;
                    if sent_paths % 1000 == 0 {
                        info!(
                            "Total paths sent to processing: {} of {}",
                            sent_paths.to_formatted_string(&Locale::en),
                            total_paths.to_formatted_string(&Locale::en)
                        )
                    }
                }
            }
        });
        Ok(())
    }

    pub async fn process_derivation_path_stream(
        &mut self,
        receiver: &mut mpsc::Receiver<DerivationPath>,
    ) -> Result<(), RetrieverError> {
        let secp = Secp256k1::new();
        let select_descriptors = self.select_descriptors.clone();
        let uspk_set = self.uspk_set.get_immutable_inner_set();
        let mut paths_received = 0;
        while let Some(path) = receiver.recv().await {
            paths_received += 1;
            if paths_received % 1000 == 0 {
                info!(
                    "Total paths received to process: {}",
                    paths_received.to_formatted_string(&Locale::en)
                );
            }
            let pubkey = self
                .explorer
                .get_master_xpriv()
                .derive_priv(&secp, &path)
                .unwrap()
                .to_keypair(&secp)
                .public_key();
            if select_descriptors.contains(&CoveredDescriptors::P2pk) {
                let desc = Descriptor::new_pk(pubkey);
                let desc_pubkey = desc.script_pubkey();
                let target = desc_pubkey.as_bytes();
                if uspk_set.contains(target) {
                    warn!("Found a UTXO match for ScriptPubKey.");
                    self.finds
                        .lock()
                        .unwrap()
                        .push(PathDescriptorPair::new(path.to_owned(), desc));
                }
            }
            if select_descriptors.contains(&CoveredDescriptors::P2pkh) {
                let desc = Descriptor::new_pkh(pubkey)
                    .map_err(RetrieverError::from)
                    .unwrap();
                let desc_pubkey = desc.script_pubkey();
                let target = desc_pubkey.as_bytes();
                if uspk_set.contains(target) {
                    warn!("Found a UTXO match for ScriptPubKey.");
                    self.finds
                        .lock()
                        .unwrap()
                        .push(PathDescriptorPair::new(path.to_owned(), desc));
                }
            }
            if select_descriptors.contains(&CoveredDescriptors::P2wpkh) {
                let desc = Descriptor::new_wpkh(pubkey)
                    .map_err(RetrieverError::from)
                    .unwrap();
                let desc_pubkey = desc.script_pubkey();
                let target = desc_pubkey.as_bytes();
                if uspk_set.contains(target) {
                    warn!("Found a UTXO match for ScriptPubKey.");
                    self.finds
                        .lock()
                        .unwrap()
                        .push(PathDescriptorPair::new(path.to_owned(), desc));
                }
            }
            if select_descriptors.contains(&CoveredDescriptors::P2shwpkh) {
                let desc = Descriptor::new_sh_wpkh(pubkey)
                    .map_err(RetrieverError::from)
                    .unwrap();
                let desc_pubkey = desc.script_pubkey();
                let target = desc_pubkey.as_bytes();
                if uspk_set.contains(target) {
                    warn!("Found a UTXO match for ScriptPubKey.");
                    self.finds
                        .lock()
                        .unwrap()
                        .push(PathDescriptorPair::new(path.to_owned(), desc));
                }
            }
            if select_descriptors.contains(&CoveredDescriptors::P2tr) {
                let desc = Descriptor::new_tr(pubkey, None)
                    .map_err(RetrieverError::from)
                    .unwrap();
                let desc_pubkey = desc.script_pubkey();
                let target = desc_pubkey.as_bytes();
                if uspk_set.contains(target) {
                    warn!("Found a UTXO match for ScriptPubKey.");
                    self.finds
                        .lock()
                        .unwrap()
                        .push(PathDescriptorPair::new(path.to_owned(), desc));
                }
            }
        }
        Ok(())
    }

    pub async fn search_the_uspk_set(&mut self) -> Result<(), RetrieverError> {
        let (tx, mut rx) = mpsc::channel(1024);
        let _ = tokio::join!(self.create_derivation_path_stream(tx));
        let _ = tokio::join!(self.process_derivation_path_stream(&mut rx));
        Ok(())
    }

    pub async fn get_details_of_finds_from_bitcoincore(&mut self) -> Result<(), RetrieverError> {
        // if self.finds.lock().unwrap().is_empty() {
        //     return Err(RetrieverError::NoSearchHasBeenPerformed);
        // } else
        if self.finds.lock().unwrap().is_empty() {
            println!("No UTXO match were found in the explored paths.");
            Ok(())
        } else {
            let path_scan_request_pairs = self
                .finds
                .lock()
                .unwrap()
                .iter()
                .map(|item| item.to_path_scan_request_descriptor_trio())
                .collect();
            self.detailed_finds = Some(self.client.scan_utxo_set(path_scan_request_pairs).await?);
            Ok(())
        }
    }

    pub fn print_detailed_finds_on_console(&self) -> Result<(), RetrieverError> {
        if self.detailed_finds.is_none() {
            return Err(RetrieverError::DetailsHaveNotBeenFetched);
        };
        for (index, detail) in self.detailed_finds.as_ref().unwrap().iter().enumerate() {
            let info = format!(
                "\nResult {}\nPath: {}\nAmount(satoshis): {}\nDescriptor: {}",
                index + 1,
                detail.0,
                detail
                    .1
                    .total_amount
                    .to_sat()
                    .to_formatted_string(&Locale::en),
                detail.2
            );
            println!("{info}");
        }
        Ok(())
    }

    pub fn get_detailed_finds(&self) -> Result<Vec<PathScanResultDescriptorTrio>, RetrieverError> {
        if self.detailed_finds.is_none() {
            Err(RetrieverError::DetailsHaveNotBeenFetched)
        } else {
            Ok(self.detailed_finds.as_ref().unwrap().to_owned())
        }
    }
}

impl Zeroize for Retriever {
    fn zeroize(&mut self) {
        self.client.zeroize();
        // self.explorer.as_ref().zeroize();
        self.data_dir.zeroize();
    }
}

impl ZeroizeOnDrop for Retriever {}
