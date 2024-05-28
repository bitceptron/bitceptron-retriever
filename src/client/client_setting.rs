use getset::Getters;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Settings used for creating a bitcoincore rpc client.
#[derive(Debug, Zeroize, ZeroizeOnDrop, Getters, Default)]
#[get = "pub with_prefix"]
pub struct ClientSetting {
    rpc_url: String,
    rpc_port: String,
    /// Usually resides in the datadir of your bitcoin setup (.bitcoin folder).
    cookie_path: String,
    /// This is the time period in which the rpc connection stays alive despite not receiving a response from bitcoincore.
    /// It is important to set this high enough for creating a utxo set dump or scanning the utxo set takes more than the default 15 seconds.
    timeout_seconds: u64,
}

impl ClientSetting {
    pub fn new(rpc_url: &str, rpc_port: &str, cookie_path: &str, timeout_seconds: u64) -> Self {
        ClientSetting {
            rpc_url: rpc_url.to_string(),
            rpc_port: rpc_port.to_string(),
            cookie_path: cookie_path.to_string(),
            timeout_seconds,
        }
    }
}
