use super::KeychainError;
use hdkey::HDKey;
use identity::{Account, IdentityError, Initializable, MultiKeyPair};
use utils::{Controller, Observable};
use vault::{Vault, VaultError};

#[derive(Debug)]
pub enum KeyPair<M = HDKey>
where
  M: MultiKeyPair<[u8; 32], [u8; 33], usize>,
{
  MultiKeyPair(Vault<M>),
}

#[derive(Clone, Debug)]
pub struct KeychainState {
  /// The accounts in the keychain
  /// This is a list of public accounts
  pub accounts: Vec<Account<usize>>,
}

/// A `Keychain` is a collection of keyparis with different capabilities.
/// Each keypair is stored in a `Vault`, which provides basic encryption features
/// and serialization / deserialization from bytes.
#[derive(Debug)]
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

  /// Add an existing keypair to the keychain
  pub fn add_key_pair(&mut self, key_pair: KeyPair<M>) {
    self.key_pairs.push(key_pair);
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

  /// Backup the `Keychain` serializing all the keypairs to bytes and encrypting them
  pub fn backup(&mut self, password: &str) -> Result<Vec<u8>, KeychainError>
  where
    M: Initializable,
  {
    let mut bytes_matrix = self
      .key_pairs
      .iter_mut()
      .map(|key_pair| match key_pair {
        KeyPair::MultiKeyPair(vault) => {
          if vault.is_unlocked() {
            vault.lock(password.as_bytes())?;
            let bytes = vault.to_bytes()?;
            vault.unlock(password.as_bytes())?;
            // 0u8 is a byte representation of a MultiKeyPair
            return Ok((0u8, bytes));
          }

          // 0u8 is a byte representation of a MultiKeyPair
          Ok((0u8, vault.to_bytes()?))
        }
      })
      .collect::<Result<Vec<(u8, Vec<u8>)>, VaultError>>()?;

    let mut condensed: Vec<u8> = vec![];
    bytes_matrix
      .iter_mut()
      .try_for_each(|(vault_type, bytes)| {
        let length = u8::try_from(bytes.len()).or(Err(KeychainError::ByteSerializationError))?;
        // The length of the bytes is prepended to the type of vault
        condensed.append(&mut [length].to_vec());
        // The type of vault is prepended to the bytes
        condensed.append(&mut [*vault_type].to_vec());
        condensed.append(bytes);
        Ok::<(), KeychainError>(())
      })?;

    Ok(condensed)
  }

  /// Restore a `Keychain` from a backup
  pub fn restore(backup: Vec<u8>, password: &str) -> Result<Self, KeychainError>
  where
    M: Initializable,
  {
    let mut keychain = Keychain::<M> {
      key_pairs: vec![],
      store: Observable::new(KeychainState { accounts: vec![] }),
    };
    // Loop through the bytes and deserialize the vaults
    let mut bytes = backup.clone();
    while !bytes.is_empty() {
      // Each vault has a byte to represent the size
      let length = usize::try_from(bytes[0]).or(Err(KeychainError::ByteDeserializationError(
        "Error casting bytes vector length to usize".to_string(),
      )))?;
      // And one to represent its type
      let key_pair_type = bytes[1];

      match key_pair_type {
        0u8 => {
          let key_pair_bytes = bytes[2..(length + 2)].to_vec();
          let key_pair = KeyPair::MultiKeyPair(Vault::<M>::try_from(key_pair_bytes)?);

          keychain.add_key_pair(key_pair);
        }
        unsupported => {
          return Err(KeychainError::ByteDeserializationError(format!(
            "Unsupported key pair type: {}",
            unsupported
          )))
        }
      }

      bytes = bytes[(length + 2)..].to_vec();
    }

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

impl PartialEq for KeyPair {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (KeyPair::MultiKeyPair(vault), KeyPair::MultiKeyPair(other_vault)) => vault == other_vault,
    }
  }
}

impl PartialEq for Keychain {
  fn eq(&self, other: &Self) -> bool {
    self.key_pairs == other.key_pairs
  }
}
