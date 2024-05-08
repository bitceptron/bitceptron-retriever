# bitceptron retriever

This is part of the bitceptron product suite. A utility crate to scan the utxo set for bitcoins locked in scripts built by custom derivation paths.

### About

Various wallets using different BIP32 derivation paths can be a problem for bitcoiners. In some cases, this might lead to confusion or a perception of loss of bitcoins, when incompatible wallets are used for receiving bitcoins and retrieving them in a later time. Bitceptron retriever is to alleviate that problem to some extent.

Bitceptron retriever uses the txout dump of bitcoincore to scan the utxo set for various descriptors derivable from your mnemonic and passphrase. We use miniscript to create the following single key descriptors:

1. P2PK.
2. P2PKH.
3. P2SHWPKH.
4. P2WPKH.
5. P2TR (Single key path spending without a script tree).

### WIP notice

Please note that this is an work in progress at the moment and breaking changes might happen.

### Future steps

We plan to:

1. Build a GUI.
2. Add async support for faster preparation.
3. Add support for multisigs.
4. Optimize the txoutset crate for retriever's specific use case of the dump file.
5. Add more documentation.
6. Increase test coverage.

### Usage

`bitceptron-retriever-cli` provides a cli tool to use this crate. Please refer to <https://github.com/bitceptron/bitceptron-retriever/bitceptron-retriever-cli> for instructions.

## Epilogue

Happy rusting plebs.
