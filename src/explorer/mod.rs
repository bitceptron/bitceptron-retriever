pub mod auxiliaries;
pub mod exploration_path;
pub mod explorer_setting;

use std::{str::FromStr, time::Instant};

use bitcoin::{
    bip32::{DerivationPath, Xpriv, Xpub},
    key::Secp256k1,
};
use indicatif::ProgressStyle;
use miniscript::Descriptor;

use num_format::{Locale, ToFormattedString};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    data::defaults::NUMBER_OF_DESCRIPTOR_CLASSES,
    error::RetrieverError,
    explorer::auxiliaries::{
        from_input_str_to_mnemonic, from_mnemonic_to_seed, from_seed_to_master_xpriv,
    },
    path_pairs::{PathDescriptorPair, PathXpubPair},
};

use self::{exploration_path::ExplorationPath, explorer_setting::ExplorerSetting};

/// a data structure to capture the set of self-sufficient data for scanning certain paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Explorer {
    path_xpubs: Vec<PathXpubPair>,
    num_scans: u64,
}

impl Zeroize for Explorer {
    fn zeroize(&mut self) {
        let master_xpriv = Xpriv::new_master(bitcoin::Network::Regtest, &[0u8; 64]).unwrap();
        let num_xpubs = self.path_xpubs.len();
        self.path_xpubs = vec![
            PathXpubPair::new(
                DerivationPath::from_str("m/0/0/0/0/0/0/0/0/0/0").unwrap(),
                Xpub::from_priv(&Secp256k1::new(), &master_xpriv),
            );
            num_xpubs
        ];
        self.num_scans.zeroize();
    }
}

impl ZeroizeOnDrop for Explorer {}

impl Explorer {
    #[allow(unused_assignments)]
    pub fn new(setting: ExplorerSetting) -> Result<Self, RetrieverError> {
        let exploration_path =
            ExplorationPath::new(&setting.exploration_path, setting.exploration_depth)?;
        let num_scans = NUMBER_OF_DESCRIPTOR_CLASSES
            * setting.base_derivation_paths.len() as u64
            * if setting.sweep {
                exploration_path.num_of_paths_sweep_from_root()
            } else {
                exploration_path.num_of_paths()
            };
        let mut mnemonic = from_input_str_to_mnemonic(&setting.mnemonic)?;
        let mut seed = from_mnemonic_to_seed(mnemonic.clone(), &setting.passphrase);
        mnemonic.zeroize();
        let mut master_xpriv = from_seed_to_master_xpriv(seed.clone(), setting.network)?;
        seed.zeroize();
        let base_derivation_paths = {
            let mut paths = vec![];
            for path_string in setting.base_derivation_paths.clone() {
                paths.push(DerivationPath::from_str(&path_string)?)
            }
            paths
        };
        let mut complete_paths = vec![];
        let exploration_paths = if setting.sweep {
            exploration_path.generate_derivation_paths_for_exploration_path_sweep()?
        } else {
            exploration_path.generate_derivation_paths_for_exploration_path()?
        };
        for base_path in base_derivation_paths.iter() {
            for path in &exploration_paths {
                complete_paths.push(base_path.extend(path))
            }
        }
        let secp = Secp256k1::new();
        let mut path_xpubs = vec![];
        // Loop info.
        let creation_start = Instant::now();
        let step_size = 1000u64;
        let mut average_step_time_in_milis = 0u128;
        let total_loops = complete_paths.len() as u64;
        let mut loops_done = 0u64;
        let mut steps_done = 0u128;
        let mut steps_remaining = (total_loops / step_size) as u128;
        let pb =
            indicatif::ProgressBar::new(total_loops as u64).with_prefix("Creating the xpubs: ");
        pb.set_style(
            ProgressStyle::with_template(&format!("{{prefix}}▕{{bar:.{}}}▏ {{msg}}", "╢▌▌░╟"))
                .unwrap(),
        );
        let mut step_start_time = Instant::now();
        // Loop.
        for path in complete_paths {
            path_xpubs.push(PathXpubPair::new(
                path.clone(),
                Xpub::from_priv(&secp, &master_xpriv.derive_priv(&secp, &path)?),
            ));
            // Loop info.
            loops_done += 1;
            if loops_done % step_size == 0 {
                steps_done += 1;
                steps_remaining -= 1;
                average_step_time_in_milis = (step_start_time.elapsed().as_millis()
                    + (steps_done - 1) * average_step_time_in_milis)
                    / steps_done as u128;
                let remaining_time_in_milis = average_step_time_in_milis * steps_remaining;
                pb.inc(step_size);
                pb.clone().with_message(format!(
                    "{} / {}\nEstimated time to completion: ~{} minutes.",
                    loops_done.to_formatted_string(&Locale::en),
                    total_loops.to_formatted_string(&Locale::en),
                    1 + remaining_time_in_milis / 60_000,
                ));
                step_start_time = Instant::now();
            }
        }
        pb.finish_with_message(format!(
            "Created the explorer with {} xpubs in ~{} mins.",
            total_loops.to_formatted_string(&Locale::en),
            1 + creation_start.elapsed().as_secs() / 60
        ));

        master_xpriv = from_seed_to_master_xpriv([0u8; 64], bitcoin::Network::Bitcoin)?;
        Ok(Explorer {
            path_xpubs,
            num_scans,
        })
    }

    pub fn get_all_p2pk_descriptors(&self) -> Result<Vec<PathDescriptorPair>, RetrieverError> {
        let creation_start = Instant::now();
        let mut p2pk_descriptors = vec![];
        // Loop info.
        let step_size = 100_000u64;
        let mut average_step_time_in_milis = 0u128;
        let total_loops = self.path_xpubs.len() as u64;
        let mut loops_done = 0u64;
        let mut steps_done = 0u128;
        let mut steps_remaining = (total_loops / step_size) as u128;
        let pb = indicatif::ProgressBar::new(total_loops as u64)
            .with_prefix("Creating P2PK descriptors: ");
        pb.set_style(
            ProgressStyle::with_template(&format!("{{prefix}}▕{{bar:.{}}}▏ {{msg}}", "╢▌▌░╟"))
                .unwrap(),
        );
        let mut step_start_time = Instant::now();
        for PathXpubPair(path, xpub) in self.path_xpubs.iter() {
            p2pk_descriptors.push(PathDescriptorPair::new(
                path.to_owned(),
                Descriptor::new_pk(bitcoin::secp256k1::PublicKey::from_slice(
                    &xpub.to_pub().to_bytes(),
                )?),
            ));
            // Loop info stuff.
            loops_done += 1;
            if loops_done % step_size == 0 {
                steps_done += 1;
                steps_remaining -= 1;
                average_step_time_in_milis = (step_start_time.elapsed().as_millis()
                    + (steps_done - 1) * average_step_time_in_milis)
                    / steps_done as u128;
                let remaining_time_in_milis = average_step_time_in_milis * steps_remaining;
                pb.inc(step_size);
                pb.clone().with_message(format!(
                    "{} / {}\nEstimated time to completion: ~{} minutes.",
                    loops_done.to_formatted_string(&Locale::en),
                    total_loops.to_formatted_string(&Locale::en),
                    1 + remaining_time_in_milis / 60_000,
                ));
                step_start_time = Instant::now();
            }
        }
        pb.finish_with_message(format!(
            "{} P2PK descriptors created in ~{} mins.",
            total_loops.to_formatted_string(&Locale::en),
            1 + creation_start.elapsed().as_secs() / 60
        ));
        Ok(p2pk_descriptors)
    }

    pub fn get_all_p2pkh_descriptors(&self) -> Result<Vec<PathDescriptorPair>, RetrieverError> {
        let creation_start = Instant::now();
        let mut p2pkh_descriptors = vec![];
        // Loop info.
        let step_size = 100_000u64;
        let mut average_step_time_in_milis = 0u128;
        let total_loops = self.path_xpubs.len() as u64;
        let mut loops_done = 0u64;
        let mut steps_done = 0u128;
        let mut steps_remaining = (total_loops / step_size) as u128;
        let pb = indicatif::ProgressBar::new(total_loops as u64)
            .with_prefix("Creating P2PKH descriptors: ");
        pb.set_style(
            ProgressStyle::with_template(&format!("{{prefix}}▕{{bar:.{}}}▏ {{msg}}", "╢▌▌░╟"))
                .unwrap(),
        );
        let mut step_start_time = Instant::now();
        for PathXpubPair(path, xpub) in &self.path_xpubs {
            p2pkh_descriptors.push(PathDescriptorPair::new(
                path.to_owned(),
                Descriptor::new_pkh(bitcoin::secp256k1::PublicKey::from_slice(
                    &xpub.to_pub().to_bytes(),
                )?)?,
            ));
            // Loop info stuff.
            loops_done += 1;
            if loops_done % step_size == 0 {
                steps_done += 1;
                steps_remaining -= 1;
                average_step_time_in_milis = (step_start_time.elapsed().as_millis()
                    + (steps_done - 1) * average_step_time_in_milis)
                    / steps_done as u128;
                let remaining_time_in_milis = average_step_time_in_milis * steps_remaining;
                pb.inc(step_size);
                pb.clone().with_message(format!(
                    "{} / {}\nEstimated time to completion: ~{} minutes.",
                    loops_done.to_formatted_string(&Locale::en),
                    total_loops.to_formatted_string(&Locale::en),
                    1 + remaining_time_in_milis / 60_000,
                ));
                step_start_time = Instant::now();
            }
        }
        pb.finish_with_message(format!(
            "{} P2PKH descriptors created in ~{} mins.",
            total_loops.to_formatted_string(&Locale::en),
            1 + creation_start.elapsed().as_secs() / 60
        ));
        Ok(p2pkh_descriptors)
    }

    pub fn get_all_p2wpkh_descriptors(&self) -> Result<Vec<PathDescriptorPair>, RetrieverError> {
        let creation_start = Instant::now();
        let mut p2wpkh_descriptors = vec![];
        // Loop info.
        let step_size = 100_000u64;
        let mut average_step_time_in_milis = 0u128;
        let total_loops = self.path_xpubs.len() as u64;
        let mut loops_done = 0u64;
        let mut steps_done = 0u128;
        let mut steps_remaining = (total_loops / step_size) as u128;
        let pb = indicatif::ProgressBar::new(total_loops as u64)
            .with_prefix("Creating P2WPKH descriptors: ");
        pb.set_style(
            ProgressStyle::with_template(&format!("{{prefix}}▕{{bar:.{}}}▏ {{msg}}", "╢▌▌░╟"))
                .unwrap(),
        );
        let mut step_start_time = Instant::now();
        for PathXpubPair(path, xpub) in &self.path_xpubs {
            p2wpkh_descriptors.push(PathDescriptorPair::new(
                path.to_owned(),
                Descriptor::new_wpkh(bitcoin::secp256k1::PublicKey::from_slice(
                    &xpub.to_pub().to_bytes(),
                )?)?,
            ));
            // Loop info stuff.
            loops_done += 1;
            if loops_done % step_size == 0 {
                steps_done += 1;
                steps_remaining -= 1;
                average_step_time_in_milis = (step_start_time.elapsed().as_millis()
                    + (steps_done - 1) * average_step_time_in_milis)
                    / steps_done as u128;
                let remaining_time_in_milis = average_step_time_in_milis * steps_remaining;
                pb.inc(step_size);
                pb.clone().with_message(format!(
                    "{} / {}\nEstimated time to completion: ~{} minutes.",
                    loops_done.to_formatted_string(&Locale::en),
                    total_loops.to_formatted_string(&Locale::en),
                    1 + remaining_time_in_milis / 60_000,
                ));
                step_start_time = Instant::now();
            }
        }
        pb.finish_with_message(format!(
            "{} P2WPKH descriptors created in ~{} mins.",
            total_loops.to_formatted_string(&Locale::en),
            1 + creation_start.elapsed().as_secs() / 60
        ));
        Ok(p2wpkh_descriptors)
    }

    pub fn get_all_p2shwpkh_descriptors(&self) -> Result<Vec<PathDescriptorPair>, RetrieverError> {
        let creation_start = Instant::now();
        let mut p2shwpkh_descriptors = vec![];
        // Loop info.
        let step_size = 100_000u64;
        let mut average_step_time_in_milis = 0u128;
        let total_loops = self.path_xpubs.len() as u64;
        let mut loops_done = 0u64;
        let mut steps_done = 0u128;
        let mut steps_remaining = (total_loops / step_size) as u128;
        let pb = indicatif::ProgressBar::new(total_loops as u64)
            .with_prefix("Creating P2SHWPKH descriptors: ");
        pb.set_style(
            ProgressStyle::with_template(&format!("{{prefix}}▕{{bar:.{}}}▏ {{msg}}", "╢▌▌░╟"))
                .unwrap(),
        );
        let mut step_start_time = Instant::now();
        for PathXpubPair(path, xpub) in &self.path_xpubs {
            p2shwpkh_descriptors.push(PathDescriptorPair::new(
                path.to_owned(),
                Descriptor::new_sh_wpkh(bitcoin::secp256k1::PublicKey::from_slice(
                    &xpub.to_pub().to_bytes(),
                )?)?,
            ));
            // Loop info stuff.
            loops_done += 1;
            if loops_done % step_size == 0 {
                steps_done += 1;
                steps_remaining -= 1;
                average_step_time_in_milis = (step_start_time.elapsed().as_millis()
                    + (steps_done - 1) * average_step_time_in_milis)
                    / steps_done as u128;
                let remaining_time_in_milis = average_step_time_in_milis * steps_remaining;
                pb.inc(step_size);
                pb.clone().with_message(format!(
                    "{} / {}\nEstimated time to completion: ~{} minutes.",
                    loops_done.to_formatted_string(&Locale::en),
                    total_loops.to_formatted_string(&Locale::en),
                    1 + remaining_time_in_milis / 60_000,
                ));
                step_start_time = Instant::now();
            }
        }
        pb.finish_with_message(format!(
            "{} P2SHWPKH descriptors created in ~{} mins.",
            total_loops.to_formatted_string(&Locale::en),
            1 + creation_start.elapsed().as_secs() / 60
        ));
        Ok(p2shwpkh_descriptors)
    }

    pub fn get_all_p2tr_only_inner_key_descriptors(
        &self,
    ) -> Result<Vec<PathDescriptorPair>, RetrieverError> {
        let creation_start = Instant::now();
        let mut p2wpkh_descriptors = vec![];
        // Loop info.
        let step_size = 100_000u64;
        let mut average_step_time_in_milis = 0u128;
        let total_loops = self.path_xpubs.len() as u64;
        let mut loops_done = 0u64;
        let mut steps_done = 0u128;
        let mut steps_remaining = (total_loops / step_size) as u128;
        let pb = indicatif::ProgressBar::new(total_loops as u64)
            .with_prefix("Creating P2TR descriptors: ");
        pb.set_style(
            ProgressStyle::with_template(&format!("{{prefix}}▕{{bar:.{}}}▏ {{msg}}", "╢▌▌░╟"))
                .unwrap(),
        );
        let mut step_start_time = Instant::now();
        for PathXpubPair(path, xpub) in &self.path_xpubs {
            p2wpkh_descriptors.push(PathDescriptorPair::new(
                path.to_owned(),
                Descriptor::new_tr(
                    bitcoin::secp256k1::PublicKey::from_slice(&xpub.to_pub().to_bytes())?,
                    None,
                )?,
            ));
            // Loop info stuff.
            loops_done += 1;
            if loops_done % step_size == 0 {
                steps_done += 1;
                steps_remaining -= 1;
                average_step_time_in_milis = (step_start_time.elapsed().as_millis()
                    + (steps_done - 1) * average_step_time_in_milis)
                    / steps_done as u128;
                let remaining_time_in_milis = average_step_time_in_milis * steps_remaining;
                pb.inc(step_size);
                pb.clone().with_message(format!(
                    "{} / {}\nEstimated time to completion: ~{} minutes.",
                    loops_done.to_formatted_string(&Locale::en),
                    total_loops.to_formatted_string(&Locale::en),
                    1 + remaining_time_in_milis / 60_000,
                ));
                step_start_time = Instant::now();
            }
        }
        pb.finish_with_message(format!(
            "{} P2TR descriptors created in ~{} mins.",
            total_loops.to_formatted_string(&Locale::en),
            1 + creation_start.elapsed().as_secs() / 60
        ));
        Ok(p2wpkh_descriptors)
    }

    pub fn get_all_single_key_script_descriptors(
        &self,
    ) -> Result<Vec<PathDescriptorPair>, RetrieverError> {
        let mut single_key_path_descriptors = vec![];
        single_key_path_descriptors.extend(self.get_all_p2pk_descriptors()?);
        single_key_path_descriptors.extend(self.get_all_p2pkh_descriptors()?);
        single_key_path_descriptors.extend(self.get_all_p2shwpkh_descriptors()?);
        single_key_path_descriptors.extend(self.get_all_p2wpkh_descriptors()?);
        single_key_path_descriptors.extend(self.get_all_p2tr_only_inner_key_descriptors()?);
        Ok(single_key_path_descriptors)
    }

    // pub fn get_all_single_key_script_descriptors_string(
    //     &self,
    // ) -> Result<Vec<PathDescriptorStringPair>, RetrieverError> {
    //     let single_key_descriptors_strings = self
    //         .get_all_single_key_script_descriptors()?
    //         .iter()
    //         .map(|descriptor| descriptor.to_path_descriptor_string())
    //         .collect::<Vec<_>>();
    //     Ok(single_key_descriptors_strings)
    // }

    // pub fn get_all_single_key_scan_requests(
    //     &self,
    // ) -> Result<Vec<PathScanRequestDescriptorTrio>, RetrieverError> {
    //     let mut scan_requests = vec![];
    //     self.get_all_single_key_script_descriptors_string()?
    //         .iter()
    //         .for_each(|PathDescriptorStringPair(path, descriptor_string)| {
    //             scan_requests.push(PathScanRequestDescriptorTrio::new(
    //                 path.clone(),
    //                 ScanTxOutRequest::Single(descriptor_string.clone()),
    //             ))
    //         });
    //     Ok(scan_requests)
    // }
}
