use crate::{
  keychain::{Account, KeychainError, Signer, Vault},
  utils::{Controller, Observable},
};

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
  ///
  /// assert!(keychain.is_ok());
  /// ```
  ///
  pub fn from_mnemonic(mnemonic: String) -> Result<Self, KeychainError> {
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
  ///
  /// assert!(key.is_ok());
  /// ```
  ///
  pub fn add_account(&mut self) -> Result<Account, KeychainError> {
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
  /// use walleth::{Keychain, Signable};
  ///
  /// let mut keychain = Keychain::new();
  /// let key = keychain.add_account().unwrap();
  /// let message = Signable::from_str("Hello world!");
  ///
  /// let signature = keychain.use_signer(key.address, |signer| {
  ///   signer.sign(&message)
  /// });
  ///
  /// assert!(signature.is_ok());
  /// ```
  pub fn use_signer<T, R>(&self, address: String, hook: T) -> Result<R, KeychainError>
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
      Some((key_index, _)) => Ok(self.vault.use_signer(key_index, hook)?),
      None => Err(KeychainError::KeyNotFoundForAddress(address)),
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
  ///
  /// keychain.lock("my password").unwrap();
  /// let key = keychain.add_account();
  ///
  /// assert!(!key.is_ok());
  /// ```
  pub fn lock(&mut self, password: &str) -> Result<(), KeychainError> {
    self.store.update(|state| {
      state.accounts = vec![];
    });

    Ok(self.vault.lock(password.as_bytes())?)
  }

  /// Unlock the keychain
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::Keychain;
  ///
  /// let mut keychain = Keychain::new();
  /// let account = keychain.add_account().unwrap();
  /// keychain.lock("password").unwrap();
  ///
  /// let recovered_accounts = keychain.unlock("password").unwrap();
  ///
  /// assert_eq!(account.address, recovered_accounts[0].address);
  /// ```
  pub fn unlock(&mut self, password: &str) -> Result<&Vec<Account>, KeychainError> {
    let recovered_accounts = self.vault.unlock(password.as_bytes())?;
    self.store.update(|state| {
      state.accounts = recovered_accounts.clone();
    });

    Ok(&self.store.get_state().accounts)
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
  ///
  /// assert_eq!(state.accounts.len(), 0);
  /// ```
  fn get_state(&self) -> &KeychainState {
    &self.store.get_state()
  }

  /// Update the state of the keychain
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::{Keychain, Controller, Account};
  ///
  /// let mut keychain = Keychain::new();
  /// keychain.add_account();
  ///
  /// keychain.update(|state| {
  ///  state.accounts = vec![];
  /// });
  ///
  /// assert_eq!(keychain.get_state().accounts.len(), 0);
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
  ///   assert_eq!(state.accounts.len(), 1);
  /// });
  ///
  /// keychain.add_account();
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
