use walleth::Signable;

const MESSAGE_DIGEST: &str = "c0535e4be2b79ffd93291305436bf889314e4a3faec05ecffcbb7df31ad9e51a";

mod new {
	use super::*;

	#[test]
	fn it_creates_a_new_signable() {
		let signable = Signable::new(b"Hello world!");
		assert_eq!(
			signable.to_signable_message().to_string(),
			MESSAGE_DIGEST.to_string()
		);
	}
}

mod from_str {
	use super::*;

	#[test]
	fn it_creates_a_new_signable() {
		let signable = Signable::from_str("Hello world!");
		assert_eq!(
			signable.to_signable_message().to_string(),
			MESSAGE_DIGEST.to_string()
		);
	}
}

mod from_bytes {
	use super::*;

	#[test]
	fn it_creates_a_new_signable() {
		let signable = Signable::from_bytes(b"Hello world!");
		assert_eq!(
			signable.to_signable_message().to_string(),
			MESSAGE_DIGEST.to_string()
		);
	}
}
