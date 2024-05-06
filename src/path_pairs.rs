use std::str::FromStr;

use bitcoin::{
    bip32::DerivationPath, key::Secp256k1, secp256k1::SecretKey, Amount, BlockHash, ScriptBuf, Txid,
};
use bitcoincore_rpc::json::{ScanTxOutRequest, ScanTxOutResult, Utxo};
use miniscript::{
    bitcoin::{bip32::Xpub, secp256k1::PublicKey},
    Descriptor,
};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathScriptPubKeyBytesPair(DerivationPath, Vec<u8>);

// impl PathScriptPubKeyBytesPair {
//     pub fn new(path: DerivationPath, descriptor: Descriptor<PublicKey>) -> Self {
//         PathScriptPubKeyBytesPair(path, descriptor.script_pubkey().to_bytes())
//     }

//     pub fn to_path_descriptor_string(&self) -> PathDescriptorStringPair {
//         let spk = ScriptBuf::from_bytes(self.1).to;
//         PathDescriptorStringPair::new(self.0.clone(), Descriptor::fr)
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathDescriptorPair(pub(crate) DerivationPath, pub(crate) Descriptor<PublicKey>);

impl PathDescriptorPair {
    pub fn new(path: DerivationPath, descriptor: Descriptor<PublicKey>) -> Self {
        PathDescriptorPair(path, descriptor)
    }

    pub fn to_path_scan_request_descriptor_trio(&self) -> PathScanRequestDescriptorTrio {
        let scan_request = ScanTxOutRequest::Single(self.1.to_string());
        PathScanRequestDescriptorTrio(self.0.clone(), scan_request, self.1.clone())
    }

    pub fn to_path_descriptor_string(&self) -> PathDescriptorStringPair {
        PathDescriptorStringPair::new(self.0.clone(), self.1.to_string())
    }
}

impl Zeroize for PathDescriptorPair {
    fn zeroize(&mut self) {
        let paths = vec!["0".to_string(); self.0.len()].join::<&str>("/");
        self.0 = DerivationPath::from_str(format!("m/{}", paths).as_str()).unwrap();
        self.1 = Descriptor::new_pkh(
            SecretKey::from_slice(&[0u8; 32])
                .unwrap()
                .public_key(&Secp256k1::new()),
        )
        .unwrap();
    }
}

impl ZeroizeOnDrop for PathDescriptorPair {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathDescriptorStringPair(pub(crate) DerivationPath, pub(crate) String);

impl PathDescriptorStringPair {
    pub fn new(path: DerivationPath, descriptor_string: String) -> Self {
        PathDescriptorStringPair(path, descriptor_string)
    }
}

impl Zeroize for PathDescriptorStringPair {
    fn zeroize(&mut self) {
        let paths = vec!["0".to_string(); self.0.len()].join::<&str>("/");
        self.0 = DerivationPath::from_str(format!("m/{}", paths).as_str()).unwrap();
        self.1.zeroize();
    }
}

impl ZeroizeOnDrop for PathDescriptorStringPair {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathXpubPair(pub(crate) DerivationPath, pub(crate) Xpub);

impl PathXpubPair {
    pub fn new(path: DerivationPath, xpub: Xpub) -> Self {
        PathXpubPair(path, xpub)
    }
}

impl Zeroize for PathXpubPair {
    fn zeroize(&mut self) {
        let paths = vec!["0".to_string(); self.0.len()].join::<&str>("/");
        self.0 = DerivationPath::from_str(format!("m/{}", paths).as_str()).unwrap();
        self.1 = Xpub::from_priv(
            &Secp256k1::new(),
            &bitcoin::bip32::Xpriv::new_master(bitcoin::Network::Bitcoin, &[0u8; 32]).unwrap(),
        );
    }
}

impl ZeroizeOnDrop for PathXpubPair {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathScanRequestDescriptorTrio(
    pub(crate) DerivationPath,
    pub(crate) ScanTxOutRequest,
    pub(crate) Descriptor<PublicKey>,
);

impl PathScanRequestDescriptorTrio {
    pub fn new(
        path: DerivationPath,
        scan_request: ScanTxOutRequest,
        descriptor: Descriptor<PublicKey>,
    ) -> Self {
        PathScanRequestDescriptorTrio(path, scan_request, descriptor)
    }

    pub fn from_path_descriptor_pair(path_descriptor_pair: PathDescriptorPair) -> Self {
        let scan_request = ScanTxOutRequest::Single(path_descriptor_pair.1.to_string());
        PathScanRequestDescriptorTrio(path_descriptor_pair.0, scan_request, path_descriptor_pair.1)
    }
}

impl Zeroize for PathScanRequestDescriptorTrio {
    fn zeroize(&mut self) {
        let paths = vec!["0".to_string(); self.0.len()].join::<&str>("/");
        self.0 = DerivationPath::from_str(format!("m/{}", paths).as_str()).unwrap();
        self.1 = ScanTxOutRequest::Single("00000000000000000".to_string());
    }
}

impl ZeroizeOnDrop for PathScanRequestDescriptorTrio {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathScanResultDescriptorTrio(
    pub(crate) DerivationPath,
    pub(crate) ScanTxOutResult,
    pub(crate) Descriptor<PublicKey>,
);

impl PathScanResultDescriptorTrio {
    pub fn new(
        path: DerivationPath,
        scan_result: ScanTxOutResult,
        descriptor: Descriptor<PublicKey>,
    ) -> Self {
        PathScanResultDescriptorTrio(path, scan_result, descriptor)
    }

    pub fn get_derivation_path(&self) -> DerivationPath {
        self.0.clone()
    }

    pub fn get_scan_result(&self) -> ScanTxOutResult {
        self.1.clone()
    }

    pub fn get_descriptor(&self) -> Descriptor<PublicKey> {
        self.2.clone()
    }
}

impl Zeroize for PathScanResultDescriptorTrio {
    fn zeroize(&mut self) {
        let paths = vec!["0".to_string(); self.0.len()].join::<&str>("/");
        self.0 = DerivationPath::from_str(format!("m/{}", paths).as_str()).unwrap();
        self.1 = ScanTxOutResult {
            success: Some(false),
            tx_outs: Some(42),
            height: Some(5326),
            best_block_hash: Some(
                BlockHash::from_str(
                    "00000000000000000002ac885fab3cd598f5ae4092fc92b3d4c7096ef0f0caae",
                )
                .unwrap(),
            ),
            unspents: vec![Utxo {
                txid: Txid::from_str(
                    "f3aa99937337582a105c90e0595847177d8ab99d50201e318634a5d2db4f9d85",
                )
                .unwrap(),
                vout: 21,
                script_pub_key: ScriptBuf::from_hex(
                    "a91442402a28dd61f2718a4b27ae72a4791d5bbdade787",
                )
                .unwrap(),
                descriptor: "none".to_string(),
                amount: Amount::from_sat(2100000000000000),
                height: 42,
            }],
            total_amount: Amount::from_sat(2100000000000000),
        };
    }
}

impl ZeroizeOnDrop for PathScanResultDescriptorTrio {}
