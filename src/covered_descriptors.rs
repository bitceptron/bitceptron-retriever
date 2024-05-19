use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CoveredDescriptors {
    P2pk,
    P2pkh,
    P2wpkh,
    P2shwpkh,
    P2tr,
}