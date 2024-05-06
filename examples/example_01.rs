use bitceptron_retriever::{retriever::Retriever, setting::RetrieverSetting};

fn main() {
    let setting = RetrieverSetting {
        bitcoincore_rpc_url: Some("127.0.0.1".to_string()),
        bitcoincore_rpc_port: Some("18333".to_string()),
        bitcoincore_rpc_cookie_path:
            "/Users/bedlam/Library/Application Support/Bitcoin/regtest/.cookie".to_string(),
        bitcoincore_rpc_timeout_seconds: Some(10000),
        mnemonic: Some(
            "word connect future boring bird aisle cute height pumpkin danger calm knock"
                .to_string(),
        ),
        passphrase: Some("".to_string()),
        base_derivation_paths: Some(vec!["m/0'/0".to_string(), "m/4000'".to_string()]),
        exploration_path: Some("*'/*a/*".to_string()),
        sweep: Some(false),
        exploration_depth: Some(44),
        network: Some(bitcoin::Network::Bitcoin),
        data_dir: "/Users/bedlam/Desktop".to_string(),
    };
    let mut ret = Retriever::new(setting).unwrap();
    let _ = ret.populate_uspk_set().unwrap();
    let res = ret.search_the_uspk_set().unwrap();
    println!("{res:?}");
}
