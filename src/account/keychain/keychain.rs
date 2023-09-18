use crate::{Controller, Observable, Signer, Account, Vault};

#[derive(Clone, Debug)]
pub struct KeychainState {
	/// The accounts in the keychain
	/// This is a list of public accounts
	pub accounts: Vec<Account>,
}

/// A `Keychain` is a collection of accounts with signing capabilities
/// It is backed by an encrypted vault which holds the mnemonic
/// and some metadata for generated accounts.
pub struct Keychain {
	/// The vault
	vault: Vault,
	/// An observable wrapper around the keychain state
	store: Observable<KeychainState>,
}

impl Keychain {
	/// Create a new keychain with an empty vault
	/// and no accounts
	///
	/// # Example
	///
	/// ```
	/// use walleth::Keychain;
	///
	/// let keychain = Keychain::new();
	/// ```
	///
	pub fn new() -> Self {
		Keychain {
			vault: Vault::new(),
			store: Observable::new(KeychainState { accounts: vec![] }),
		}
	}

	/// Create a new keychain from a mnemonic
	///
	/// # Example
	///
	/// ```
	/// use walleth::Keychain;
	///
	/// let keychain = Keychain::from_mnemonic("grocery belt target explain clay essay focus spatial skull brain measure matrix toward visual protect owner stone scale slim ghost panda exact combine game".to_string());
	/// ```
	///
	pub fn from_mnemonic(mnemonic: String) -> Result<Self, String> {
		Ok(Keychain {
			vault: Vault::from_phrase(mnemonic)?,
			store: Observable::new(KeychainState { accounts: vec![] }),
		})
	}

	/// Add a new account to the keychain
	/// Returns the account
	///
	/// # Example
	///
	/// ```
	/// use walleth::Keychain;
	///
	/// let mut keychain = Keychain::new();
	/// let key = keychain.add_account();
	/// ```
	///
	pub fn add_account(&mut self) -> Result<Account, String> {
		let account = self.vault.add_key()?;
		self.store.update(|state| {
			state.accounts.push(account.clone());
		});
		Ok(account)
	}

	/// Hook into a signer for a given address
	/// Returns the result of the hook
	///
	/// # Example
	///
	/// ```
	/// use walleth::Keychain;
	/// use walleth::signer::Signer;
	///
	/// let keychain = Keychain::new();
	/// let key = keychain.add_account();
	///
	/// keychain.use_signer(key.address, |signer) {
	///   signer.sign(&[0; 32])
	/// })
	pub fn use_signer<T, R>(&self, address: String, hook: T) -> Result<R, String>
	where
		T: FnMut(&Signer) -> R,
	{
		match self
			.store
			.get_state()
			.accounts
			.iter()
			.enumerate()
			.find(|(_, key)| key.address == address)
		{
			Some((key_index, _)) => self.vault.use_signer(key_index, hook),
			None => Err("Key not found".to_string()),
		}
	}

	/// Lock the keychain
	/// This will lock the vault, removing all
	/// private keys from memory
	///
	/// # Example
	///
	/// ```
	/// use walleth::Keychain;
	///
	/// let mut keychain = Keychain::new();
	/// keychain.lock();
	/// ```
	pub fn lock(&mut self, password: &str) -> Result<(), String> {
		self.vault.lock(password.as_bytes())
	}

	/// Unlock the keychain
	///
	/// # Example
	///
	/// ```
	/// use walleth::Keychain;
	///
	/// let mut keychain = Keychain::new();
	/// keychain.unlock("password");
	/// ```
	pub fn unlock(&mut self, password: &str) -> Result<(), String> {
		self.vault.unlock(password.as_bytes())
	}
}

impl Controller<KeychainState> for Keychain {
	/// Get the state of the keychain
	///
	/// # Example
	///
	/// ```
	/// use walleth::Keychain;
	/// use walleth::Controller;
	///
	/// let keychain = Keychain::new();
	/// let state = keychain.get_state();
	/// ```
	fn get_state(&self) -> &KeychainState {
		&self.store.get_state()
	}

	/// Update the state of the keychain
	///
	/// # Example
	///
	/// ```
	/// use walleth::Keychain;
	/// use walleth::Controller;
	///
	/// let mut keychain = Keychain::new();
	/// keychain.update(|state| {
	///  state.accounts.push(Key::from_extended_public_key(&[0; 64]).unwrap());
	/// });
	/// ```
	fn update<F>(&mut self, updater: F) -> ()
	where
		F: Fn(&mut KeychainState),
	{
		self.store.update(updater);
	}

	/// Subscribe to state changes
	///
	/// # Example
	///
	/// ```
	/// use walleth::Keychain;
	/// use walleth::Controller;
	///
	/// let mut keychain = Keychain::new();
	/// let id = keychain.subscribe(|state| {
	///  println!("New state: {:?}", state);
	/// });
	/// ```
	fn subscribe<F>(&mut self, subscriber: F) -> usize
	where
		F: 'static + FnMut(&KeychainState),
	{
		self.store.subscribe(subscriber)
	}

	/// Unsubscribe from state changes
	///
	/// # Example
	///
	/// ```
	/// use walleth::Keychain;
	/// use walleth::Controller;
	///
	/// let mut keychain = Keychain::new();
	/// let id = keychain.subscribe(|state| {
	///  println!("New state: {:?}", state);
	/// });
	/// keychain.unsubscribe(id);
	fn unsubscribe(&mut self, id: usize) -> () {
		self.store.unsubscribe(id)
	}
}