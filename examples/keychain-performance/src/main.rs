use std::time::{Duration, Instant};

use walleth::{Keychain, Signable};

fn main() {
  // Test Keychain creation time
  let keychain_creation = Instant::now();
  for _ in 0..10 {
    Keychain::new();
  }
  println!(
    "Keychain average creation time: {}ms",
    keychain_creation.elapsed().as_millis() / 10
  );

  // Test keychain lock/unlock time
  let mut keychain = Keychain::new();
  keychain.add_account().unwrap();
  let keychain_lock = Instant::now();
  for _ in 0..10 {
    keychain.lock("password").unwrap();
    keychain.unlock("password").unwrap();
  }
  println!(
    "Keychain average lock/unlock time: {}ms",
    keychain_lock.elapsed().as_millis() / 10
  );

  // Test 1 million signatures
  let account = keychain.add_account().unwrap();
  let message = Signable::from_bytes(b"Hello world!");
  let account_creation = Instant::now();
  for _ in 0..1_000 {
    let _signature = keychain
      .use_signer(account.address.clone(), |signer| signer.sign(&message))
      .unwrap();
  }
  println!(
    "Signing average time: {}ms",
    account_creation.elapsed().as_millis() / 1000
  );

  // Test backup / restore
  let keychain_backup = Instant::now();
  for _ in 0..10 {
    let backup = keychain.backup("password").unwrap();
    let _restored_keychain = Keychain::restore(backup, "password").unwrap();
  }
}
