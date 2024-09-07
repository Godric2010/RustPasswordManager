use crossterm::event::KeyCode;
use crate::database_context::DatabaseContext;
use crate::encryption_controller::{encrypt_database, PasswordEncryption};
use crate::file_accesssor::{create_directory_and_files, does_directory_and_files_exist, write_password_to_disk};
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;

use crate::input_handler::*;
use crate::transition::Transition::{ToExit};

pub struct SetAuthenticationStateItem {
	next_state: Option<Transition>,
	input_buffer: String,
	password_encryption: Option<PasswordEncryption>,
	internal_state: SetAuthState,
}

enum SetAuthState {
	EnterPassword,
	ConfirmPassword,
	Success,
	Failure,
}

impl SetAuthenticationStateItem {
	pub fn new() -> Self {
		Self {
			next_state: None,
			input_buffer: String::new(),
			password_encryption: None,
			internal_state: SetAuthState::EnterPassword,
		}
	}

	fn store_pwd(&mut self) {
		let encrypted_password = match &self.password_encryption {
			Some(pwd) => pwd,
			None => panic!("At this point a password should be set!")
		};

		if does_directory_and_files_exist() {
			write_password_to_disk(encrypted_password.create_string());
		} else {
			let empty_db = DatabaseContext::new().unwrap();
			let encrypted_db = encrypt_database(&empty_db, &encrypted_password.get_encrypted_string()).unwrap();
			create_directory_and_files(encrypted_db, encrypted_password.create_string());
		}
	}
}


impl StateItem for SetAuthenticationStateItem {
	fn setup(&mut self) {}

	fn display(&self, context: &mut TerminalContext) {
		let center_y = context.get_height() / 2;
		match self.internal_state {
			SetAuthState::EnterPassword => {
				context.print_at_position(0, center_y, "Set new master password:");
				context.print_at_position(0, center_y + 1, "");
			}
			SetAuthState::ConfirmPassword => {
				context.print_at_position(0, center_y, "Confirm new master password:");
				context.print_at_position(0, center_y + 1, "");
			}
			SetAuthState::Success => {
				context.print_at_position(0, center_y, "Master password set.");
				context.print_at_position(0, center_y + 1, "Press <Enter> to end the program. Start it again to continue.");
			}
			SetAuthState::Failure => {
				context.print_at_position(0, center_y, "Confirmation failed.");
				context.print_at_position(0, center_y + 1, "Press <Enter> to try again.");
			}
		}
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match self.internal_state {
			SetAuthState::EnterPassword => {
				if get_text_input(key_code, &mut self.input_buffer) {
					self.password_encryption = Some(PasswordEncryption::generate_new(&self.input_buffer));
					self.input_buffer.clear();
					self.internal_state = SetAuthState::ConfirmPassword;
				}
			}
			SetAuthState::ConfirmPassword => {
				if get_text_input(key_code, &mut self.input_buffer) {
					if let Some(pwd) = &self.password_encryption {
						if pwd.verify_string(&self.input_buffer) {
							self.internal_state = SetAuthState::Success;
							return;
						}
					}
					self.internal_state = SetAuthState::Failure;
				}
			}
			SetAuthState::Success => {
				if get_enter_press(key_code) {
					self.store_pwd();
					self.next_state = Some(ToExit);
				}
			}
			SetAuthState::Failure => {
				if get_enter_press(key_code) {
					self.input_buffer.clear();
					self.internal_state = SetAuthState::EnterPassword;
				}
			}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}