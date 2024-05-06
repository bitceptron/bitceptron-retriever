use getset::Getters;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// This is to collect the json response for `dumptxoutset` rpc call.
#[derive(Debug, Serialize, Deserialize, Getters, Clone, Zeroize, ZeroizeOnDrop)]
#[get = "pub with_prefix"]
pub struct DumpTxoutSetResult {
    coins_written: u64,
    base_hash: String,
    base_height: u64,
    path: String,
    txoutset_hash: String,
    nchaintx: u64,
}

impl DumpTxoutSetResult {
    pub fn new(
        coins_written: u64,
        base_hash: String,
        base_height: u64,
        path: String,
        txoutset_hash: String,
        nchaintx: u64,
    ) -> Self {
        DumpTxoutSetResult {
            coins_written,
            base_hash,
            base_height,
            path,
            txoutset_hash,
            nchaintx,
        }
    }
}
