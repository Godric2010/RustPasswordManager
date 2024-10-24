use std::sync::{Arc, Mutex};
use crossterm::event::KeyCode;
use crate::encryption_controller::PasswordEncryption;
use crate::file_accesssor::{delete_directory_and_files, read_password_from_disk};
use crate::input_handler::{evaluate_yes_no_answer, get_text_input};
use crate::state_item::{wait_for_seconds, StateItem};
use crate::terminal_context::{TerminalContext, Visibility};
use crate::texts::get_texts;
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
				let are_you_sure_text = &get_texts().wipe.are_you_sure_question;
				let warning_text = &get_texts().wipe.warning;
				context.print_at_position(center_x - are_you_sure_text.len() as u16 / 2, center_y, are_you_sure_text);
				context.print_at_position(center_x - warning_text.len() as u16 / 2, center_y + 1, warning_text);
				context.draw_control_footer(vec![&get_texts().misc.confirm_input]);
			}
			WipeState::EnterPassword => {
				let text = &get_texts().wipe.delete_msg;
				context.print_at_position(center_x - text.len() as u16 / 2, center_y, text);
				context.draw_input_footer(&get_texts().wipe.enter_pwd_request, Visibility::Hidden)
			}
			WipeState::WipeSuccess => {
				let text = &get_texts().wipe.success_msg;
				context.print_at_position(center_x - text.len() as u16 / 2, center_y, text);
			}
			WipeState::WipeFailure => {
				let text = &get_texts().wipe.failure_msg;
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