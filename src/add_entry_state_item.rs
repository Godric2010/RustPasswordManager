use crossterm::event::KeyCode;
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;

use crate::input_handler::*;

enum AddEntryState {
	SetAccount,
	AccountExists,
	GeneratePasswordRequest,
	EnterPassword,
	PasswordGenerated,
	PasswordSet,
}

pub struct AddEntryStateItem {
	next_state: Option<Transition>,
	internal_state: AddEntryState,
	account_name: String,
	password_buffer: String,
}

impl AddEntryStateItem {
	pub fn new() -> Self {
		AddEntryStateItem {
			next_state: None,
			internal_state: AddEntryState::SetAccount,
			account_name: String::new(),
			password_buffer: String::new(),
		}
	}
}

impl StateItem for AddEntryStateItem {
	fn setup(&mut self) {}

	fn display(&self, context: &mut TerminalContext) {
		context.print_at_position(0, 0, "Account name:");
		context.print_at_position(0, 1, self.account_name.as_str());
		match self.internal_state {
			AddEntryState::SetAccount => {}
			AddEntryState::AccountExists => {
				context.print_at_position(0, 2, "This account already exists!");
				context.print_at_position(0, 3, "Press <Enter> to go back to main menu");
			}
			AddEntryState::GeneratePasswordRequest => {
				context.print_at_position(0, 2, "Do you want to generate a secure password for this account? [Y]es [N]o")
			}
			AddEntryState::EnterPassword => {
				context.print_at_position(0, 2, "Enter password:");
				context.print_at_position(0, 3, "");
			}
			AddEntryState::PasswordGenerated => {
				context.print_at_position(0, 2, "Secure password has been generated!");
				context.print_at_position(0, 3, "Press <Enter> to continue");
			}
			AddEntryState::PasswordSet => {
				context.print_at_position(0, 2, "Password set!");
				context.print_at_position(0, 3, "Press <Enter> to continue");
			}
		}
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match self.internal_state {
			AddEntryState::SetAccount => {
				if get_text_input(key_code, &mut self.account_name) {
					self.internal_state = AddEntryState::GeneratePasswordRequest;
				}
			}
			AddEntryState::AccountExists => {
				if get_enter_press(key_code) {
					self.next_state = Some(Transition::ToMainMenu);
				}
			}
			AddEntryState::GeneratePasswordRequest => {
				if let Some(confirm) = evaluate_yes_no_answer(key_code) {
					if confirm {
						self.internal_state = AddEntryState::PasswordGenerated;
					} else {
						self.internal_state = AddEntryState::EnterPassword
					}
				}
			}
			AddEntryState::EnterPassword => {
				let mut pwd_buff = self.password_buffer.clone();
				if get_text_input(key_code, &mut pwd_buff) {
					self.internal_state = AddEntryState::PasswordSet;
					self.password_buffer = pwd_buff;
				}
			}
			AddEntryState::PasswordGenerated => {
				if get_enter_press(key_code) {
					self.next_state = Some(Transition::ToMainMenu);
				}
			}
			AddEntryState::PasswordSet => {
				if get_enter_press(key_code) {
					self.next_state = Some(Transition::ToMainMenu);
				}
			}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}