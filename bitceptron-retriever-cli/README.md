# bitceptron retriever cli

This is part of the bitceptron product suite.

### About

Various wallets using different BIP32 derivation paths can be a problem for bitcoiners. In some cases, this might lead to confusion or a perception of loss of bitcoins, when incompatible wallets are used for receiving bitcoins and retrieving them in a later time. Bitceptron retriever is to alleviate that problem to some extent.

Bitceptron retriever uses the txout dump of bitcoincore to scan the utxo set for various descriptors derivable from your mnemonic and passphrase. We use miniscript to create the following single key descriptors:

1. P2PK.
2. P2PKH.
3. P2SHWPKH.
4. P2WPKH.
5. P2TR (Single key path spending without a script tree).

### WIP notice

Please note that this is an work in progress at the moment.

### `config.toml`

To use bitceptron retriever, you have to create a config file named `config.toml`. Let's walk through the items in `config.toml`:

#### bitcoincore_rpc_cookie_path (MUST BE PROVIDED)

This is the only config that has not a default value. You must provide this in your `config.toml` file, for bitceptron retriever to be able to authenticate a connection to bitcoincore rpc. `.cookie` can be found in in your bitcoincore folder. When you run bitcoind, the address is shown in the first few lines. Unless you are using user pass authentication method which is to be deprecated.

`bitcoincore_rpc_cookie_path = "/your/path/to/.cookie"`

#### bitcoincore_rpc_url

This is the url for bitcoincore. If not set, `bitcoincore_rpc_url` defaults to `127.0.0.1`.

`bitcoincore_rpc_url = "127.0.0.1"`

#### bitcoincore_rpc_port

This is the **rpc port** for bitcoincore. If not set, `bitcoincore_rpc_port` defaults to `8332`.

`bitcoincore_rpc_port = "8332"`

#### bitcoincore_rpc_timeout_seconds

This sets the timeout for rpc client in seconds. Essentially this is the period in which the client keeps the connection alive while bitcoincore is answering client's request. The default for bitcoincore-rpc crate is 15 seconds which is a tad too short for our purposes. If not set, `bitcoincore_rpc_timeout_seconds` defaults to `6800`.

`bitcoincore_rpc_timeout_seconds = "6800"`

#### mnemonic (MUST BE PROVIDED)

This is the mnemonic you must have according to BIP39. You can either enter your mnemonic here, or be prompted by the application to enter mnemonics manually.

**MAKE SURE YOU SECURELY DELETE THIS FILE IF YOU DECIDE TO ENTER THE MNEMONIC HERE!!!**

`mnemonic = "grass tribe october slam curve pave glory false mule snake wood high"`

#### passphrase (MUST BE PROVIDED)

This is the optional passphrase you might have set according to BIP39. You can either enter your passphrase here, or be prompted
by the application to enter it manually. If not set, leave it empty here, or just press Enter when prompted by the program.

**MAKE SURE YOU SECURELY DELETE THIS FILE IF YOU DECIDE TO ENTER THE PASSPHRASE HERE!!!**

`passphrase = "strong passphrase"`

#### base_derivation_paths

This is a vector of base derivation paths. These are the fixed parts of the derivation path, after which the exploration
starts. These base paths should comply with these formatting rules:

1. Must start with "m"
2. Each child should be separated by a "/"
3. Children may be normal or hardened. Normal children are just numbers and hardened children are numbers followed by either of "h" or " ' " characters.

Some valid examples:

- "m/84'/0/0"
- "m/40/0h/0h"
- "m/0/1/2'/4h/8"

If base_derivation_paths is not set, it will use the built-in list of all known base paths for bitcoin wallets which is based on the data provided by <https://walletsrecovery.org>  

`base_derivation_paths = ["m/42'/0'/0", "m/42/0h"]`

#### exploration_path

This is the exploration path in which the program searches. Exploration path consists of steps separated by a "/". Step semantics are as follows:

1. For any A, a member of u32: A means the specific child number A of the parent.
2. For any A and B, members of u32 with A <= B: A..B means all children number A (inclusive) to number B (inclusive) of the parent.
3. For and A, a member of u32: ..A means all the children from number 0 (inclusive) to number B (inclusive) of the parents.
4. " * " means all children from (inclusive) 0 to exploration_depth (inclusive).
5. suffixes " ' " and " h " mean all hardened children. Not using these suffixes makes all children in that step normal.
6. Suffix " a " means exploring both hardened and normal children at that step.

Some valid examples:

- "..100'/50..75a/*/*"
- "42a/83..120a/68h/*a/54h"
- "*'/*h/*a"

`exploration_path = "..5'/6a/..5"`

#### sweep

sweep is a boolean. If set to true, it will sweep the exploration path from the root to the last step. For example, if the base path is "m/0h" and the exploration path is "*/*h/..100", by setting sweep to true, all these paths get explored: "m/0h", "m/0h/\*", "m/0h/*/*h" and "m/0h/*/*h/..100". If set to false, only the specified path will be explored. As per the last example, the explored path would be "m/0h/*/*h/..100". If not set, defaults to false.

`sweep = true`

#### exploration_depth

This is the exploration depth. When using the * in exploration path, all children from 0 to this number (all inclusive) will be explored. If not set, defaults to 100.

`exploration_depth = "100"`

#### network

Indicated the network and can be: "Bitcoin", "Testnet", "Regtest" or "Signet".
If not set, defaults to "Bitcoin".

`network = "Bitcoin"`

### Usage

To use the bitceptron-retriever-cli, you must follow these steps:

1. Make sure you have a bitcoind instance running with an rpc port open to requests.
2. Create your config.toml file as per your requirements.
3. Build the `bitceptron-retriever-cli` from source (`cargo build --release`) or download pertinent executable.
4. run `./bitceptron-retriever-cli --conf=<path to your config.toml file>` from where you put your release build which defaults to `target/release` or run `cargo run --release -- --conf=<path to your config.toml file>` from the root of the repository.

It takes about 15 minutes to build the in-memory utxo database of about 181m UTXOs. Building xpubs takes a bit of time too.

## Epilogue

Happy rusting plebs.
