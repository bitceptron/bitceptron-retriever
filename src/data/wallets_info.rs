// Data from https://walletsrecovery.org

use std::{collections::HashSet, str::FromStr};

use bitcoin::bip32::DerivationPath;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter)]
pub enum WalletsInfo {
    // Hardware wallets:
    AirGapVault,
    Arculus,
    BitBox01,
    BitBox02,
    CoboVault,
    Jade,
    CoboVaultWithBTCOnlyFirmware,
    ColdCardMk1,
    ColdCardMk2,
    ColdCardMk3,
    ColdCardMk4,
    CoolWalletS,
    LedgerNanoS,
    LedgerNanoX,
    Passport,
    SeedSigner,
    TrezorOne,
    TrezorModelT,
    KeepKey,
    KoinKeepHardwareWallet,
    Krux,
    Opendime,
    ProkeyOptimum,

    // Software wallets:
    AirGapWallet,           // https://airgap.it
    AtomicWallet,           // https://atomicwallet.io/
    BitcoinCore,            // https://bitcoin.org/
    BitcoinWalletApp,       // https://github.com/bitcoin-wallet/bitcoin-wallet
    Bisq,                   // https://bisq.network/
    Bither,                 // https://bither.net/
    BlockchainDotCom,       // https://www.blockchain.com/en/wallet
    BlockstreamGreen,       // https://blockstream.com/green/
    BlueWallet,             // https://bluewallet.io/
    BreadWallet,            // https://brd.com/
    BTCDotComApp,           // https://btc.com/applications/app
    Casa,                   // https://keys.casa/
    CoinWallet,             // https://coin.space/
    Coinomi,                // https://www.coinomi.com/
    Copay,                  // https://copay.io/
    DropBit,                // https://dropbit.app/
    EdgeWallet,             // https://edge.app/
    Electrum,               // https://electrum.org/
    Exodus,                 // https://exodus.io/
    FullyNoded,             // https://github.com/Fonta1n3/FullyNoded,
    HodlWallet,             // https://hodlwallet.com/
    JaxxLiberty,            // https://jaxx.io/downloads
    JoinMarket,             // https://github.com/JoinMarket-Org/joinmarket-clientserver
    JoinMarketLegacy,       // https://github.com/JoinMarket-Org/joinmarket
    LedgerLive,             // https://shop.ledger.com/pages/ledger-live
    Luxstack,               // https://luxstack.com/
    KeepKeyClient, // https://chrome.google.com/webstore/detail/keepkey-client/idgiipeogajjpkgheijapngmlbohdhjg
    KoinKeepSoftwareWallet, // https://koinkeep.com/
    MultibitHD,    // https://multibit.org/
    MyceliumAndroid, // https://wallet.mycelium.com/
    MyceliumiPhone, // https://wallet.mycelium.com/
    NthKey,        // https://nthkey.com/
    OpenBazaar,    // https://openbazaar.org/
    Pine,          // https://pine.pm/
    Relai,         // https://relai.app/
    RiseWallet,    // https://www.risewallet.com/
    Samourai,      // https://samouraiwallet.com/
    Sparrow,       // https://github.com/sparrowwallet/sparrow
    SpecterDesktop, // https://github.com/cryptoadvance/specter-desktop
    TrezorWebWallet, // https://wallet.trezor.io/
    TrustWallet,   // https://trustwallet.com/
    UnchainedCapital, // https://www.unchained-capital.com/
    UnstoppableWallet, // https://unstoppable.money/
    Wasabi,        // https://wasabiwallet.io/

    // Lightning wallets:
    BitcoinLightningWallet,    // https://lightning-wallet.com/
    SimpleBitcoinWallet,       // https://lightning-wallet.com/
    OpenBitcoinWallet,         // https://github.com/nbd-wtf/obw
    CLightning,                // https://github.com/ElementsProject/lightning
    EclairMobile,              // https://github.com/ACINQ/eclair-mobile
    LNDLightningNetworkDaemon, // https://github.com/lightningnetwork/lnd
    BlixtLNDMobileNodeWallet,  // https://github.com/hsjoberg/blixt-wallet
    StakenetDEXOpenBeta,       // https://medium.com/stakenet/stakenet-dex-open-beta-dd5c78175608
    MutinyWallet,              // https://mutinywallet.com/
    ZeusLN,                    // https://zeusln.com/

    // Combo hardware + software wallets:
    BTCPayServerANDColdcard, // https://coldcardwallet.com/
    ElectrumANDCoboVault,    // https://cobo.com/hardware-wallet/cobo-vault
    ElectrumANDColdcard,     // https://coldcardwallet.com/
    ElectrumANDLedger,       // https://ledger.com/
    ElectrumANDKeepKey,      // https://shapeshift.io/keepkey/
    ElectrumANDTrezor,       // https://trezor.com/
    WasabiANDColdcard,       // https://coldcardwallet.com/
}

impl WalletsInfo {
    pub fn get_wallet_derivation_paths(&self) -> Vec<DerivationPath> {
        match self {
            WalletsInfo::AirGapVault => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::Arculus => vec![DerivationPath::from_str("m/0'").unwrap()],
            WalletsInfo::BitBox01 => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::BitBox02 => vec![
                DerivationPath::from_str("m/48'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::CoboVault => vec![DerivationPath::from_str("m/49'/0'/0'").unwrap()],
            WalletsInfo::Jade => vec![
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::CoboVaultWithBTCOnlyFirmware => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/48'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::ColdCardMk1 => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/48'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::ColdCardMk2 => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/48'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::ColdCardMk3 => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/48'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::ColdCardMk4 => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/48'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::CoolWalletS => vec![DerivationPath::from_str("m/44'/0'/0'").unwrap()],
            WalletsInfo::LedgerNanoS => vec![
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::LedgerNanoX => vec![
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::Passport => vec![
                DerivationPath::from_str("m/48'/0'/0'/2'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/2147483646'").unwrap(),
            ],
            WalletsInfo::SeedSigner => vec![
                DerivationPath::from_str("m/48'/0'/0'/2'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::TrezorOne => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::TrezorModelT => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::KeepKey => vec![DerivationPath::from_str("m/44'/0'/0'").unwrap()],
            WalletsInfo::KoinKeepHardwareWallet => {
                vec![DerivationPath::from_str("m/44'/0'/1'").unwrap()]
            }
            WalletsInfo::Krux => vec![
                DerivationPath::from_str("m/48'/0'/0'/2'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::Opendime => vec![],
            WalletsInfo::ProkeyOptimum => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::AirGapWallet => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::AtomicWallet => vec![DerivationPath::from_str("m/44'/0'/0'/0/0").unwrap()],
            WalletsInfo::BitcoinCore => vec![DerivationPath::from_str("m/0'/0'").unwrap()],
            WalletsInfo::BitcoinWalletApp => vec![],
            WalletsInfo::Bisq => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/44'/0'/1'").unwrap(),
            ],
            WalletsInfo::Bither => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
            ],
            WalletsInfo::BlockchainDotCom => vec![DerivationPath::from_str("m/44'/0'").unwrap()],
            WalletsInfo::BlockstreamGreen => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::BlueWallet => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::BreadWallet => vec![DerivationPath::from_str("m/0'").unwrap()],
            WalletsInfo::BTCDotComApp => vec![DerivationPath::from_str("m/0'").unwrap()],
            // Casa m/49/0/X (X increments with each key rotation)
            WalletsInfo::Casa => vec![DerivationPath::from_str("m/49/0").unwrap()],
            WalletsInfo::CoinWallet => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::Coinomi => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::Copay => vec![DerivationPath::from_str("m/44'/0'").unwrap()],
            WalletsInfo::DropBit => vec![
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::EdgeWallet => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
            ],
            WalletsInfo::Electrum => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::Exodus => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::FullyNoded => vec![DerivationPath::from_str("m/84'/0'/0'").unwrap()],
            WalletsInfo::HodlWallet => vec![DerivationPath::from_str("m/0'").unwrap()],
            WalletsInfo::JaxxLiberty => vec![DerivationPath::from_str("m/44'/0'/0'").unwrap()],
            WalletsInfo::JoinMarket => vec![DerivationPath::from_str("m/84'/0'").unwrap()],
            WalletsInfo::JoinMarketLegacy => vec![DerivationPath::from_str("m/0").unwrap()],
            WalletsInfo::LedgerLive => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
            ],
            WalletsInfo::Luxstack => vec![DerivationPath::from_str("m/0'").unwrap()],
            WalletsInfo::KeepKeyClient => vec![DerivationPath::from_str("m/44'/0'/0'").unwrap()],
            // KoinKeep m/44'/0'/0'|m/44'/n'/0' (n increments with each new account created)
            WalletsInfo::KoinKeepSoftwareWallet => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/44'").unwrap(),
            ],
            WalletsInfo::MultibitHD => vec![DerivationPath::from_str("m/0'").unwrap()],
            // Mycelium for Android	m/44'|49'|84'/0'/n'
            WalletsInfo::MyceliumAndroid => vec![
                DerivationPath::from_str("m/44'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'").unwrap(),
            ],
            // Mycelium for iPhone	m/44'/0'/n'
            WalletsInfo::MyceliumiPhone => vec![DerivationPath::from_str("m/44'/0'").unwrap()],
            WalletsInfo::NthKey => vec![
                DerivationPath::from_str("m/48'/0'/0'/2'/0").unwrap(),
                DerivationPath::from_str("m/48'/0'/0'/2'/1").unwrap(),
            ],
            WalletsInfo::OpenBazaar => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/44'/1'/0'").unwrap(),
                DerivationPath::from_str("m/44'/133'/0'").unwrap(),
                DerivationPath::from_str("m/44'/145'/0'").unwrap(),
            ],
            WalletsInfo::Pine => vec![DerivationPath::from_str("m/49'/0'/0'").unwrap()],
            WalletsInfo::Relai => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'/0/0").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'/0/0").unwrap(),
            ],
            WalletsInfo::RiseWallet => vec![DerivationPath::from_str("m/49'/0'/0'").unwrap()],
            WalletsInfo::Samourai => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/47'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/2147483644'").unwrap(),
                DerivationPath::from_str("m/84'/0'/2147483645'").unwrap(),
                DerivationPath::from_str("m/84'/0'/2147483646'").unwrap(),
                DerivationPath::from_str("m/44'/0'/2147483647'").unwrap(),
                DerivationPath::from_str("m/49'/0'/2147483647'").unwrap(),
                DerivationPath::from_str("m/84'/0'/2147483647'").unwrap(),
            ],
            WalletsInfo::Sparrow => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
                DerivationPath::from_str("m/86'/0'/0'").unwrap(),
            ],
            WalletsInfo::SpecterDesktop => vec![
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::TrezorWebWallet => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
            ],
            WalletsInfo::TrustWallet => vec![DerivationPath::from_str("m/84'/0'/0'/0/0").unwrap()],
            WalletsInfo::UnchainedCapital => {
                vec![DerivationPath::from_str("m/45'/0'/0'/0/0").unwrap()]
            }

            WalletsInfo::UnstoppableWallet => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::Wasabi => vec![
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
                DerivationPath::from_str("m/86'/0'/0'").unwrap(),
            ],
            WalletsInfo::BitcoinLightningWallet => {
                vec![DerivationPath::from_str("m/84'/0'/0'").unwrap()]
            }
            WalletsInfo::SimpleBitcoinWallet => vec![
                DerivationPath::from_str("m/0'").unwrap(),
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::OpenBitcoinWallet => vec![
                DerivationPath::from_str("m/0'").unwrap(),
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::CLightning => vec![
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
                DerivationPath::from_str("m/141'/0'/0'").unwrap(),
            ],
            WalletsInfo::EclairMobile => vec![DerivationPath::from_str("m/49'/0'/0'").unwrap()],
            // aezeed
            WalletsInfo::LNDLightningNetworkDaemon => vec![],
            WalletsInfo::BlixtLNDMobileNodeWallet => {
                vec![DerivationPath::from_str("m/84'/0'/0'").unwrap()]
            }
            WalletsInfo::StakenetDEXOpenBeta => {
                vec![DerivationPath::from_str("m/44'/0'/0'").unwrap()]
            }
            WalletsInfo::MutinyWallet => vec![DerivationPath::from_str("m/86'/0'/0'").unwrap()],
            WalletsInfo::ZeusLN => vec![DerivationPath::from_str("m/86'/0'/0'").unwrap()],
            WalletsInfo::BTCPayServerANDColdcard => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::ElectrumANDCoboVault => {
                vec![DerivationPath::from_str("m/49'/0'/0'").unwrap()]
            }
            WalletsInfo::ElectrumANDColdcard => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::ElectrumANDLedger => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::ElectrumANDKeepKey => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::ElectrumANDTrezor => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
            ],
            WalletsInfo::WasabiANDColdcard => vec![
                DerivationPath::from_str("m/44'/0'/0'").unwrap(),
                DerivationPath::from_str("m/49'/0'/0'").unwrap(),
                DerivationPath::from_str("m/84'/0'/0'").unwrap(),
                DerivationPath::from_str("m/86'/0'/0'").unwrap(),
            ],
        }
    }

    pub fn get_all_unique_preset_wallet_base_paths() -> Vec<DerivationPath> {
        let mut wallet_base_paths_set = HashSet::new();
        wallet_base_paths_set.extend(
            WalletsInfo::iter()
                .flat_map(|wallet| wallet.get_wallet_derivation_paths())
                .collect::<Vec<DerivationPath>>(),
        );
        wallet_base_paths_set
            .iter()
            .map(|item| item.to_owned())
            .collect::<Vec<bitcoin::bip32::DerivationPath>>()
    }

    pub fn get_all_unique_preset_wallet_base_paths_string_vec() -> Vec<String> {
        let paths = WalletsInfo::get_all_unique_preset_wallet_base_paths();
        let paths_string = paths
            .iter()
            .map(|path| path.to_string())
            .collect::<Vec<_>>();
        paths_string
    }
}
