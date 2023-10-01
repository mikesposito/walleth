use hdkey::HDKey;
use serde::{Deserialize, Serialize};

use super::KeychainError;
use identity::{Account, IdentityError, Initializable, MultiKeyPair};
use utils::{Controller, Observable};
use vault::Vault;

pub enum KeyPair<M = HDKey>
where
  M: MultiKeyPair<[u8; 32], [u8; 33], usize>,
{
  MultiKeyPair(Vault<M>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeychainState {
  /// The accounts in the keychain
  /// This is a list of public accounts
  pub accounts: Vec<Account<usize>>,
}

/// A `Keychain` is a collection of keyparis with different capabilities.
/// Each keypair is stored in a `Vault`, which provides basic encryption features
/// and serialization / deserialization from bytes.
pub struct Keychain<M = HDKey>
where
  M: MultiKeyPair<[u8; 32], [u8; 33], usize>,
{
  /// Key pairs handled by the keychain
  key_pairs: Vec<KeyPair<M>>,
  /// An observable wrapper around the keychain state
  store: Observable<KeychainState>,
}

impl<M> Keychain<M>
where
  M: MultiKeyPair<[u8; 32], [u8; 33], usize>,
{
  /// Create a new keychain
  pub fn new() -> Self {
    Keychain {
      key_pairs: vec![],
      store: Observable::new(KeychainState { accounts: vec![] }),
    }
  }

  /// Add a new `KeyPair` to the `Keychain` with multiple 
  /// private keys derivation capabilities
  pub fn add_multi_keypair<F, A>(&mut self, factory: F, args: A) -> Result<&M, KeychainError>
  where
    F: FnOnce(A) -> Result<M, Box<dyn IdentityError>>,
  {
    let keypair = KeyPair::MultiKeyPair(Vault::new(factory, args)?);
    self.key_pairs.push(keypair);

    match self.key_pairs.last().unwrap() {
      KeyPair::MultiKeyPair(vault) => Ok(vault.get_identity()?),
    }
  }

  /// Get an identity from the keychain
  pub fn get_keypair(&self, at_index: usize) -> Option<&KeyPair<M>> {
    self.key_pairs.get(at_index)
  }

  /// Get a mutable identity from the keychain
  pub fn get_keypair_mut(&mut self, at_index: usize) -> Option<&mut KeyPair<M>> {
    self.key_pairs.get_mut(at_index)
  }

  /// Lock the keychain
  /// This will lock all the internal vaults, removing all
  /// private keys from memory
  pub fn lock(&mut self, password: &str) -> Result<(), KeychainError>
  where
    M: Initializable,
  {
    self.store.update(|state| {
      state.accounts = vec![];
    })?;

    Ok(
      self
        .key_pairs
        .iter_mut()
        .try_for_each(|keypair| match keypair {
          KeyPair::MultiKeyPair(vault) => vault.lock(password.as_bytes()),
        })?,
    )
  }

  /// Unlock the keychain
  pub fn unlock(&mut self, password: &str) -> Result<(), KeychainError>
  where
    M: Initializable,
  {
    Ok(
      self
        .key_pairs
        .iter_mut()
        .try_for_each(|key_pair| match key_pair {
          KeyPair::MultiKeyPair(vault) => vault.unlock(password.as_bytes()),
        })?,
    )
  }

  /// Backup the keychain
  pub fn backup(&mut self, password: &str) -> Result<Vec<u8>, KeychainError> {
    if self.vault.is_unlocked() {
      self.lock(password)?;
      let bytes = self.vault.to_bytes()?;
      self.unlock(password)?;

      return Ok(bytes);
    }

    Ok(self.vault.to_bytes()?)
  }

  /// Restore a keychain from a backup
  pub fn restore(bytes: Vec<u8>, password: &str) -> Result<Self, KeychainError> {
    let mut keychain = Keychain {
      vault: Vault::try_from(bytes)?,
      store: Observable::new(KeychainState { accounts: vec![] }),
    };

    keychain.unlock(password)?;

    Ok(keychain)
  }
}

impl Controller<KeychainState, KeychainError> for Keychain {
  /// Get the state of the keychain
  fn get_state(&self) -> &KeychainState {
    self.store.get_state()
  }

  /// Update the state of the keychain
  fn update<F>(&mut self, updater: F) -> Result<(), KeychainError>
  where
    F: Fn(&mut KeychainState),
  {
    Ok(self.store.update(updater)?)
  }

  /// Subscribe to state changes
  fn subscribe<F>(&mut self, subscriber: F) -> usize
  where
    F: 'static + FnMut(&KeychainState),
  {
    self.store.subscribe(subscriber)
  }

  /// Unsubscribe from state changes
  fn unsubscribe(&mut self, id: usize) {
    self.store.unsubscribe(id)
  }
}
