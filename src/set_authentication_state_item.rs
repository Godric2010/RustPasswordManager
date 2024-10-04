use std::sync::{Arc, Mutex};
use crossterm::event::KeyCode;
use crate::database_context::{DatabaseContext, DatabaseManager};
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
	database_manager: Arc<Mutex<DatabaseManager>>,
	internal_state: SetAuthState,
}

enum SetAuthState {
	EnterPassword,
	ConfirmPassword,
	Success,
	Failure,
}

impl SetAuthenticationStateItem {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) ->Self{
		Self{
			next_state: None,
			input_buffer: String::new(),
			password_encryption: None,
			database_manager: db_manager.clone(),
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
	fn display(&self, context: &mut TerminalContext) {
		let pos_x = context.get_width() / 4;
		let center_y = context.get_height() / 2;
		match self.internal_state {
			SetAuthState::EnterPassword => {
				context.print_at_position(pos_x, center_y - 1, "Set new master password:");
				context.print_at_position(pos_x, center_y, "");
				context.draw_control_footer(vec!["Press [\u{21B5}] to confirm input".to_string()]);
				context.move_cursor_to_position(pos_x, center_y);
			}
			SetAuthState::ConfirmPassword => {
				context.print_at_position(pos_x, center_y - 1, "Confirm new master password:");
				context.print_at_position(pos_x, center_y, "");
				context.draw_control_footer(vec!["[\u{21B5}] to confirm input".to_string(), "[ESC] to quit".to_string()]);
				context.move_cursor_to_position(pos_x, center_y);
			}
			SetAuthState::Success => {
				context.print_at_position(pos_x, center_y, "Master password set!");
				context.print_at_position(pos_x, center_y + 1, "Restart the program to continue editing.");
				context.draw_control_footer(vec!["[\u{21B5}] to continue".to_string()]);
			}
			SetAuthState::Failure => {
				context.print_at_position(pos_x, center_y, "Confirmation failed!");
				context.draw_control_footer(vec!["[\u{21B5}] to continue".to_string()]);
			}
		}
	}

	fn update_display(&self) -> bool {
		false
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