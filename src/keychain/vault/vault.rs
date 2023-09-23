use bip32::XPrv;

use crate::{signer::Signer, Account, EncryptionKey, HDWallet, Safe, VaultError};

/// A `Vault` is a safe wrapper around a Hierarchical Deterministic (HD) wallet
/// backed by a mnemonic phrase. It can generate new keys and sign transactions.
///
/// When locked, the mnemonic phrase is encrypted safely and the keys are removed from memory.
/// When unlocked, the mnemonic phrase is decrypted and the keys are recreated in memory.
///
/// # Example
///
/// ```
/// use walleth::Vault;
/// use walleth::Signable;
///
/// // Create a new vault
/// let mut vault = Vault::new();
///
/// // Generate new private key from the HD wallet in the vault
/// vault.add_key().unwrap();
/// vault.add_key().unwrap();
///
/// // Lock the vault
/// vault.lock(b"my secret password").unwrap();
///
/// // Unlock the vault
/// vault.unlock(b"my secret password").unwrap();
///
/// // Use a signer from the vault
/// let signature = vault.use_signer(0, |signer| {
///  signer.sign(&Signable::from_str("Hello world!"))
/// });
///
/// assert!(signature.is_ok());
/// ```
#[derive(Clone)]
pub struct Vault {
  /// The HD wallet of the vault.
  /// Available in-memory only when the vault is unlocked.
  hdwallet: Option<HDWallet>,
  /// The private keys of the vault.
  /// Empty when the vault is locked.
  private_keys: Vec<XPrv>,
  /// An encrypted wrapper around the vault.
  /// Available in-memory only when the vault is locked.
  /// The safe holds the number of keys in the vault and
  /// the encryption salt as plaintext metadata
  safe: Option<Safe<([u8; 16], usize)>>,
}

impl Vault {
  /// Create a new vault with a new random seed and no keys
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::Vault;
  ///
  /// let vault = Vault::new();
  /// ```
  pub fn new() -> Self {
    Vault {
      hdwallet: Some(HDWallet::new()),
      private_keys: vec![],
      safe: None,
    }
  }

  /// Create a new vault from a mnemonic phrase
  /// and no keys
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::Vault;
  ///
  /// let vault = Vault::from_phrase("oak ethics setup flat gesture security must leader people boring donkey one".to_string());
  /// ```
  pub fn from_phrase(phrase: String) -> Result<Self, VaultError> {
    Ok(Vault {
      hdwallet: Some(
        HDWallet::from_mnemonic_str(phrase.as_str()).or(Err(VaultError::InvalidMnemonic))?,
      ),
      private_keys: vec![],
      safe: None,
    })
  }

  /// Add a new key to the vault
  /// Returns the key
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::Vault;
  ///
  /// let mut vault = Vault::new();
  /// let key = vault.add_key();
  /// ```
  pub fn add_key(&mut self) -> Result<Account, VaultError> {
    let index = self.private_keys.len();
    let hdwallet = self.get_hdwallet()?;
    let (private_key, public_key) = hdwallet
      .keypair_at_path(0, 0, index)
      .or(Err(VaultError::KeyDerivation))?;

    self.private_keys.push(private_key);

    Ok(Account::from_extended_public_key(&public_key)?)
  }

  /// Use a `Signer` from the vault, capable of signing transactions
  /// Returns the result of the hook
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::{Vault, Signable};
  ///
  /// let mut vault = Vault::new();
  /// let message = Signable::from_str("Hello world!");
  /// vault.add_key().unwrap();
  ///
  /// let signature = vault.use_signer(0, |signer| {
  /// 	signer.sign(&message)
  /// });
  ///
  /// assert!(signature.is_ok());
  /// ```
  pub fn use_signer<T, R>(&self, key_index: usize, mut hook: T) -> Result<R, VaultError>
  where
    T: FnMut(&Signer) -> R,
  {
    let signer = Signer::new(self.private_keys[key_index].to_bytes())?;

    Ok(hook(&signer))
  }

  /// Lock the vault
  ///
  /// Remove all private keys and the seed from memory
  /// and encrypt the HD wallet, storing an unencrypted count
  /// of the number of keys in the vault, to be able to recreate
  /// the same accounts when unlocking.
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::Vault;
  ///
  /// let mut vault = Vault::new();
  ///
  /// vault.lock(b"my secret password");
  /// ```
  pub fn lock(&mut self, password: &[u8]) -> Result<(), VaultError> {
    match &self.hdwallet {
      Some(hdwallet) => {
        // Create an encryption key from the password
        let encryption_key = EncryptionKey::new(password, 1000);
        // A safe is created with the number of keys in the vault
        // and the encryption salt as metadata, and
        // the HD wallet as encrypted data bytes
        self.safe = Some(
          Safe::from_plain_bytes(
            (encryption_key.salt, self.private_keys.len()),
            &encryption_key.pubk,
            hdwallet.to_bytes(),
          )
          .or(Err(VaultError::SafeCreation))?,
        );
        // The HD wallet is removed from memory
        self.hdwallet = None;
        // The private keys are removed from memory
        self.private_keys = vec![];

        Ok(())
      }
      None => Ok(()),
    }
  }

  /// Unlock the vault
  ///
  /// Recreate the HD wallet from the seed,
  /// recreate the private keys from the HD wallet,
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::Vault;
  ///
  /// let mut vault = Vault::new();
  ///
  /// let account = vault.add_key().unwrap();
  ///
  /// vault.lock(b"my secret password").unwrap();
  /// let recovered_accounts = vault.unlock(b"my secret password").unwrap();
  ///
  /// assert_eq!(recovered_accounts.len(), 1);
  /// assert_eq!(account.address, recovered_accounts[0].address);
  /// ```
  pub fn unlock(&mut self, password: &[u8]) -> Result<Vec<Account>, VaultError> {
    match &self.safe {
      Some(safe) => {
        // The encryption key is recreated from the password and the salt
        let encryption_key = EncryptionKey::with_salt(password, safe.metadata.0, 1000);
        // The seed is decrypted from the safe
        let recovered_seed = safe
          .decrypt(&encryption_key.pubk)
          .or(Err(VaultError::SafeDecrypt))?;
        // The HD wallet is recreated from the seed
        let hdwallet =
          HDWallet::from_bytes(&recovered_seed).or(Err(VaultError::InvalidMnemonic))?;
        // The number of keys in the vault is retrieved from the safe
        // metadata and private keys are recreated from the HD wallet
        self.private_keys = (0..safe.metadata.1)
          .map(|index| hdwallet.private_key_at_path(0, 0, index))
          .collect::<Result<Vec<XPrv>, String>>()
          .or(Err(VaultError::KeyDerivation))?;
        // The safe is removed from memory
        self.safe = None;
        // The HD wallet is stored in memory
        self.hdwallet = Some(hdwallet);

        Ok(
          self
            .private_keys
            .iter()
            .map(|key| {
              Ok(
                Account::from_extended_public_key(&key.public_key())
                  .or(Err(VaultError::AccountCreation))?,
              )
            })
            .collect::<Result<Vec<Account>, VaultError>>()?,
        )
      }
      None => Err(VaultError::AlreadyUnlocked),
    }
  }

  /// Get the HD wallet of the vault
  fn get_hdwallet(&mut self) -> Result<&mut HDWallet, VaultError> {
    match &mut self.hdwallet {
      Some(hdwallet) => Ok(hdwallet),
      None => Err(VaultError::ForbiddenWhileLocked),
    }
  }
}
