use std::sync::{Arc, Mutex};
use crossterm::event::KeyCode;
use crate::database_context::{DatabaseManager};
use crate::encryption_controller::{PasswordEncryption};
use crate::file_accesssor::{does_directory_and_files_exist, write_password_to_disk};
use crate::state_item::{wait_for_seconds, StateItem};
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;
use crate::input_handler::*;

pub struct SetAuthenticationStateItem {
	next_state: Arc<Mutex<bool>>,
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
	Cancel,
}

impl SetAuthenticationStateItem {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
		Self {
			next_state: Arc::new(Mutex::new(false)),
			input_buffer: String::new(),
			password_encryption: None,
			database_manager: db_manager.clone(),
			internal_state: SetAuthState::EnterPassword,
		}
	}

	fn check_if_new_password_is_valid(&mut self) {
		if let Some(pwd) = &self.password_encryption {
			self.internal_state = if pwd.verify_string(&self.input_buffer) {
				self.store_pwd();
				SetAuthState::Success
			} else {
				SetAuthState::Failure
			};
			wait_for_seconds(2, Arc::clone(&self.next_state));
		} else {
			panic!("Password encryption should be available at this point.")
		}
	}


	fn store_pwd(&mut self) {
		let encrypted_password = match &self.password_encryption {
			Some(pwd) => pwd,
			None => panic!("At this point a password should be set!")
		};

		if does_directory_and_files_exist() {
			write_password_to_disk(encrypted_password.create_string());
			self.database_manager.lock().unwrap().set_new_passkey(encrypted_password);
			self.database_manager.lock().unwrap().safe_database();
		} else {
			self.database_manager.lock().unwrap().create_new_database(encrypted_password);
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
				let text = "Master password set!";
				let pos_x = context.get_width() / 2 - text.len() as u16 / 2;
				context.print_at_position(pos_x, center_y, text);
			}
			SetAuthState::Failure => {
				let text = "Confirmation failed";
				let pos_x = context.get_width() / 2 - text.len() as u16 / 2;
				context.print_at_position(pos_x, center_y, text);
			}
			SetAuthState::Cancel => {
				let text = "Do you want to cancel setting a new master password?";
				let pos_x = context.get_width() / 2 - text.len() as u16 / 2;
				context.print_at_position(pos_x, center_y, text);
				context.draw_request_footer(String::new(), "[Y]es | [N]o".to_string());
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
				if key_code == KeyCode::Esc {
					self.internal_state = SetAuthState::Cancel;
				}
			}
			SetAuthState::ConfirmPassword => {
				if get_text_input(key_code, &mut self.input_buffer) {
					self.check_if_new_password_is_valid();
				}
				if key_code == KeyCode::Esc {
					self.internal_state = SetAuthState::Cancel;
				}
			}
			SetAuthState::Cancel => {
				if let Some(confirm) = evaluate_yes_no_answer(key_code) {
					if confirm {
						*self.next_state.lock().unwrap() = true;
					} else {
						self.input_buffer.clear();
						self.internal_state = SetAuthState::EnterPassword;
					}
				}
			}
			SetAuthState::Success => {}
			SetAuthState::Failure => {}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		if *self.next_state.lock().unwrap() {
			match self.internal_state {
				SetAuthState::Cancel => Some(Transition::ToMainMenu),
				SetAuthState::Success => Some(Transition::ToMainMenu),
				SetAuthState::Failure => Some(Transition::ToChangeAuthentication),
				_ => None
			}
		} else {
			None
		}
	}
}