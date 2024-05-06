use std::{fs, path::PathBuf, str::FromStr, sync::Arc};

use num_format::{Locale, ToFormattedString};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    client::{dump_utxout_set_result::DumpTxoutSetResult, BitcoincoreRpcClient},
    error::RetrieverError,
    explorer::Explorer,
    path_pairs::{PathDescriptorPair, PathScanResultDescriptorTrio},
    setting::RetrieverSetting,
    uspk_set::UnspentScriptPupKeysSet,
};

#[derive(Debug)]
pub struct Retriever {
    client: BitcoincoreRpcClient,
    explorer: Explorer,
    uspk_set: Option<Arc<UnspentScriptPupKeysSet>>,
    data_dir: String,
    dump_result: Option<DumpTxoutSetResult>,
    finds: Option<Vec<PathDescriptorPair>>,
    detailed_finds: Option<Vec<PathScanResultDescriptorTrio>>,
}

impl Retriever {
    pub fn new(setting: RetrieverSetting) -> Result<Self, RetrieverError> {
        let client_setting = setting.get_client_setting();
        let explorer_setting = setting.get_explorer_setting();
        let client = BitcoincoreRpcClient::new(client_setting)?;
        let explorer = Explorer::new(explorer_setting)?;
        Ok(Retriever {
            client,
            explorer,
            uspk_set: None,
            data_dir: setting.get_data_dir().to_owned(),
            dump_result: None,
            finds: None,
            detailed_finds: None,
        })
    }

    pub fn check_for_dump_in_data_dir_or_create_dump_file(&mut self) -> Result<(), RetrieverError> {
        let data_dir_path = PathBuf::from_str(&self.data_dir).unwrap();
        let mut dump_file_path = data_dir_path.clone();
        dump_file_path.extend(["utxo_dump.dat"]);
        if dump_file_path.exists() {
            Ok(())
        } else {
            fs::create_dir_all(data_dir_path)?;
            let dump_result = self.client.dump_utxo_set(&self.data_dir)?;
            self.dump_result = Some(dump_result);
            Ok(())
        }
    }

    pub fn populate_uspk_set(&mut self) -> Result<(), RetrieverError> {
        let dump_file_path_str = format!("{}/utxo_dump.dat", self.data_dir);
        let dump_file_path = PathBuf::from_str(&dump_file_path_str).unwrap();
        if !dump_file_path.exists() {
            return Err(RetrieverError::NoDumpFileInDataDir);
        }
        self.uspk_set = Some(Arc::new(UnspentScriptPupKeysSet::from_dump_file(
            &dump_file_path_str,
        )?));
        Ok(())
    }

    pub fn search_the_uspk_set(&mut self) -> Result<(), RetrieverError> {
        if self.uspk_set.is_none() {
            return Err(RetrieverError::UnspentScriptPublicKeySetIsNotPopulated);
        }
        let mut path_descriptor_pairs_vec =
            self.explorer.get_all_single_key_script_descriptors()?;
        let finds = self
            .uspk_set
            .as_ref()
            .unwrap()
            .search_for_path_descriptor_pairs_and_return_those_present(
                &mut path_descriptor_pairs_vec,
            );

        self.finds = Some(finds);
        Ok(())
    }

    pub fn get_details_of_finds_from_bitcoincore(&mut self) -> Result<(), RetrieverError> {
        if self.finds.is_none() {
            return Err(RetrieverError::NoSearchHasBeenPerformed);
        }
        let path_scan_request_pairs = self
            .finds
            .as_ref()
            .unwrap()
            .iter()
            .map(|item| item.to_path_scan_request_descriptor_trio())
            .collect();
        self.detailed_finds = Some(self.client.scan_utxo_set(path_scan_request_pairs)?);
        Ok(())
    }

    pub fn print_detailed_finds_on_console(&self) -> Result<(), RetrieverError> {
        if self.detailed_finds.is_none() {
            return Err(RetrieverError::DetailsHaveNotBeenFetched);
        };
        for (index, detail) in self.detailed_finds.as_ref().unwrap().iter().enumerate() {
            let info = format!(
                "\nResult {}\nPath: {}\nAmount(satoshis): {}\nDescriptor: {}",
                index + 1,
                detail.0.to_string(),
                detail
                    .1
                    .total_amount
                    .to_sat()
                    .to_formatted_string(&Locale::en),
                detail.2.to_string()
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
        self.explorer.zeroize();
        self.finds.zeroize();
        self.data_dir.zeroize();
        self.dump_result.zeroize();
    }
}

impl ZeroizeOnDrop for Retriever {}
