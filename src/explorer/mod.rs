pub mod auxiliaries;
pub mod exploration_path;
pub mod exploration_step;
pub mod explorer_setting;

use std::sync::Arc;

use bitcoin::bip32::Xpriv;
use getset::Getters;

use tracing::info;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    error::RetrieverError,
    explorer::auxiliaries::{
        from_input_str_to_mnemonic, from_mnemonic_to_seed, from_seed_to_master_xpriv,
    },
};

use self::{exploration_path::ExplorationPath, explorer_setting::ExplorerSetting};

/// a data structure to capture the set of self-sufficient data for scanning certain paths.
#[derive(Debug, Clone, Getters)]
#[get = "pub with_prefix"]
pub struct Explorer {
    master_xpriv: Arc<Xpriv>,
    exploration_path: Arc<ExplorationPath>,
}

impl Default for Explorer {
    fn default() -> Self {
        Self {
            master_xpriv: Arc::new(
                Xpriv::new_master(bitcoin::Network::Bitcoin, &[0u8; 64]).unwrap(),
            ),
            exploration_path: Default::default(),
        }
    }
}

impl Explorer {
    pub fn new(setting: ExplorerSetting) -> Result<Self, RetrieverError> {
        info!("Creation of explorer started.");
        let exploration_path = ExplorationPath::new(
            Some(setting.get_base_derivation_paths().to_owned()),
            setting.get_exploration_path(),
            *setting.get_exploration_depth(),
            setting.get_sweep().to_owned(),
        )?;
        let mut mnemonic = from_input_str_to_mnemonic(setting.get_mnemonic())?;
        let mut seed = from_mnemonic_to_seed(mnemonic.clone(), setting.get_passphrase());
        mnemonic.zeroize();
        let master_xpriv = from_seed_to_master_xpriv(seed, *setting.get_network())?;
        seed.zeroize();
        info!("Creation of explorer finished successfully.");
        Ok(Explorer {
            master_xpriv: Arc::new(master_xpriv),
            exploration_path: Arc::new(exploration_path),
        })
    }
}

impl Zeroize for Explorer {
    fn zeroize(&mut self) {
        info!("Zeroizing explorer initialized.");
        self.master_xpriv =
            Arc::new(Xpriv::new_master(bitcoin::Network::Bitcoin, &[0u8; 64]).unwrap());
        self.exploration_path = Arc::new(ExplorationPath::new(None, "*a/*a", 10, false).unwrap());
    }
}

impl ZeroizeOnDrop for Explorer {}

#[cfg(test)]
mod tests {}
