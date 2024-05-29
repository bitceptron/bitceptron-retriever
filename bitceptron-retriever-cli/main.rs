use bitceptron_retriever::{retriever::Retriever, setting::RetrieverSetting};
use clap::{Arg, Command};
use tracing_log::LogTracer;

#[tokio::main]
async fn main() {
    LogTracer::init().unwrap();
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();
    let matches = Command::new("Bitceptron Scanner")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Scans the UTXO set for BIP32 custom exploration paths from various derivation paths in use by bitcoin wallets.")
        .author("bitceptron")
        .arg(
            Arg::new("conf")
                .long("conf")
                .short('c')
                .help("Path to the config.toml file.")
                .required(true)
        ).get_matches();

    let config_file_path_string = matches.get_one::<String>("conf").expect("required");

    let setting = RetrieverSetting::from_config_file(config_file_path_string)
        .map_err(|err| panic!("Error while reading the config file: {:#?}", err))
        .unwrap();
    let mut ret = Retriever::new(setting)
        .await
        .map_err(|err| panic!("Error while creating the retriever: {:#?}", err))
        .unwrap();
    ret.check_for_dump_in_data_dir_or_create_dump_file()
        .await
        .map_err(|err| {
            panic!(
                "Error while checking/creating dump file in data dir: {:#?}",
                err
            )
        })
        .unwrap();
    ret.populate_uspk_set()
        .await
        .map_err(|err| panic!("Error while populating in-memory UTXO database: {:#?}", err))
        .unwrap();
    ret.search_the_uspk_set()
        .await
        .map_err(|err| panic!("Error while searching in-memory UTXO database: {:#?}", err))
        .unwrap();
    ret.get_details_of_finds_from_bitcoincore()
        .await
        .map_err(|err| {
            panic!(
                "Error while fetching details of finds from bitcoincore: {:#?}",
                err
            )
        })
        .unwrap();
    let _ = ret.print_detailed_finds_on_console();
}
