use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rusqlite::{params, Connection, Result};

pub struct Account {
	pub id: i32,
	pub account_name: String,
	pub username: String,
	pub password: String,
	pub email: Option<String>,
	pub created_at: SystemTime,
	pub updated_at: SystemTime,
}

pub struct DatabaseContext {
	conn: Connection,
}

impl DatabaseContext {
	pub fn new(database_path: &str) -> Result<Self> {
		let conn = Connection::open(database_path)?;

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

	pub fn add_account(&self, account_name: &str, username: &str, password: &str, email: Option<&str>) -> Result<()> {
		let current_time = system_time_to_timestamp(SystemTime::now());
		self.conn.execute(
			"INSERT INTO accounts (account_name, username, password, email, created_at, updated_at)\
			VALUES (?1, ?2, ?3, ?4, ?5, ?6",
			params![account_name, username, password, email, current_time, current_time],
		)?;
		Ok(())
	}

	pub fn remove_account(&self, id: i32) -> Result<()> {
		self.conn.execute("DELETE FROM account WHERE id = ?1", params![id])?;
		Ok(())
	}

	pub fn search_accounts_by_name(&self, name_part: &str) -> Result<Vec<Account>> {
		let mut stmt = self.conn.prepare(
			"SELECT id, account_name, username, password, email, created_at, updated_at FROM accounts WHERE account_name LIKE ?1",
		)?;

		let account_iter = stmt.query_map(params![format!("%{}%",name_part)], |row| {
			Ok(Account {
				id: row.get(0)?,
				account_name: row.get(1)?,
				username: row.get(2)?,
				password: row.get(3)?,
				email: row.get(4)?,
				created_at: timestamp_to_system_time(row.get(5)?),
				updated_at: timestamp_to_system_time(row.get(6)?),
			})
		})?;

		let mut accounts = Vec::new();
		for account in account_iter {
			accounts.push(account?);
		}
		Ok(accounts)
	}

	pub fn list_all_accounts(&self) -> Result<Vec<Account>> {
		let mut stmt = self.conn.prepare(
			"SELECT id, account_name, username, password, email, created_at, updated_at FROM accounts ",
		)?;

		let account_iter = stmt.query_map([], |row| {
			Ok(Account {
				id: row.get(0)?,
				account_name: row.get(1)?,
				username: row.get(2)?,
				password: row.get(3)?,
				email: row.get(4)?,
				created_at: timestamp_to_system_time(row.get(5)?),
				updated_at: timestamp_to_system_time(row.get(6)?),
			})
		})?;

		let mut accounts = Vec::new();
		for account in account_iter {
			accounts.push(account?);
		}
		Ok(accounts)
	}
}

fn system_time_to_timestamp(time: SystemTime) -> i64 {
	time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

fn timestamp_to_system_time(timestamp: i64) -> SystemTime {
	UNIX_EPOCH + Duration::from_secs(timestamp as u64)
}