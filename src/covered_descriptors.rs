use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum CoveredDescriptors {
    P2pk,
    P2pkh,
    P2wpkh,
    P2shwpkh,
    P2tr,
}