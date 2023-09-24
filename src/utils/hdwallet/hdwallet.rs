use bip32::{XPrv, XPub};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};

use crate::{generate_seed_bytes, get_derivation_path, parse_mnemonic};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HDWallet {
  seed: Vec<u8>,
}

impl HDWallet {
  /// Create a new `HDWallet` from a random seed
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::HDWallet;
  ///
  /// let hdwallet = HDWallet::new();
  /// ```
  pub fn new() -> Self {
    HDWallet {
      seed: generate_seed_bytes(),
    }
  }

  /// Create a new `HDWallet` from a seed as slice of bytes
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::HDWallet;
  ///
  /// let hdwallet = HDWallet::from_bytes(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
  /// ```
  pub fn from_bytes(seed: &[u8]) -> Result<Self, String> {
    Ok(HDWallet { seed: seed.into() })
  }

  /// Create a new `HDWallet` from a mnemonic phrase
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::HDWallet;
  ///
  /// let hdwallet = HDWallet::from_mnemonic_str("grocery belt target explain clay essay focus spatial skull brain measure matrix toward visual protect owner stone scale slim ghost panda exact combine game").unwrap();
  /// ```
  pub fn from_mnemonic_str(mnemonic: &str) -> Result<Self, String> {
    let seed = parse_mnemonic(mnemonic.to_string())?.to_seed("");

    Ok(HDWallet {
      seed: seed.as_bytes().to_vec(),
    })
  }

  /// Get the private key at a derivation path
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::HDWallet;
  ///
  /// let hdwallet = HDWallet::from_mnemonic_str("grocery belt target explain clay essay focus spatial skull brain measure matrix toward visual protect owner stone scale slim ghost panda exact combine game").unwrap();
  /// let private_key = hdwallet.private_key_at_path(0, 0, 0);
  /// ```
  pub fn private_key_at_path(
    &self,
    account: usize,
    change: usize,
    index: usize,
  ) -> Result<XPrv, String> {
    match XPrv::derive_from_path(&self.seed, &get_derivation_path(account, change, index)?) {
      Ok(private_key) => Ok(private_key),
      Err(e) => Err(e.to_string()),
    }
  }

  /// Get the public key at a derivation path
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::HDWallet;
  ///
  /// let hdwallet = HDWallet::from_mnemonic_str("grocery belt target explain clay essay focus spatial skull brain measure matrix toward visual protect owner stone scale slim ghost panda exact combine game").unwrap();
  /// let public_key = hdwallet.public_key_at_path(0, 0, 0);
  /// ```
  pub fn public_key_at_path(
    &self,
    account: usize,
    change: usize,
    index: usize,
  ) -> Result<XPub, String> {
    match XPrv::derive_from_path(&self.seed, &get_derivation_path(account, change, index)?) {
      Ok(private_key) => Ok(private_key.public_key()),
      Err(e) => Err(e.to_string()),
    }
  }

  /// Get the keypair at a derivation path
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::HDWallet;
  ///
  /// let hdwallet = HDWallet::from_mnemonic_str("grocery belt target explain clay essay focus spatial skull brain measure matrix toward visual protect owner stone scale slim ghost panda exact combine game").unwrap();
  /// let (private_key, public_key) = hdwallet.keypair_at_path(0, 0, 0).unwrap();
  /// ```
  pub fn keypair_at_path(
    &self,
    account: usize,
    change: usize,
    index: usize,
  ) -> Result<(SecretKey, PublicKey), String> {
    let secp = Secp256k1::new();
    let derived_pvk =
      XPrv::derive_from_path(&self.seed, &get_derivation_path(account, change, index)?)
        .or(Err("Invalid derivation path"))?;

    let private_key = SecretKey::from_slice(&derived_pvk.private_key().to_bytes())
      .or(Err("Invalid private key"))?;

    let public_key = private_key.public_key(&secp);

    Ok((private_key, public_key))
  }

  /// Get the seed as a slice of bytes
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::HDWallet;
  ///
  /// let hdwallet = HDWallet::from_mnemonic_str("grocery belt target explain clay essay focus spatial skull brain measure matrix toward visual protect owner stone scale slim ghost panda exact combine game").unwrap();
  /// let seed_bytes = hdwallet.to_bytes();
  /// ```
  pub fn to_bytes(&self) -> &[u8] {
    &self.seed
  }
}

impl PartialEq for HDWallet {
  fn eq(&self, other: &Self) -> bool {
    self.seed == other.seed
  }
}
