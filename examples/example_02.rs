use std::{fs, path::PathBuf, str::FromStr};

use bitceptron_retriever::{retriever::Retriever, setting::RetrieverSetting};
use tracing_log::LogTracer;

const REGTEST_PORTS: [&str; 2] = ["18998", "18999"];
const TEMP_DIR_PATH: &str = "tests/temp";

#[tokio::main]
async fn main() {
    LogTracer::init().unwrap();
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();

    // Create our own address.
    let mnemonic_str =
        "response tag season adapt huge win catalog correct harbor cruise result east";

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
    let mut ret = Retriever::new(setting).await.unwrap();
    let _ = ret
        .check_for_dump_in_data_dir_or_create_dump_file()
        .unwrap();
    let _ = ret.populate_uspk_set().unwrap();
    let _ = ret.search_the_uspk_set().await.unwrap();
    let _ = ret.get_details_of_finds_from_bitcoincore();
    let _ = ret.print_detailed_finds_on_console();
    assert_eq!(
        ret.get_detailed_finds()
            .unwrap()
            .iter()
            .fold(0u64, |acc, trio| acc
                + trio.get_scan_result().total_amount.to_sat()),
        4200000000
    );
}
