use crate::models::database::Database;
use crate::utilities::encryption_controller::PasswordEncryption;
use crate::utilities::file_accesssor::read_password_from_disk;

pub struct AuthenticationLogic {
	encrypted_database: Option<Database>,
	master_password: PasswordEncryption,
}

impl AuthenticationLogic {
	pub fn new() -> Self {
		let pwd_string = read_password_from_disk();
		let master_password = PasswordEncryption::create_from_string(pwd_string.unwrap()).unwrap();
		Self {
			encrypted_database: None,
			master_password,
		}
	}

	pub fn verify_password(&self, input_password: &str) -> bool {
		self.master_password.verify_string(input_password)
	}

	pub fn load_encrypted_data(database_path: &str) -> Option<Database> {
		Database::new(database_path).ok()
	}
}