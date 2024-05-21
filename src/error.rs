#[derive(Debug)]
pub enum RetrieverError {
    BitcoincoreRpcCrateError(bitcoincore_rpc::Error),
    JsonRpcHttpError(bitcoincore_rpc::jsonrpc::simple_http::Error),
    BitcoincoreRpcUnreachable,
    DumpFileAlreadyExistsInPath,
    IoError(std::io::Error),
    ConsensusEncodeError(bitcoincore_rpc::bitcoin::consensus::encode::Error),
    InvalidExplorationPath,
    Bip32Error(bitcoin::bip32::Error),
    InvalidStepRange,
    Bip39Error(bip39::Error),
    MiniscriptError(miniscript::Error),
    Secp256k1Error(bitcoin::secp256k1::Error),
    NoDumpFileInDataDir,
    UnspentScriptPublicKeySetIsNotPopulated,
    NoSearchHasBeenPerformed,
    DetailsHaveNotBeenFetched,
    ConfigError(config::ConfigError),
    TokioJoinError(tokio::task::JoinError),
    PopulatingUSPKSetInProgress,
    USPKSetAlreadyPopulated,
}

impl From<bitcoincore_rpc::Error> for RetrieverError {
    fn from(value: bitcoincore_rpc::Error) -> Self {
        RetrieverError::BitcoincoreRpcCrateError(value)
    }
}

impl From<bitcoincore_rpc::jsonrpc::simple_http::Error> for RetrieverError {
    fn from(value: bitcoincore_rpc::jsonrpc::simple_http::Error) -> Self {
        RetrieverError::JsonRpcHttpError(value)
    }
}

impl From<std::io::Error> for RetrieverError {
    fn from(value: std::io::Error) -> Self {
        RetrieverError::IoError(value)
    }
}

impl From<bitcoincore_rpc::bitcoin::consensus::encode::Error> for RetrieverError {
    fn from(value: bitcoincore_rpc::bitcoin::consensus::encode::Error) -> Self {
        RetrieverError::ConsensusEncodeError(value)
    }
}

impl From<bitcoin::bip32::Error> for RetrieverError {
    fn from(value: bitcoin::bip32::Error) -> Self {
        RetrieverError::Bip32Error(value)
    }
}

impl From<bip39::Error> for RetrieverError {
    fn from(value: bip39::Error) -> Self {
        RetrieverError::Bip39Error(value)
    }
}

impl From<miniscript::Error> for RetrieverError {
    fn from(value: miniscript::Error) -> Self {
        RetrieverError::MiniscriptError(value)
    }
}

impl From<bitcoin::secp256k1::Error> for RetrieverError {
    fn from(value: bitcoin::secp256k1::Error) -> Self {
        RetrieverError::Secp256k1Error(value)
    }
}

impl From<config::ConfigError> for RetrieverError {
    fn from(value: config::ConfigError) -> Self {
        RetrieverError::ConfigError(value)
    }
}

impl From<tokio::task::JoinError> for RetrieverError {
    fn from(value: tokio::task::JoinError) -> Self {
        RetrieverError::TokioJoinError(value)
    }
}