use std::{fs, io::BufRead, path::PathBuf, process::{Command, Stdio}, str::FromStr, thread::sleep, time::Duration};

use bitceptron_retriever::{retriever::Retriever, setting::RetrieverSetting};
use tracing_log::LogTracer;

const BITCOIND_PATH: &str = "tests/bitcoind";
const BITCOIN_CONF_PATH: &str = "tests/bitcoin.conf";
const REGTEST_PORTS: [&str; 2] = ["18998", "18999"];
const TEMP_DIR_PATH: &str = "/Users/bedlam/Desktop";
/// This example uses an already existing utxo_dump.dat file. So, make sure the temp dir path exists and 
/// contains the utxo_dump.dat file.
#[tokio::main]
async fn main() {
    LogTracer::init().unwrap();
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();
// Finding any bitcoind process using regtest ports.
    let pid_of_processes_using_ports: Vec<String> = Command::new("lsof")
        .args([
            "-i",
            format!(":{}", REGTEST_PORTS.join(",")).as_str(),
            "-a",
            "-c",
            "bitcoind",
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
    // Remove regtest from temp dir.
    let _ = fs::remove_dir_all(format!("{}/regtest", TEMP_DIR_PATH));
    // Copy bitcoin.conf to temp.
    let _ = fs::copy(BITCOIN_CONF_PATH, format!("{}/bitcoin.conf", TEMP_DIR_PATH)).unwrap();

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
    let mut ret = Retriever::new(&setting).await.unwrap();
    let _ = ret
        .check_for_dump_in_data_dir_or_create_dump_file().await
        .unwrap();
    let _ = ret.populate_uspk_set().await.unwrap();
    let _ = ret.search_the_uspk_set().await.unwrap();
    let _ = ret.get_details_of_finds_from_bitcoincore();
    let _ = ret.print_detailed_finds_on_console();
    
    // Remove regtest from temp dir.
    let _ = fs::remove_dir_all(format!("{}/regtest", TEMP_DIR_PATH));
}
