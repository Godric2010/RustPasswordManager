use std::sync::{Arc, Mutex};
use crossterm::event::KeyCode;
use crate::encryption_controller::PasswordEncryption;
use crate::file_accesssor::{delete_directory_and_files, read_password_from_disk};
use crate::input_handler::{evaluate_yes_no_answer, get_text_input};
use crate::state_item::{wait_for_seconds, StateItem};
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;

#[derive(PartialEq)]
enum WipeState {
	ConfirmWipe,
	EnterPassword,
	WipeSuccess,
	WipeFailure,
}
pub struct WipeDatabaseStateItem {
	next_state_ready: Arc<Mutex<bool>>,
	wipe_state: WipeState,
	password_buffer: String,
}

impl WipeDatabaseStateItem {
	pub fn new() -> Self {
		Self {
			next_state_ready: Arc::new(Mutex::new(false)),
			wipe_state: WipeState::ConfirmWipe,
			password_buffer: String::new(),
		}
	}

	fn validate_password_input(&mut self) {
		let pwd_string = read_password_from_disk();
		let master_password = PasswordEncryption::create_from_string(pwd_string.unwrap()).unwrap();
		if master_password.verify_string(self.password_buffer.trim()) {
			delete_directory_and_files();
			self.wipe_state = WipeState::WipeSuccess;
		} else {
			self.wipe_state = WipeState::WipeFailure;
		}
		wait_for_seconds(2, Arc::clone(&self.next_state_ready))
	}
}

impl StateItem for WipeDatabaseStateItem {
	fn display(&self, context: &mut TerminalContext) {
		let center_y = context.get_height() / 2;
		let center_x = context.get_width() / 2;
		match self.wipe_state {
			WipeState::ConfirmWipe => {
				let are_you_sure_text = "Are you sure you want to wipe the database?";
				let warning_text = "This action cannot be undone!";
				context.print_at_position(center_x - are_you_sure_text.len() as u16 / 2, center_y, are_you_sure_text);
				context.print_at_position(center_x - warning_text.len() as u16 / 2, center_y + 1, warning_text);
				context.draw_control_footer(vec!["[Y]es".to_string(), "[N]o".to_string()]);
			}
			WipeState::EnterPassword => {
				let text = "Deleting database... Confirmation required.";
				context.print_at_position(center_x - text.len() as u16 / 2, center_y, text);
				context.draw_input_footer("Enter master password to confirm:".to_string(), &String::new())
			}
			WipeState::WipeSuccess => {
				let text = "Database wiped successfully!";
				context.print_at_position(center_x - text.len() as u16 / 2, center_y, text);
			}
			WipeState::WipeFailure => {
				let text = "Master Password wrong. Failed to wipe the database!";
				context.print_at_position(center_x - text.len() as u16 / 2, center_y, text);
			}
		}
	}

	fn update_display(&self) -> bool {
		false
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match self.wipe_state {
			WipeState::ConfirmWipe => {
				if let Some(confirm_wipe) = evaluate_yes_no_answer(key_code) {
					if confirm_wipe {
						self.wipe_state = WipeState::EnterPassword;
					} else {
						*self.next_state_ready.lock().unwrap() = true;
					}
				}
			}
			WipeState::EnterPassword => {
				if get_text_input(key_code, &mut self.password_buffer) {
					self.validate_password_input();
				}
			}
			WipeState::WipeSuccess => {}
			WipeState::WipeFailure => {}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		if *self.next_state_ready.lock().unwrap() {
			let transition = if self.wipe_state == WipeState::WipeSuccess {
				Transition::ToExit
			} else {
				Transition::ToMainMenu
			};
			Some(transition)
		} else {
			None
		}
	}
}