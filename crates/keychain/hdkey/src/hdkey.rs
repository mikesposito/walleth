use bip32::XPrv;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

use crate::{
  utils::{generate_seed_bytes, get_derivation_path, parse_mnemonic},
  HDKeyError,
};
use identity::{
  signer::{Signable, Signer},
  Account, AccountDeriver, GenericIdentity, IdentityError, Initializable, MultiKeyPair,
};

#[derive(Clone, Debug)]
pub struct HDKey {
  seed: Vec<u8>,
}

impl HDKey {
  /// Create a new `HDKey` from a mnemonic phrase
  pub fn from_mnemonic_str(mnemonic: &str) -> Result<Self, Box<dyn IdentityError>> {
    let seed = parse_mnemonic(mnemonic.to_string())
      .or(Err(HDKeyError::InvalidMnemonic.into()))?
      .to_seed("");

    Ok(HDKey {
      seed: seed.as_bytes().to_vec(),
    })
  }

  /// Get the keypair at a derivation path
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
  pub fn to_bytes(&self) -> &[u8] {
    &self.seed
  }
}

impl TryFrom<Vec<u8>> for HDKey {
  type Error = HDKeyError;

  /// Create a new `HDKey` from a seed as slice of bytes
  fn try_from(seed: Vec<u8>) -> Result<Self, HDKeyError> {
    Ok(HDKey { seed: seed.into() })
  }
}

impl Into<Vec<u8>> for HDKey {
  /// Get the seed as a slice of bytes
  fn into(self) -> Vec<u8> {
    self.seed.clone()
  }
}

impl From<&[u8]> for HDKey {
  /// Create a new `HDKey` from a seed as slice of bytes
  fn from(seed: &[u8]) -> Self {
    HDKey {
      seed: seed.to_vec(),
    }
  }
}

impl GenericIdentity for HDKey {
  fn identity_type(&self) -> String {
    "HDKey".to_string()
  }

  fn serialize(&self) -> Vec<u8> {
    self.seed.clone()
  }

  fn deserialize(&mut self, bytes: &[u8]) -> Result<(), Box<dyn IdentityError>> {
    self.seed = bytes.to_vec();
    Ok(())
  }
}

impl Initializable for HDKey {
  /// Create a new `HDKey` from a random seed
  fn new() -> Self {
    HDKey {
      seed: generate_seed_bytes(),
    }
  }
}

impl AccountDeriver<usize> for HDKey {
  /// Get an account of the hdkey
  fn account_at(&self, index: usize) -> Result<Account<usize>, Box<dyn IdentityError>> {
    let (_, public_key) = match self.keypair_at_path(0, 0, index) {
      Ok(keypair) => keypair,
      Err(_) => return Err(HDKeyError::WrongDerivationPath.into()),
    };

    match Account::from_public_key(&public_key, index) {
      Ok(account) => Ok(account),
      Err(_) => Err(HDKeyError::WrongDerivationPath.into()),
    }
  }
}

impl MultiKeyPair<[u8; 32], [u8; 33], usize> for HDKey {
  /// Get the private key at a derivation path
  fn private_key_at(&self, index: usize) -> Result<[u8; 32], Box<dyn IdentityError>> {
    let derivation_path = match get_derivation_path(0, 0, index) {
      Ok(derivation_path) => derivation_path,
      Err(_) => return Err(HDKeyError::WrongDerivationPath.into()),
    };

    match XPrv::derive_from_path(&self.seed, &derivation_path) {
      Ok(private_key) => Ok(private_key.to_bytes().into()),
      Err(_) => Err(HDKeyError::WrongDerivationPath.into()),
    }
  }

  /// Get the public key at a derivation path
  fn public_key_at(&self, index: usize) -> Result<[u8; 33], Box<dyn IdentityError>> {
    let derivation_path = match get_derivation_path(0, 0, index) {
      Ok(derivation_path) => derivation_path,
      Err(_) => return Err(HDKeyError::WrongDerivationPath.into()),
    };

    match XPrv::derive_from_path(&self.seed, &derivation_path) {
      Ok(private_key) => Ok(private_key.public_key().to_bytes().into()),
      Err(_) => Err(Box::new(HDKeyError::WrongDerivationPath)),
    }
  }

  /// Sign a message with the hdkey
  fn sign(&self, from: &Account<usize>, message: &[u8]) -> Result<Vec<u8>, Box<dyn IdentityError>> {
    let private_key = self.private_key_at(from.path)?;
    let signer = Signer::new(private_key).or(Err(HDKeyError::InvalidPrivateKey.into()))?;
    let signable = Signable::from_bytes(message);

    let signature = signer.sign(&signable);

    Ok(signature.serialize_der().to_vec())
  }

  /// Verify a signature with the hdkey
  fn verify(
    &self,
    from: &Account<usize>,
    message: &[u8],
    signature: &[u8],
  ) -> Result<(), Box<dyn IdentityError>> {
    let private_key = self.private_key_at(from.path)?;
    let signer = Signer::new(private_key).or(Err(HDKeyError::InvalidPrivateKey.into()))?;

    Ok(
      signer
        .verify(&Signable::from_bytes(message), signature)
        .or(Err(HDKeyError::InvalidSignature.into()))?,
    )
  }
}

impl PartialEq for HDKey {
  fn eq(&self, other: &Self) -> bool {
    self.seed == other.seed
  }
}
