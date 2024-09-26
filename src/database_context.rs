use rusqlite::types::Value;
use rusqlite::{params, Connection, Result, Row};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::encryption_controller::{encrypt_database, load_encrypted_db};
use crate::file_accesssor::{read_db_from_disk, write_db_to_disk};

#[derive(Clone)]
pub struct Account {
	pub id: i32,
	pub account_name: String,
	pub password: String,
	pub email: Option<String>,
	pub created_at: SystemTime,
	pub updated_at: SystemTime,
}

pub enum DatabaseState {
	Locked,
	Unlocked(DatabaseContext),
}

pub struct DatabaseManager {
	state: DatabaseState,
	passkey: [u8; 32],
}

impl DatabaseManager {
	pub fn new() -> Self {
		DatabaseManager {
			state: DatabaseState::Locked,
			passkey: [0; 32],
		}
	}

	pub fn unlock(&mut self, passkey: &[u8; 32]) {
		let encrypted_database = read_db_from_disk().expect("Failed to read db from disk");
		let db_content = load_encrypted_db(encrypted_database, passkey).expect("Failed to load encrypted db");
		let context = DatabaseContext::restore_db(db_content).expect("Failed to restore db");
		self.passkey = passkey.clone();
		self.state = DatabaseState::Unlocked(context);
	}

	pub fn safe_database(&self) {
		let context = match &self.state {
			DatabaseState::Locked => return,
			DatabaseState::Unlocked(context) => context,
		};
		let encrypted_db = encrypt_database(context, &self.passkey).expect("Failed to encrypt db");
		write_db_to_disk(encrypted_db);
	}

	pub fn get_database_context(&self) -> Option<&DatabaseContext> {
		match &self.state {
			DatabaseState::Locked => None,
			DatabaseState::Unlocked(context) => Some(context),
		}
	}
}


pub struct DatabaseContext {
	pub(crate) conn: Connection,
}

impl DatabaseContext {
	pub fn new() -> Result<Self> {
		let conn = Connection::open_in_memory()?;

		conn.execute(
			"CREATE TABLE IF NOT EXISTS accounts (\
				id              INTEGER PRIMARY KEY AUTOINCREMENT,
				account_name    TEXT NOT NULL,
				password        TEXT NOT NULL,
				email           TEXT,
				created_at      INTEGER DEFAULT CURRENT_TIMESTAMP,
				updated_at      INTEGER DEFAULT CURRENT_TIMESTAMP
			)", [],
		)?;

		Ok(DatabaseContext { conn })
	}

	pub fn restore_db(plain_data: Vec<u8>) -> Result<Self> {
		let conn = Connection::open_in_memory()?;
		let sql_dump = String::from_utf8(plain_data).expect("Failed to parse db dump");
		match conn.execute_batch(&sql_dump) {
			Ok(()) => Ok(DatabaseContext { conn }),
			Err(e) => panic!("Failed to restore db: {}, dump => \n{}", e.to_string(), sql_dump)
		}
	}

	pub fn dump_db(&self) -> Result<String> {
		let mut dump = String::new();
		let mut stmt = self.conn.prepare("SELECT sql FROM sqlite_master WHERE type='table';")?;
		let mut rows = stmt.query([])?;

		while let Some(row) = rows.next()? {
			let sql: String = row.get(0)?;

			if sql.contains("sqlite_sequence") {
				continue;
			}

			dump.push_str(&sql);
			dump.push(';');
		}

		let mut table_stmt = self.conn.prepare("SELECT name FROM sqlite_master WHERE type='table';")?;
		let mut table_rows = table_stmt.query([])?;

		while let Some(row) = table_rows.next()? {
			let table_name: String = row.get(0)?;

			if table_name == "sqlite_sequence" {
				continue;
			}

			let mut data_stmt = self.conn.prepare(&format!("SELECT * FROM {}", table_name))?;
			let mut data_rows = data_stmt.query([])?;

			while let Some(data_row) = data_rows.next()? {
				let column_count = 6;
				let mut values = Vec::new();

				for i in 0..column_count {
					let value: Value = data_row.get(i)?;
					values.push(match value {
						Value::Null => "NULL".to_string(),
						Value::Integer(val) => val.to_string(),
						Value::Real(val) => val.to_string(),
						Value::Text(val) => format!("'{}'", val.replace("'", "''")),
						Value::Blob(_) => {
							"X''".to_string()
						}
					});
				}
				let insert_statement = format!("INSERT INTO {} VALUES ({});", table_name, values.join(", "));
				dump.push_str(&insert_statement);
			}
		}

		Ok(dump)
	}


	pub fn add_account(&self, account_name: &str, password: &str, email: Option<String>) -> Result<()> {
		let current_time = system_time_to_timestamp(SystemTime::now());
		match self.conn.execute(
			"INSERT INTO accounts (account_name, password, email, created_at, updated_at)\
			VALUES (?1, ?2, ?3, ?4, ?5)",
			params![account_name, password, email, current_time, current_time],
		) {
			Ok(_) => Ok(()),
			Err(e) => panic!("SQL Query error: {}", e.to_string())
		}
	}

	pub fn get_account_by_id(&self, id: i32) -> Result<Option<Account>> {
		let mut stmt = self.conn.prepare(
			"SELECT id, account_name, password, email, created_at, updated_at FROM accounts WHERE id = ?1",
		)?;

		let mut account_iter = stmt.query_map(params![id], |row| {
			self.create_account_from_row(row)
		})?;

		if let Some(account) = account_iter.next() {
			return Ok(Some(account?));
		}

		Ok(None)
	}

	pub fn update_account(&self, account: &Account) {
		let current_time = system_time_to_timestamp(SystemTime::now());
		if self.conn.execute(
			"UPDATE accounts SET \
				account_name = ?1,\
				password = ?2,\
				email = ?3,\
				updated_at = ?4\
				WHERE id = ?5",
			params![account.account_name, account.password, account.email, current_time, account.id],
		).is_err() {
			panic!("Updating account failed!");
		}
	}

	pub fn remove_account(&self, id: i32) -> Result<()> {
		self.conn.execute("DELETE FROM accounts WHERE id = ?1", params![id])?;
		Ok(())
	}

	pub fn search_accounts_by_name(&self, name_part: &str) -> Result<Vec<Account>> {
		let mut stmt = self.conn.prepare(
			"SELECT id, account_name, password, email, created_at, updated_at FROM accounts WHERE account_name LIKE ?1",
		)?;

		let account_iter = stmt.query_map(params![format!("%{}%",name_part)], |row| {
			self.create_account_from_row(row)
		})?;

		let mut accounts = Vec::new();
		for account in account_iter {
			accounts.push(account?);
		}
		Ok(accounts)
	}


	pub fn list_all_accounts(&self) -> Result<Vec<Account>> {
		let mut stmt = self.conn.prepare(
			"SELECT id, account_name, password, email, created_at, updated_at FROM accounts ",
		)?;

		let account_iter = stmt.query_map([], |row| {
			self.create_account_from_row(row)
		})?;

		let mut accounts = Vec::new();
		for account in account_iter {
			accounts.push(account?);
		}
		Ok(accounts)
	}

	fn create_account_from_row(&self, row: &Row) -> Result<Account> {
		Ok(Account {
			id: row.get(0)?,
			account_name: row.get(1)?,
			password: row.get(2)?,
			email: row.get(3)?,
			created_at: timestamp_to_system_time(row.get(4)?),
			updated_at: timestamp_to_system_time(row.get(5)?),
		})
	}
}

fn system_time_to_timestamp(time: SystemTime) -> i64 {
	time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

fn timestamp_to_system_time(timestamp: i64) -> SystemTime {
	UNIX_EPOCH + Duration::from_secs(timestamp as u64)
}