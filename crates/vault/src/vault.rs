use std::fmt::{Debug, Formatter};

use identity::{Account, GenericIdentity, IdentityError, Initializable, MultiKeyPair};
use safe::{EncryptionKey, Safe};

use crate::VaultError;

/// A `Vault` is a safe wrapper around a Hierarchical Deterministic (HD) wallet
/// backed by a mnemonic phrase. It can generate new keys and sign transactions.
///
/// When locked, the mnemonic phrase is encrypted safely and the keys are removed from memory.
/// When unlocked, the mnemonic phrase is decrypted and the keys are recreated in memory.
pub struct Vault<T> {
  /// The identity inside the vault.
  /// Available in-memory only when the vault is unlocked.
  identity: Option<T>,
  /// An encrypted wrapper around the vault.
  /// Available in-memory only when the vault is locked.
  /// The safe holds the encryption salt as plaintext metadata
  safe: Option<Safe<[u8; 16]>>,
}

impl<T> Vault<T> {
  /// Create a new vault with a new random seed and no keys
  pub fn new<F, A>(factory: F, args: A) -> Result<Self, VaultError>
  where
    F: FnOnce(A) -> Result<T, Box<dyn IdentityError>>,
  {
    let identity = match factory(args) {
      Ok(identity) => identity,
      Err(err) => return Err(VaultError::IdentityError(err)),
    };

    Ok(Vault {
      identity: Some(identity),
      safe: None,
    })
  }

  /// Check if the vault is locked
  pub fn is_unlocked(&self) -> bool {
    self.safe.is_none()
  }

  pub fn get_identity(&self) -> Result<&T, VaultError> {
    match &self.identity {
      Some(identity) => Ok(identity),
      None => Err(VaultError::ForbiddenWhileLocked),
    }
  }

  pub fn get_identity_mut(&mut self) -> Result<&mut T, VaultError> {
    match &mut self.identity {
      Some(identity) => Ok(identity),
      None => Err(VaultError::ForbiddenWhileLocked),
    }
  }

  /// Serializes the vault to bytes if it is locked
  /// this operation fails when the vault is unlocked
  /// as no safe has been created, and the exported bytes would
  /// be unencrypted.
  pub fn to_bytes(&self) -> Result<Vec<u8>, VaultError> {
    match &self.safe {
      Some(safe) => Ok(safe.clone().into()),
      None => Err(VaultError::ForbiddenWhileUnlocked),
    }
  }
}

impl<T: Initializable> Vault<T> {
  /// Lock the vault
  ///
  /// Remove all private keys and the seed from memory
  /// and encrypt the HD wallet, storing an unencrypted count
  /// of the number of keys in the vault, to be able to recreate
  /// the same accounts when unlocking.
  pub fn lock(&mut self, password: &[u8]) -> Result<(), VaultError> {
    match &self.identity {
      Some(identity) => {
        // Create an encryption key from the password
        let encryption_key = EncryptionKey::new(password, 1000);
        // A safe is created with the encryption salt as metadata, and
        // the identity as encrypted data bytes
        self.safe = Some(
          Safe::from_plain_bytes(
            encryption_key.salt,
            &encryption_key.pubk,
            identity.serialize(),
          )
          .or(Err(VaultError::SafeCreation))?,
        );
        // The `identity` is removed from memory
        self.identity = None;

        Ok(())
      }
      None => Ok(()),
    }
  }

  /// Unlock the vault
  pub fn unlock(&mut self, password: &[u8]) -> Result<(), VaultError> {
    match &self.safe {
      Some(safe) => {
        // The encryption key is recreated from the password and the salt
        let encryption_key = EncryptionKey::with_salt(password, safe.metadata, 1000);
        // The seed is decrypted from the safe
        let recovered_seed = safe
          .decrypt(&encryption_key.pubk)
          .or(Err(VaultError::SafeDecrypt))?;
        // The identity is recreated from bytes
        let mut identity = T::new();
        identity.deserialize(recovered_seed.as_slice())?;
        // The safe is removed from memory
        self.safe = None;
        // The HD wallet is stored in memory
        self.identity = Some(identity);

        Ok(())
      }
      None => Err(VaultError::AlreadyUnlocked),
    }
  }
}

impl<T: GenericIdentity + MultiKeyPair<[u8; 32], [u8; 32], usize>> Vault<T> {
  /// Add a new key to the vault
  /// Returns the key
  pub fn add_key(&mut self, path: usize) -> Result<Account<usize>, VaultError> {
    let identity = self.get_identity()?;
    let private_key = identity
      .private_key_at(path)
      .or(Err(VaultError::KeyDerivation))?;

    Ok(Account::from_private_key(private_key, path)?)
  }

  /// Signs a message with one of the vault accounts.
  /// The message can be a byte slice, it will be digested internally
  /// by the function.
  pub fn sign(&self, account: &Account<usize>, message: &[u8]) -> Result<Vec<u8>, VaultError> {
    let identity = self
      .get_identity()
      .or(Err(VaultError::ForbiddenWhileLocked))?;

    Ok(identity.sign(account, message)?)
  }
}

impl<T: GenericIdentity + PartialEq> PartialEq for Vault<T> {
  fn eq(&self, other: &Self) -> bool {
    self.identity == other.identity && self.safe == other.safe
  }
}

impl<T> TryFrom<Vec<u8>> for Vault<T> {
  type Error = VaultError;

  fn try_from(bytes: Vec<u8>) -> Result<Self, VaultError> {
    Ok(Self {
      identity: None,
      safe: Some(Safe::try_from(bytes)?),
    })
  }
}

impl<T> Debug for Vault<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Vault").field("safe", &self.safe).finish()
  }
}
