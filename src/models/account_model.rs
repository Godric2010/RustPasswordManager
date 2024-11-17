use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Account{
	pub id: i32,
	pub account_name: String,
	pub password: String,
	pub email: Option<String>,
	pub notes: Option<String>,
	pub created_at: SystemTime,
}

impl Account{
	pub fn new(id: i32, account_name: String, password: String, email: Option<String>, notes: Option<String>) -> Self{
		Self{
			id,
			account_name,
			password,
			email,
			notes,
			created_at: SystemTime::now(),
		}
	}

	pub fn update_password(&mut self, new_password: String){
		self.password = new_password;
	}

	pub fn update_email(&mut self, new_email: String){
		self.email = Some(new_email);
	}

	pub fn update_notes(&mut self, new_notes: String){
		self.notes = Some(new_notes);
	}
}