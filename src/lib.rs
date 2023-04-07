use core::panic;

use base32::*;
use pgx::prelude::*;

pgx::pg_module_magic!();

#[pg_extern(immutable, parallel_safe)]
fn encode(
	data: &str,
	variant: default!(&str, "'rfc4648'"),
	padding: default!(bool, false),
) -> String {
	let alphabet = match variant.to_lowercase().as_str() {
		"rfc4648" => Alphabet::RFC4648 { padding },
		"crockford" => Alphabet::Crockford,
		_ => {
			ereport!(
				ERROR,
				PgSqlErrorCode::ERRCODE_DATA_EXCEPTION,
				"Invalid variant. Supported variants are 'rfc4648' and 'crockford'."
			);
		}
	};

	return base32::encode(alphabet, data.as_bytes());
}

#[pg_extern(immutable, parallel_safe)]
fn decode(
	data: &str,
	variant: default!(&str, "'rfc4648'"),
	padding: default!(bool, false),
) -> Vec<u8> {
	let alphabet = match variant.to_lowercase().as_str() {
		"rfc4648" => Alphabet::RFC4648 { padding },
		"crockford" => Alphabet::Crockford,
		_ => {
			ereport!(
				ERROR,
				PgSqlErrorCode::ERRCODE_DATA_EXCEPTION,
				"Invalid variant. Supported variants are 'rfc4648' and 'crockford'."
			);
		}
	};

	let decoded = match base32::decode(alphabet, data) {
		Some(decoded) => decoded,
		None => {
			ereport!(
				ERROR,
				PgSqlErrorCode::ERRCODE_DATA_EXCEPTION,
				"Invalid input. Input must be a base32 string."
			);
		}
	};

	return decoded;
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
	use pgx::prelude::*;

	#[pg_test]
	#[should_panic = "Invalid variant. Supported variants are 'rfc4648' and 'crockford'."]
	fn test_encode_invalid_variant() {
		crate::encode("foobar", "invalid", false);
	}

	#[pg_test]
	fn test_encode_rfc4648_padded() {
		// RFC4648 with padding
		assert_eq!("MY======", crate::encode("f", "rfc4648", true));
		assert_eq!("MZXQ====", crate::encode("fo", "rfc4648", true));
		assert_eq!("MZXW6===", crate::encode("foo", "rfc4648", true));
		assert_eq!("MZXW6YQ=", crate::encode("foob", "rfc4648", true));
		assert_eq!("MZXW6YTB", crate::encode("fooba", "rfc4648", true));
		assert_eq!("MZXW6YTBOI======", crate::encode("foobar", "rfc4648", true));
	}

	#[pg_test]
	fn test_encode_rfc4648_unpadded() {
		// RFC4648 without padding
		assert_eq!("MY", crate::encode("f", "rfc4648", false));
		assert_eq!("MZXQ", crate::encode("fo", "rfc4648", false));
		assert_eq!("MZXW6", crate::encode("foo", "rfc4648", false));
		assert_eq!("MZXW6YQ", crate::encode("foob", "rfc4648", false));
		assert_eq!("MZXW6YTB", crate::encode("fooba", "rfc4648", false));
		assert_eq!("MZXW6YTBOI", crate::encode("foobar", "rfc4648", false));
	}

	#[pg_test]
	fn test_encode_crockford_padded() {
		// Crockford (padding is ignored)
		assert_eq!("CR", crate::encode("f", "crockford", true));
		assert_eq!("CSQG", crate::encode("fo", "crockford", true));
		assert_eq!("CSQPY", crate::encode("foo", "crockford", true));
		assert_eq!("CSQPYRK1E8", crate::encode("foobar", "crockford", true));
	}

	#[pg_test]
	fn test_encode_crockford_unpadded() {
		assert_eq!("CR", crate::encode("f", "crockford", false));
		assert_eq!("CSQG", crate::encode("fo", "crockford", false));
		assert_eq!("CSQPY", crate::encode("foo", "crockford", false));
		assert_eq!("CSQPYRK1E8", crate::encode("foobar", "crockford", false));
	}

	#[pg_test]
	#[should_panic = "Invalid variant. Supported variants are 'rfc4648' and 'crockford'."]
	fn test_decode_invalid_variant() {
		crate::decode("foobar", "invalid", false);
	}

	#[pg_test]
	fn test_decode_rfc4648_padded() {
		// RFC4648 with padding
		assert_eq!(vec![102], crate::decode("MY======", "rfc4648", true));
		assert_eq!(vec![102, 111], crate::decode("MZXQ====", "rfc4648", true));
		assert_eq!(
			vec![102, 111, 111],
			crate::decode("MZXW6===", "rfc4648", true)
		);
		assert_eq!(
			vec![102, 111, 111, 98],
			crate::decode("MZXW6YQ=", "rfc4648", true)
		);
		assert_eq!(
			vec![102, 111, 111, 98, 97],
			crate::decode("MZXW6YTB", "rfc4648", true)
		);
		assert_eq!(
			vec![102, 111, 111, 98, 97, 114],
			crate::decode("MZXW6YTBOI======", "rfc4648", true)
		);
	}

	#[pg_test]
	fn test_decode_rfc4648_unpadded() {
		// RFC4648 without padding
		assert_eq!(vec![102], crate::decode("MY", "rfc4648", false));
		assert_eq!(vec![102, 111], crate::decode("MZXQ", "rfc4648", false));
		assert_eq!(
			vec![102, 111, 111],
			crate::decode("MZXW6", "rfc4648", false)
		);
		assert_eq!(
			vec![102, 111, 111, 98],
			crate::decode("MZXW6YQ", "rfc4648", false)
		);
		assert_eq!(
			vec![102, 111, 111, 98, 97],
			crate::decode("MZXW6YTB", "rfc4648", false)
		);
		assert_eq!(
			vec![102, 111, 111, 98, 97, 114],
			crate::decode("MZXW6YTBOI", "rfc4648", false)
		);
	}

	#[pg_test]
	#[should_panic = "Invalid input. Input must be a base32 string."]
	fn test_decode_invalid_data() {
		crate::decode("not_base32", "rfc4648", false);
	}

	#[pg_test]
	fn test_decode_crockford_padded() {
		// Crockford (padding is ignored)
		assert_eq!(vec![102], crate::decode("CR", "crockford", true));
		assert_eq!(vec![102, 111], crate::decode("CSQG", "crockford", true));
		assert_eq!(
			vec![102, 111, 111],
			crate::decode("CSQPY", "crockford", true)
		);
		assert_eq!(
			vec![102, 111, 111, 98, 97, 114],
			crate::decode("CSQPYRK1E8", "crockford", true)
		);
	}

	#[pg_test]
	fn test_decode_crockford_unpadded() {
		assert_eq!(vec![102], crate::decode("CR", "crockford", false));
		assert_eq!(vec![102, 111], crate::decode("CSQG", "crockford", false));
		assert_eq!(
			vec![102, 111, 111],
			crate::decode("CSQPY", "crockford", false)
		);
		assert_eq!(
			vec![102, 111, 111, 98, 97, 114],
			crate::decode("CSQPYRK1E8", "crockford", false)
		);
	}
}

/// This module is required by `cargo pgx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
	pub fn setup(_options: Vec<&str>) {
		// perform one-off initialization when the pg_test framework starts
	}

	pub fn postgresql_conf_options() -> Vec<&'static str> {
		// return any postgresql.conf settings that are required for your tests
		vec![]
	}
}
