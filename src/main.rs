use walleth::{Controller, Keychain, Signable};

pub fn main() {
	// A new keychain is created
	let mut keychain = Keychain::new();

	// Set an example listener on keychain changes
	keychain.subscribe(|keychain_state| {
		println!(
			"> Keychain state changed, accounts: {:?}",
			keychain_state.accounts.len()
		);
	});

	// New identity added
	let account = match keychain.add_account() {
		Ok(key) => key,
		Err(e) => {
			println!("Error: {}", e);
			return;
		}
	};
	println!("> Account: {}", account.address);

	// Lock the keychain with a password
	keychain.lock("Hello Buterin").unwrap();
	println!("> Keychain locked");

	// Unlock the keychain
	keychain.unlock("Hello Buterin").unwrap();
	println!("> Keychain unlocked");

	// Create message to sign
	let message_to_sign = Signable::from_str("Hello").unwrap();

	// Sign message with keychain address
	let signature = keychain
		.use_signer(account.address.clone(), |signer| {
			let signature = signer.sign(&message_to_sign).unwrap();
			// Whatever is returned here is returned from the use_signer hook
			signature
		})
		.unwrap();

	// Print some info of what happened
	println!(
		"> Signed message: {:?}\n> Signature: {}",
		message_to_sign.to_signable_message(),
		signature
	);

	// Try to add an account while locked
	keychain.lock("Hello Buterin").unwrap();

	if keychain.add_account().is_ok() {
		panic!("> Error: We added an account while wallet was locked");
	}
}
