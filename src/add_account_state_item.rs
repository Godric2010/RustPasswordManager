use crate::database_context::DatabaseManager;
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;
use crossterm::event::KeyCode;
use std::sync::{Arc, Mutex};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::input_handler::*;

enum AddAccountState {
	SetAccount,
	AccountExists,
	AddEmailRequest,
	EnterEmail,
	GeneratePasswordRequest,
	EnterPassword,
	PasswordGenerated,
	PasswordSet,
}

pub struct AddEntryStateItem {
	next_state: Option<Transition>,
	internal_state: AddAccountState,
	account_name: String,
	email_name: String,
	password_buffer: String,
	db_manager: Arc<Mutex<DatabaseManager>>,
}

impl AddEntryStateItem {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
		AddEntryStateItem {
			next_state: None,
			internal_state: AddAccountState::SetAccount,
			account_name: String::new(),
			email_name: String::new(),
			password_buffer: String::new(),
			db_manager,
		}
	}
	pub fn write_to_database(&self) {
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
		context.print_at_position(0, 0, "Account name:");
		context.print_at_position(0, 1, self.account_name.as_str());
		if self.email_name.len() > 0 {
			context.print_at_position(0, 3, "Email:");
			context.print_at_position(0, 4, self.email_name.as_str());
		}

		let prompt_row = context.get_height() - 2;
		let edit_row = prompt_row + 1;

		match self.internal_state {
			AddAccountState::SetAccount => {}
			AddAccountState::AccountExists => {
				context.print_at_position(0, prompt_row, "This account already exists!");
				context.print_at_position(0, edit_row, "Press <Enter> to go back to main menu");
			}
			AddAccountState::AddEmailRequest => {
				context.print_at_position(0, prompt_row, "Do you want to enter an email for this account? [Yes] [N]o");
			}
			AddAccountState::EnterEmail => {
				context.print_at_position(0, prompt_row, "Enter email:");
				context.print_at_position(0, edit_row, self.email_name.as_str());
			}
			AddAccountState::GeneratePasswordRequest => {
				context.print_at_position(0, prompt_row, "Do you want to generate a secure password for this account? [Y]es [N]o")
			}
			AddAccountState::EnterPassword => {
				context.print_at_position(0, prompt_row, "Enter password:");
				context.print_at_position(0, edit_row, "");
			}
			AddAccountState::PasswordGenerated => {
				context.print_at_position(0, prompt_row, "Secure password has been generated!");
				context.print_at_position(0, edit_row, "Press <Enter> to continue");
			}
			AddAccountState::PasswordSet => {
				context.print_at_position(0, prompt_row, "Password set!");
				context.print_at_position(0, edit_row, "Press <Enter> to continue");
			}
		}
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match self.internal_state {
			AddAccountState::SetAccount => {
				if get_text_input(key_code, &mut self.account_name) {
					if self.check_if_account_already_exists()
					{
						self.internal_state = AddAccountState::AccountExists;
					} else {
						self.internal_state = AddAccountState::AddEmailRequest;
					}
				}
			}
			AddAccountState::AccountExists => {
				if get_enter_press(key_code) {
					self.next_state = Some(Transition::ToMainMenu);
				}
			}
			AddAccountState::AddEmailRequest => {
				if let Some(confirm) = evaluate_yes_no_answer(key_code) {
					if confirm {
						self.internal_state = AddAccountState::EnterEmail;
					} else {
						self.internal_state = AddAccountState::GeneratePasswordRequest;
					}
				}
			}
			AddAccountState::EnterEmail => {
				if get_text_input(key_code, &mut self.email_name) {
					self.internal_state = AddAccountState::GeneratePasswordRequest;
				}
			}
			AddAccountState::GeneratePasswordRequest => {
				if let Some(confirm) = evaluate_yes_no_answer(key_code) {
					if confirm {
						self.generate_password();
						self.internal_state = AddAccountState::PasswordGenerated;
					} else {
						self.internal_state = AddAccountState::EnterPassword
					}
				}
			}
			AddAccountState::EnterPassword => {
				if get_text_input(key_code, &mut self.password_buffer) {
					self.internal_state = AddAccountState::PasswordSet;
				}
			}
			AddAccountState::PasswordGenerated => {
				if get_enter_press(key_code) {
					self.write_to_database();
					self.next_state = Some(Transition::ToMainMenu);
				}
			}
			AddAccountState::PasswordSet => {
				if get_enter_press(key_code) {
					self.write_to_database();
					self.next_state = Some(Transition::ToMainMenu);
				}
			}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}