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

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
	use pgx::prelude::*;

	#[pg_test]
	#[should_panic(expected = "Invalid variant. Supported variants are 'rfc4648' and 'crockford'.")]
	fn test_encode_invalid_variant() {
		crate::encode("foobar", "invalid", false);
	}

	#[pg_test]
	fn test_encode_rfc4648() {
		// RFC4648 with padding
		assert_eq!("MY======", crate::encode("f", "rfc4648", true));
		assert_eq!("MZXQ====", crate::encode("fo", "rfc4648", true));
		assert_eq!("MZXW6===", crate::encode("foo", "rfc4648", true));
		assert_eq!("MZXW6YQ=", crate::encode("foob", "rfc4648", true));
		assert_eq!("MZXW6YTB", crate::encode("fooba", "rfc4648", true));
		assert_eq!("MZXW6YTBOI======", crate::encode("foobar", "rfc4648", true));

		// RFC4648 without padding
		assert_eq!("MY", crate::encode("f", "rfc4648", false));
		assert_eq!("MZXQ", crate::encode("fo", "rfc4648", false));
		assert_eq!("MZXW6", crate::encode("foo", "rfc4648", false));
		assert_eq!("MZXW6YQ", crate::encode("foob", "rfc4648", false));
		assert_eq!("MZXW6YTB", crate::encode("fooba", "rfc4648", false));
		assert_eq!("MZXW6YTBOI", crate::encode("foobar", "rfc4648", false));
	}

	#[pg_test]
	fn test_encode_crockford() {
		// Crockford (padding is ignored)
		assert_eq!("CR", crate::encode("f", "crockford", false));
		assert_eq!("CSQG", crate::encode("fo", "crockford", false));
		assert_eq!("CSQPY", crate::encode("foo", "crockford", false));
		assert_eq!("CSQPYRK1E8", crate::encode("foobar", "crockford", false));

		assert_eq!("CR", crate::encode("f", "crockford", true));
		assert_eq!("CSQG", crate::encode("fo", "crockford", true));
		assert_eq!("CSQPY", crate::encode("foo", "crockford", true));
		assert_eq!("CSQPYRK1E8", crate::encode("foobar", "crockford", true));
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
