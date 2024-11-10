use std::collections::HashMap;
use std::time::UNIX_EPOCH;
use rusqlite::{params, Connection, Result};
use crate::models::account_model::Account;

#[derive(Debug)]
pub struct Database {
	accounts: HashMap<i32, Account>,
	connection: Connection,
}

impl Database {
	pub fn new(database_path: &str) -> Result<Self> {
		let connection = Connection::open(&database_path)?;
		Ok(Self {
			accounts: HashMap::new(),
			connection,
		})
	}

	pub fn add_account(&mut self, account: Account) -> Result<()> {
		self.connection.execute(
			"INSERT INTO accounts (id, account_name, password, email, notes, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
			params![
                account.id,
                account.account_name,
                account.password,
                account.email,
				account.notes,
                account.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            ],
		)?;
		self.accounts.insert(account.id, account);
		Ok(())
	}

	pub fn remove_account(&mut self, id: i32) -> Option<Account> {
		self.connection.execute("DELETE FROM accounts WHERE id = ?1", params![id]).ok()?;
		self.accounts.remove(&id)
	}

	pub fn get_account(&self, id: i32) -> Option<&Account> {
		self.accounts.get(&id)
	}

	pub fn list_accounts(&self) -> Vec<&Account> {
		self.accounts.values().collect()
	}

	// pub fn load_accounts(&mut self) -> Result<()> {
	// 	let mut stmt = self.connection.prepare("SELECT id, account_name, password, email, notes, created_at FROM accounts")?;
	// 	let account_iter = stmt.query_map([], |row| {
	// 		Ok(Account {
	// 			id: row.get(0)?,
	// 			account_name: row.get(1)?,
	// 			password: row.get(2)?,
	// 			email: row.get(3).ok(),
	// 			notes: row.get(4).ok(),
	// 			created_at: UNIX_EPOCH + std::time::Duration::from_secs(row.get::<_, i64>(5)? as u64),
	// 		})
	// 	})?;
	//
	// 	for account in account_iter {
	// 		let account = account?;
	// 		self.accounts.insert(account.id, account);
	// 	}
	//
	// 	Ok(())
	// }

	pub fn update_account(&mut self, id: i32, updated_account: Account) -> Result<()> {
		if let Some(account) = self.accounts.get_mut(&id) {
			*account = updated_account.clone();
			self.connection.execute(
				"UPDATE accounts SET account_name = ?1, password = ?2, email = ?3, notes = ?4, created_at = ?5 WHERE id = ?6",
				params![
					updated_account.account_name,
					updated_account.password,
					updated_account.email,
					updated_account.notes,
					updated_account.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
					updated_account.id,
				],
			)?;
		}
		Ok(())
	}
}