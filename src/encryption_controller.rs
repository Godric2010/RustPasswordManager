use crate::database_context::DatabaseContext;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit, Nonce};
use base64::{engine::general_purpose, Engine as _};
use ring::aead::NONCE_LEN;
use ring::pbkdf2;
use ring::rand::SecureRandom;
use std::num::NonZeroU32;

const CREDENTIAL_LEN: usize = 32;

pub struct PasswordEncryption {
	salt: [u8; 16],
	encrypted_string: [u8; CREDENTIAL_LEN],
}

impl PasswordEncryption {
	pub fn generate_new(string: &str) -> Self {
		let salt = Self::generate_salt();
		let encrypted_string = Self::derive_key_from_string(string, &salt);
		Self {
			salt,
			encrypted_string,
		}
	}

	pub fn create_from_string(string: String) -> std::io::Result<Self> {
		let mut parts = string.split(':');
		let salt_encoded = parts.next().ok_or(std::io::Error::new(
			std::io::ErrorKind::InvalidData,
			"Missing salt!",
		))?;

		let pwd_encoded = parts.next().ok_or(std::io::Error::new(
			std::io::ErrorKind::InvalidData,
			"Missing key!",
		))?;

		let salt_vec = general_purpose::STANDARD.decode(salt_encoded).expect("Failed to decode salt");
		let pwd_bytes = general_purpose::STANDARD.decode(pwd_encoded).expect("Failed to decode key");

		let mut pwd = [0u8; CREDENTIAL_LEN];
		pwd.copy_from_slice(&pwd_bytes);

		let salt: Option<[u8; 16]> = if salt_vec.len() == 16 {
			let mut array = [0u8; 16];
			array.copy_from_slice(&salt_vec);
			Some(array)
		} else { None };


		Ok(Self {
			salt: salt.expect("Salt has not the expected length"),
			encrypted_string: pwd,
		})
	}

	pub fn get_encrypted_string(&self) -> [u8; 32] {
		self.encrypted_string
	}

	pub fn create_string(&self) -> String {
		let salt_encoded = general_purpose::STANDARD.encode(self.salt);
		let pwd_encoded = general_purpose::STANDARD.encode(self.encrypted_string);
		format!("{}:{}", salt_encoded, pwd_encoded)
	}

	pub fn verify_string(&self, string: &str) -> bool {
		pbkdf2::verify(
			pbkdf2::PBKDF2_HMAC_SHA256,
			NonZeroU32::new(100_000).unwrap(),
			&self.salt,
			string.as_bytes(),
			&self.encrypted_string,
		).is_ok()
	}

	fn generate_salt() -> [u8; 16] {
		let mut salt = [0u8; 16];
		ring::rand::SystemRandom::new()
			.fill(&mut salt)
			.expect("Failed to generate salt");
		salt
	}

	fn derive_key_from_string(string: &str, salt: &[u8]) -> [u8; CREDENTIAL_LEN] {
		let mut key = [0u8; CREDENTIAL_LEN];
		pbkdf2::derive(
			pbkdf2::PBKDF2_HMAC_SHA256,
			NonZeroU32::new(100_000).unwrap(),
			salt,
			string.as_bytes(),
			&mut key,
		);
		key
	}
}

pub fn load_encrypted_db(encrypted_db: Vec<u8>, encryption_key: &[u8]) -> std::io::Result<Vec<u8>> {
	let (nonce, ciphertext) = encrypted_db.split_at(NONCE_LEN);

	let aes_key = Key::<Aes256Gcm>::from_slice(encryption_key);
	let cipher = Aes256Gcm::new(aes_key);
	let plaintext_db = cipher.decrypt(Nonce::from_slice(nonce), ciphertext).expect("Decryption of db failed");

	Ok(plaintext_db)
}

pub fn encrypt_database(database_context: &DatabaseContext, encryption_key: &[u8]) -> std::io::Result<Vec<u8>> {
	let aes_key = Key::<Aes256Gcm>::from_slice(encryption_key);
	let cipher = Aes256Gcm::new(aes_key);
	let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

	let plaintext = match database_context.dump_db() {
		Ok(plaintext) => plaintext,
		Err(e) => {
			println!("{}", e);
			panic!("Could not dump db to string!");
		}
	};

	let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref()).expect("Encryption of db failed");
	Ok([nonce.to_vec(), ciphertext].concat())
}