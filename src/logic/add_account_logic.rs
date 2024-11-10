use crate::models::account_model::Account;
use crate::models::database::Database;

pub struct AddAccountLogic {
	database: Database,
}

impl AddAccountLogic {
	pub fn new(database: Database) -> AddAccountLogic {
		Self { database }
	}
	pub fn validate_account_data(account_name: &str, password: &str) -> Result<bool, String> {
		if account_name.is_empty() || password.is_empty() {
			return Err("Account name or password cannot be empty.".to_string());
		}

		Ok(true)
	}

	pub fn add_to_database(&mut self, account: Account) -> bool {
		match self.database.add_account(account) {
			Ok(_) => true,
			Err(_) => false,
		}
	}
}