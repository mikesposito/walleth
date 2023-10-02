#![forbid(unsafe_code)]

pub use hdkey;
/// # walleth
///
/// A (WIP) Rust library for easily create, manage, use and protect Ethereum accounts. Interacting with keys and signing transactions securely (will be) made as easy as breathing.
///
/// ## Features
/// - [x] Multiple BIP39 HD wallets management
/// - [x] Built-in encryption for all keys managed
/// - [x] Built-in bytes serialization / deserialization for the entire keychain
/// - [x] Customizable wallet classes (HD, single, etc..)
/// - [ ] Built-in network scraper
/// - [ ] Built-in transaction manager
/// - [ ] Built-in JSON-RPC Provider engine
///
/// ## Usage
///
/// ### Create a new keychain
/// ```
/// use walleth::keychain::Keychain;
/// use walleth::hdkey::HDKey;
///
/// let mut keychain = Keychain::<HDKey>::new();
/// ```
///
/// ### Add a new HD Wallet to the keychain
/// ```
/// use walleth::keychain::Keychain;
/// use walleth::hdkey::{HDKey, hdkey_factory};
///
/// let mut keychain = Keychain::<HDKey>::new();
/// let hdwallet = keychain.add_multi_keypair(hdkey_factory, None).unwrap();
/// ```
///
/// ### Add a new HD Wallet to the keychain with a specific mnemonic
/// ```
/// use walleth::keychain::Keychain;
/// use walleth::hdkey::{HDKey, hdkey_factory};
///
/// let mnemonic = "grocery belt target explain clay essay focus spatial skull brain measure matrix toward visual protect owner stone scale slim ghost panda exact combine game".to_string();
/// let mut keychain = Keychain::<HDKey>::new();
/// let hdwallet = keychain.add_multi_keypair(hdkey_factory, Some(mnemonic)).unwrap();
/// ```
///
/// ### Derive keys and sign
///
/// ```
/// use walleth::{
///   keychain::Keychain,
///   hdkey::{HDKey, hdkey_factory},
///   identity::{
///     MultiKeyPair,
///     AccountDeriver,
///     signer::{Signer, Signable}
///   },
/// };
///
/// let mut keychain = Keychain::<HDKey>::new();
/// let hdwallet = keychain.add_multi_keypair(hdkey_factory, None).unwrap();
///
/// // Derive an account at path
/// let account = hdwallet.account_at(0).unwrap();
///
/// // Sign a message
/// let signature = hdwallet.sign(&account, "Hello".as_bytes()).unwrap();
///
/// // Verify signature
/// hdwallet.verify(&account, "Hello".as_bytes(), &signature);
/// ```
pub use identity;
pub use keychain;
pub use safe;
pub use utils;
pub use vault;
