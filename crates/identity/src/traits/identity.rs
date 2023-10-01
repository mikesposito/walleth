use std::error::Error;

use crate::Account;

pub trait IdentityError: Error + 'static {}

type IdentityResult<T> = Result<T, Box<dyn IdentityError>>;

pub trait GenericIdentity {
  /// Get the identity type
  fn identity_type(&self) -> String;

  /// Serialize the identity into a byte array
  fn serialize(&self) -> Vec<u8>;

  fn deserialize(&mut self, bytes: &[u8]) -> IdentityResult<()>;
}

pub trait Initializable: GenericIdentity {
  fn new() -> Self;
}

pub trait AccountDeriver<P> {
  /// Get an account of the identity
  fn account_at(&self, path: P) -> IdentityResult<Account<P>>;
}

pub trait KeyPair<PK, PB>
where
  PB: Sized,
  PK: Sized,
{
  /// Get the private key
  fn private_key(&self) -> IdentityResult<PK>;

  /// Get the public key
  fn public_key(&self) -> IdentityResult<PB>;

  /// Sign a message with the identity
  fn sign(&self, message: &[u8]) -> IdentityResult<Vec<u8>>;

  /// Verify a signature with the identity
  fn verify(&self, message: &[u8], signature: &[u8]) -> IdentityResult<()>;
}

pub trait MultiKeyPair<PK, PB, P>
where
  Self: GenericIdentity,
  PB: Sized,
  PK: Sized,
{
  /// Get the private key at a derivation path
  fn private_key_at(&self, path: P) -> IdentityResult<PK>;

  /// Get the public key at a derivation path
  fn public_key_at(&self, path: P) -> IdentityResult<PB>;

  /// Sign a message with an account of the identity
  fn sign(&self, from: &Account<P>, message: &[u8]) -> IdentityResult<Vec<u8>>;

  /// Verify a signature with an account of the identity
  fn verify(&self, from: &Account<P>, message: &[u8], signature: &[u8]) -> IdentityResult<()>;
}
