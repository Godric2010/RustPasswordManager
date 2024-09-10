use crate::database_context::DatabaseManager;
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;
use crossterm::event::KeyCode;
use std::sync::{Arc, Mutex};

use crate::input_handler::*;

enum AddAccountState {
	SetAccount,
	AccountExists,
	GeneratePasswordRequest,
	EnterPassword,
	PasswordGenerated,
	PasswordSet,
}

pub struct AddEntryStateItem {
	next_state: Option<Transition>,
	internal_state: AddAccountState,
	account_name: String,
	password_buffer: String,
	db_manager: Arc<Mutex<DatabaseManager>>,
}

impl AddEntryStateItem {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
		AddEntryStateItem {
			next_state: None,
			internal_state: AddAccountState::SetAccount,
			account_name: String::new(),
			password_buffer: String::new(),
			db_manager
		}
	}
	pub fn write_to_database(&self){
		let database_manager = self.db_manager.lock().unwrap();
		let db_context = match  database_manager.get_database_context(){
			Some(context) => context,
			None => return,
		};

		db_context.add_account(&self.account_name, &self.password_buffer, None).unwrap();
		database_manager.safe_database();
	}
}

impl StateItem for AddEntryStateItem {

	fn display(&self, context: &mut TerminalContext) {
		context.print_at_position(0, 0, "Account name:");
		context.print_at_position(0, 1, self.account_name.as_str());
		match self.internal_state {
			AddAccountState::SetAccount => {}
			AddAccountState::AccountExists => {
				context.print_at_position(0, 2, "This account already exists!");
				context.print_at_position(0, 3, "Press <Enter> to go back to main menu");
			}
			AddAccountState::GeneratePasswordRequest => {
				context.print_at_position(0, 2, "Do you want to generate a secure password for this account? [Y]es [N]o")
			}
			AddAccountState::EnterPassword => {
				context.print_at_position(0, 2, "Enter password:");
				context.print_at_position(0, 3, "");
			}
			AddAccountState::PasswordGenerated => {
				context.print_at_position(0, 2, "Secure password has been generated!");
				context.print_at_position(0, 3, "Press <Enter> to continue");
			}
			AddAccountState::PasswordSet => {
				context.print_at_position(0, 2, "Password set!");
				context.print_at_position(0, 3, "Press <Enter> to continue");
			}
		}
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match self.internal_state {
			AddAccountState::SetAccount => {
				if get_text_input(key_code, &mut self.account_name) {
					self.internal_state = AddAccountState::GeneratePasswordRequest;
				}
			}
			AddAccountState::AccountExists => {
				if get_enter_press(key_code) {
					self.next_state = Some(Transition::ToMainMenu);
				}
			}
			AddAccountState::GeneratePasswordRequest => {
				if let Some(confirm) = evaluate_yes_no_answer(key_code) {
					if confirm {
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