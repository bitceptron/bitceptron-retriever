use bip39::Mnemonic;
use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    key::Secp256k1,
};

use crate::error::RetrieverError;

pub fn from_seed_to_master_xpriv(
    seed: [u8; 64],
    network: bitcoin::Network,
) -> Result<Xpriv, RetrieverError> {
    let master_xpriv = Xpriv::new_master(network, &seed)?;
    Ok(master_xpriv)
}

pub fn from_master_xpriv_to_base_xpriv(
    master: Xpriv,
    path: DerivationPath,
) -> Result<Xpriv, RetrieverError> {
    let secp = Secp256k1::new();
    let base_xpriv = master.derive_priv(&secp, &path)?;
    Ok(base_xpriv)
}

pub fn from_input_str_to_mnemonic(input: &str) -> Result<bip39::Mnemonic, RetrieverError> {
    let mnemonic = bip39::Mnemonic::parse_in_normalized(bip39::Language::English, input)?;
    Ok(mnemonic)
}

pub fn from_mnemonic_to_seed(mnemonic: Mnemonic, passphrase: &str) -> [u8; 64] {
    mnemonic.to_seed(passphrase)
}

#[cfg(test)]
mod tests {
    // Used https://learnmeabitcoin.com/technical/keys/hd-wallets/mnemonic-seed/ for test cases.

    use std::str::FromStr;

    use super::*;

    #[test]
    fn normal_12_words_mnemonic_works() {
        let input = "camera phrase loan curtain island hammer soft fault hockey enter power busy";
        let mnemonic = from_input_str_to_mnemonic(input).unwrap();
        let expected = Mnemonic::from_str(
            "camera phrase loan curtain island hammer soft fault hockey enter power busy",
        )
        .unwrap();
        assert_eq!(mnemonic, expected)
    }

    #[test]
    fn normal_24_words_mnemonic_works() {
        let input = "trophy sock action walk brother media cousin enemy stuff civil dizzy hidden fan joke cause slender access few beef winner toddler blade gasp welcome";
        let mnemonic = from_input_str_to_mnemonic(input).unwrap();
        let expected = Mnemonic::from_str("trophy sock action walk brother media cousin enemy stuff civil dizzy hidden fan joke cause slender access few beef winner toddler blade gasp welcome").unwrap();
        assert_eq!(mnemonic, expected)
    }

    #[test]
    fn seed_gen_works_wo_passphrase() {
        let input = "ahead since shoe review home mirror creek cry ability industry liquid depart citizen volcano naive talent output eternal stereo bless ski like loop tape";
        let mnemonic = from_input_str_to_mnemonic(input).unwrap();
        let passphrase = "";
        let seed = from_mnemonic_to_seed(mnemonic, passphrase);
        let expected = hex::decode("6e1145dd3d82911969f1e582ff5eea1acad7ec5b5fec7292f2853718ff8914883536c5a90d358630c73de8e1fbf58c5e93d91bba605f9af4e59f83d4e494d839").unwrap();
        assert_eq!(seed.to_vec(), expected)
    }

    #[test]
    fn seed_gen_works_w_passphrase() {
        let input = "ahead since shoe review home mirror creek cry ability industry liquid depart citizen volcano naive talent output eternal stereo bless ski like loop tape";
        let mnemonic = from_input_str_to_mnemonic(input).unwrap();
        let passphrase = "mnemonic";
        let seed = from_mnemonic_to_seed(mnemonic, passphrase);
        let expected = hex::decode("15d3623d6af7790aa70cc21fd19fbbae6494e457369e2d4aef13b3663e251425f64aa1835b8ddd634055a0ee501292ab0ae7b9f30432db897f65fed14ac8b4b7").unwrap();
        assert_eq!(seed.to_vec(), expected);

        let input = "ahead since shoe review home mirror creek cry ability industry liquid depart citizen volcano naive talent output eternal stereo bless ski like loop tape";
        let mnemonic = from_input_str_to_mnemonic(input).unwrap();
        let passphrase = "hard password";
        let seed = from_mnemonic_to_seed(mnemonic, passphrase);
        let expected = hex::decode("87b50b8fbda1509700852f6ad3a0f9c8ee6ba076716a3bdf77044b5b8d48d49993384a10a2994713d63147517862fad9dc7989eea3ca9471fce0a13b823c7cd2").unwrap();
        assert_eq!(seed.to_vec(), expected);
    }
}
