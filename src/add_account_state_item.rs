use crate::database_context::DatabaseManager;
use crate::state_item::{wait_for_seconds, StateItem};
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;
use crossterm::event::KeyCode;
use std::sync::{Arc, Mutex};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::input_handler::*;

#[derive(Eq, PartialEq)]
enum AddAccountState {
	SetAccount,
	AccountExists,
	AddEmailRequest,
	EnterEmail,
	GeneratePasswordRequest,
	EnterPassword,
	PasswordGenerated,
	PasswordSet,
	Cancel,
}

pub struct AddEntryStateItem {
	switch_state: Arc<Mutex<bool>>,
	internal_state: AddAccountState,
	account_name: String,
	email_name: String,
	password_buffer: String,
	db_manager: Arc<Mutex<DatabaseManager>>,
}

impl AddEntryStateItem {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
		AddEntryStateItem {
			switch_state: Arc::new(Mutex::new(false)),
			internal_state: AddAccountState::SetAccount,
			account_name: String::new(),
			email_name: String::new(),
			password_buffer: String::new(),
			db_manager,
		}
	}

	fn show_account_data(&self, show_account: bool, show_email: bool, show_password: bool, context: &mut TerminalContext) {
		context.print_at_position(0, 0, "Account name:");
		if show_account {
			context.print_at_position(0, 1, self.account_name.as_str());
		}
		context.print_at_position(0, 3, "Email:");
		if show_email {
			context.print_at_position(0, 4, self.email_name.as_str());
		}
		context.print_at_position(0, 6, "Password:");
		if show_password {
			context.print_at_position(0, 7, "*********************");
		}
	}

	fn finalize_account_creation(&mut self) {
		self.write_to_database();
		self.switch_to_main_menu_state(2);
	}

	fn switch_to_main_menu_state(&mut self, duration: u64) {
		wait_for_seconds(duration, Arc::clone(&self.switch_state))
	}

	fn write_to_database(&self) {
		let database_manager = self.db_manager.lock().unwrap();
		let db_context = match database_manager.get_database_context() {
			Some(context) => context,
			None => return,
		};

		let email = if self.email_name.len() == 0 {
			None
		} else {
			Some(self.email_name.clone())
		};

		db_context.add_account(&self.account_name, &self.password_buffer, email).unwrap();
		database_manager.safe_database();
	}

	fn generate_password(&mut self) {
		const SPECIAL_CHARS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";
		let length = 30;
		let mut rng = rand::thread_rng();

		let mut password: Vec<char> = (0..length)
			.map(|_| rng.sample(Alphanumeric) as char)
			.collect();

		for _ in 0..(length / 4) {
			let idx = rng.gen_range(0..length);
			let special_char = *SPECIAL_CHARS.choose(&mut rng).unwrap() as char;
			password[idx] = special_char;
		}


		self.password_buffer = password.into_iter().collect();
	}

	fn check_if_account_already_exists(&self) -> bool {
		let database_manager = self.db_manager.lock().unwrap();
		let db_context = match database_manager.get_database_context() {
			Some(context) => context,
			None => panic!("Could not access database!"),
		};
		let results = match db_context.search_accounts_by_name(&self.account_name) {
			Ok(accounts) => accounts.len(),
			Err(e) => { panic!("Could not read accounts from database! {}", e.to_string()); }
		};
		results > 0
	}
}

impl StateItem for AddEntryStateItem {
	fn display(&self, context: &mut TerminalContext) {
		match self.internal_state {
			AddAccountState::SetAccount => {
				self.show_account_data(false, false, false, context);
				context.draw_input_footer("Account name:".to_string(), &self.account_name)
			}
			AddAccountState::AccountExists => {
				let text = format!("There is already an account called {}", self.account_name);
				let center_y = context.get_height() / 2;
				let pos_x = context.get_width() / 2 - text.len() as u16 / 2;
				context.print_at_position(pos_x, center_y, text.as_str());
			}
			AddAccountState::AddEmailRequest => {
				self.show_account_data(true, false, false, context);
				context.draw_request_footer("Add email for this account?".to_string(), "[Y]es | [N]o".to_string());
			}
			AddAccountState::EnterEmail => {
				self.show_account_data(true, false, false, context);
				context.draw_input_footer("Enter email:".to_string(), &self.email_name)
			}
			AddAccountState::GeneratePasswordRequest => {
				self.show_account_data(true, true, false, context);
				context.draw_request_footer("Generate password for account?".to_string(), "[Y]es | [N]o".to_string());
			}
			AddAccountState::EnterPassword => {
				self.show_account_data(true, true, false, context);
				context.draw_input_footer("Enter password:".to_string(), &"".to_string())
			}
			AddAccountState::PasswordGenerated => {
				self.show_account_data(true, true, true, context);
				context.draw_control_footer(vec!["Secure password has been generated".to_string()]);
			}
			AddAccountState::PasswordSet => {
				self.show_account_data(true, true, true, context);
				context.draw_control_footer(vec!["Password set".to_string()]);
			}
			AddAccountState::Cancel => {
				let text = "Do you want to cancel the account creation?";
				context.print_at_position(context.get_width() / 2 - text.len() as u16 /2, context.get_height() / 2, text);
				context.draw_request_footer("".to_string(), "[Y]es | [N]o".to_string());
			}
		}
	}

	fn update_display(&self) -> bool {
		false
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match self.internal_state {
			AddAccountState::SetAccount => {
				if get_text_input(key_code, &mut self.account_name) {
					if self.check_if_account_already_exists()
					{
						self.internal_state = AddAccountState::AccountExists;
						self.switch_to_main_menu_state(2);
					} else {
						self.internal_state = AddAccountState::AddEmailRequest;
					}
				}
				if key_code == KeyCode::Esc {
					self.internal_state = AddAccountState::Cancel;
				}
			}
			AddAccountState::AccountExists => {}
			AddAccountState::AddEmailRequest => {
				if let Some(confirm) = evaluate_yes_no_answer(key_code) {
					if confirm {
						self.internal_state = AddAccountState::EnterEmail;
					} else {
						self.internal_state = AddAccountState::GeneratePasswordRequest;
					}
				}
				if key_code == KeyCode::Esc{
					self.internal_state = AddAccountState::Cancel;
				}
			}
			AddAccountState::EnterEmail => {
				if get_text_input(key_code, &mut self.email_name) {
					self.internal_state = AddAccountState::GeneratePasswordRequest;
				}
				if key_code == KeyCode::Esc{
					self.internal_state = AddAccountState::Cancel;
				}
			}
			AddAccountState::GeneratePasswordRequest => {
				if let Some(confirm) = evaluate_yes_no_answer(key_code) {
					if confirm {
						self.generate_password();
						self.internal_state = AddAccountState::PasswordGenerated;
						self.finalize_account_creation();
					} else {
						self.internal_state = AddAccountState::EnterPassword
					}
				}
				if key_code == KeyCode::Esc{
					self.internal_state = AddAccountState::Cancel;
				}
			}
			AddAccountState::EnterPassword => {
				if get_text_input(key_code, &mut self.password_buffer) {
					self.internal_state = AddAccountState::PasswordSet;
					self.finalize_account_creation();
				}
				if key_code == KeyCode::Esc{
					self.internal_state = AddAccountState::Cancel;
				}
			}
			AddAccountState::PasswordGenerated => {}
			AddAccountState::PasswordSet => {}
			AddAccountState::Cancel => {
				if let Some(confirm)  = evaluate_yes_no_answer(key_code){
					if confirm {
						self.switch_to_main_menu_state(0);
					}
					else {
						self.internal_state = AddAccountState::SetAccount;
					}
				}
			}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		if *self.switch_state.lock().unwrap() {
			Some(Transition::ToMainMenu)
		} else {
			None
		}
	}
}