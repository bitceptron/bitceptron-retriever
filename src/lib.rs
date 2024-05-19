//! This a light-weight crate that searches the utxo set for unspent transactions locked in spending scripts 
//! created by derived keys from a master xpriv. 
//! 

pub mod client;
pub mod uspk_set;
pub mod retriever;
pub mod setting;
pub mod error;
pub mod data;
pub mod path_pairs;
pub mod explorer;
pub mod covered_descriptors;