use walleth::{Keychain, hdkey_factory};

pub fn main() {
  let mut keychain = Keychain::new();
  println!("keychain: {:?}", keychain);
  keychain.add_multi_keypair(hdkey_factory, None).unwrap();
  println!("keychain: {:?}", keychain);
  keychain.add_multi_keypair(hdkey_factory, None).unwrap();
  println!("keychain: {:?}", keychain);
  keychain.add_multi_keypair(hdkey_factory, None).unwrap();
  println!("keychain: {:?}", keychain);
  let backup = keychain.backup("password").unwrap();
  println!("backup: {:?}", backup);

  let recovered = Keychain::restore(backup, "password").unwrap();

  assert_eq!(recovered, keychain);
}
