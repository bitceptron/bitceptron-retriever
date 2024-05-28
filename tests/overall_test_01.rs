use std::{
    fs, io::BufRead, path::PathBuf, process::{Command, Stdio}, str::FromStr, sync::Arc, thread::sleep, time::Duration
};

use bip39::Mnemonic;
use bitceptron_retriever::{retriever::Retriever, setting::RetrieverSetting};
use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    key::Secp256k1,
    Amount,
};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use miniscript::Descriptor;
use tokio::join;

const BITCOIND_PATH: &str = "tests/bitcoind";
const BITCOIN_CONF_PATH: &str = "tests/bitcoin.conf";
const REGTEST_PORTS: [&str; 2] = ["18998", "18999"];
const TEMP_DIR_PATH: &str = "tests/temp/overall_test_01";

#[tokio::test]
async fn test_with_regtest() {
    // Finding any bitcoind process using regtest ports.
    let pid_of_processes_using_ports: Vec<String> = Command::new("lsof")
        .args([
            "-i",
            format!(":{}", REGTEST_PORTS.join(",")).as_str(),
            "-a",
            "-t",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap()
        .stdout
        .lines()
        .map(|line| line.unwrap())
        .collect();
    // Killing if any.
    if !pid_of_processes_using_ports.is_empty() {
        for pid in pid_of_processes_using_ports {
            let _ = Command::new("kill")
                .args(["-9", format!("{}", pid.as_str()).as_str()])
                .spawn()
                .unwrap()
                .wait();
        }
    };
    // Create temp dir.
    let _ = fs::create_dir_all(TEMP_DIR_PATH);
    let _ = fs::remove_dir_all(format!("{}/regtest", TEMP_DIR_PATH));
    let _ = fs::remove_file(format!("{}/utxo_dump.dat", TEMP_DIR_PATH));

    // Copy bitcoin.conf to temp.
    let _ = fs::copy(BITCOIN_CONF_PATH, format!("{}/bitcoin.conf", TEMP_DIR_PATH)).unwrap();
    let _ = fs::remove_dir_all(format!("{}/regtest", TEMP_DIR_PATH));

    // Run the regtest daemon.
    Command::new(BITCOIND_PATH.to_owned())
        .args([
            "-regtest",
            "-daemon",
            format!("-port={}", REGTEST_PORTS[0]).as_str(),
            format!("-rpcport={}", REGTEST_PORTS[1]).as_str(),
            format!("-datadir={}", TEMP_DIR_PATH).as_str(),
            format!("-conf={}", "bitcoin.conf").as_str(),
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't run bitcoind.")
        .wait_with_output()
        .unwrap();
    sleep(Duration::from_millis(1000));
    // Create a bitcoincore rpc client.
    let client = Client::new(
        &format!("http://127.0.0.1:{}", REGTEST_PORTS[1]),
        Auth::CookieFile(PathBuf::from_str(&format!("{}/regtest/.cookie", TEMP_DIR_PATH)).unwrap()),
    )
    .unwrap();
    // Create a wallet for client.
    let _ = client
        .create_wallet("test", None, None, None, Some(true))
        .unwrap();
    // Create a mining address.
    let mining_address = client
        .get_new_address(
            Some("mining_address"),
            Some(bitcoincore_rpc::json::AddressType::Bech32),
        )
        .unwrap()
        .require_network(bitcoin::Network::Regtest)
        .unwrap();
    // Create our own address.
    let mnemonic_str =
        "response tag season adapt huge win catalog correct harbor cruise result east";
    let mnemonic = Mnemonic::from_str(&mnemonic_str).unwrap();
    let xpriv = Xpriv::new_master(bitcoin::Network::Regtest, &mnemonic.to_seed("")).unwrap();
    let derivation_path = DerivationPath::from_str("m/0/0'/5/8h").unwrap();
    let secretkey_for_derivation_path = xpriv
        .derive_priv(&Secp256k1::new(), &derivation_path)
        .unwrap()
        .to_priv();
    let pubkey_for_derivation_path = secretkey_for_derivation_path.public_key(&Secp256k1::new());
    let address = Descriptor::new_wpkh(pubkey_for_derivation_path)
        .unwrap()
        .address(bitcoin::Network::Regtest)
        .unwrap();
    // Make client mine some.
    let _ = client.generate_to_address(50, &mining_address);
    let _ = client.generate_to_address(50, &mining_address);
    let _ = client.generate_to_address(50, &mining_address);
    // Send 42 bitcoins to our address.
    let _txid = client
        .send_to_address(
            &address,
            Amount::from_int_btc(42),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
    // Mine to mine the transaction.
    let _ = client.generate_to_address(50, &mining_address);
    // Now retrieve.
    let setting = RetrieverSetting::new(
        Some("127.0.0.1".to_string()),
        Some(REGTEST_PORTS[1].to_string()),
        format!("{}/regtest/.cookie", TEMP_DIR_PATH),
        Some(10000),
        mnemonic_str.to_string(),
        "".to_string(),
        Some(vec!["m/0".to_string()]),
        Some("*a/*a/*a".to_string()),
        None,
        Some(false),
        Some(10),
        Some(bitcoin::Network::Regtest),
        fs::canonicalize(PathBuf::from_str(TEMP_DIR_PATH).unwrap())
            .unwrap()
            .to_string_lossy()
            .to_string(),
    );
    let mut ret = join!(Retriever::new(setting)).0.unwrap();
    let _ = join!(ret.check_for_dump_in_data_dir_or_create_dump_file());
    let _ = join!(ret.populate_uspk_set());
    let _ = join!(ret.search_the_uspk_set());
    let _ = join!(ret.get_details_of_finds_from_bitcoincore());
    let _ = ret.print_detailed_finds_on_console();
    assert_eq!(
        ret.get_detailed_finds()
            .unwrap()
            .iter()
            .fold(0u64, |acc, trio| acc
                + trio.get_scan_result().total_amount.to_sat()),
        4200000000
    );
    client.stop().unwrap();
    sleep(Duration::from_millis(100));
}
