use crate::database_context::DatabaseManager;
use crate::encryption_controller::PasswordEncryption;
use crate::file_accesssor::read_password_from_disk;
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;
use crossterm::event::KeyCode;
use std::sync::{Arc, Mutex};

enum LockState {
	Locked,
	Invalid,
	Unlocked,
}

pub struct AuthenticationStateItem {
	next_state: Option<Transition>,
	master_password: PasswordEncryption,
	lock_state: LockState,
	input_buffer: String,
	db_manager: Arc<Mutex<DatabaseManager>>,
}

impl AuthenticationStateItem {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
		let pwd_string = read_password_from_disk();
		let master_password = PasswordEncryption::create_from_string(pwd_string.unwrap()).unwrap();

		AuthenticationStateItem {
			next_state: None,
			master_password,
			lock_state: LockState::Locked,
			input_buffer: String::new(),
			db_manager,
		}
	}

	fn test_password(&mut self) {
		if self.master_password.verify_string(self.input_buffer.trim()) {
			self.lock_state = LockState::Unlocked;
		} else {
			self.lock_state = LockState::Invalid;
		}
	}

	fn remove_character(&mut self) {
		self.input_buffer.pop();
	}

	fn unlock_database(&self) {
		self.db_manager.lock().unwrap().unlock(&self.master_password.get_encrypted_string());
	}
}

impl StateItem for AuthenticationStateItem {

	fn display(&self, context: &mut TerminalContext) {
		let vert_center = context.get_height() / 2;
		let continue_prompt = "Press <Enter> to continue";
		match self.lock_state {
			LockState::Locked => {
				let enter_prompt = "Please enter master password!";
				let pos_x = (context.get_width() - enter_prompt.len() as u16) / 2;
				context.print_at_position(pos_x, vert_center, enter_prompt);
				context.print_at_position(pos_x, vert_center + 1, "");
				context.move_cursor_to_position(pos_x, vert_center + 1);
			}
			LockState::Invalid => {
				let enter_prompt = "Invalid password!";
				let pos_x_enter = (context.get_width() - enter_prompt.len() as u16) / 2;
				let pos_x_continue = (context.get_width() - continue_prompt.len() as u16) / 2;
				context.print_at_position(pos_x_enter, vert_center, enter_prompt);
				context.print_at_position(pos_x_continue, vert_center + 1, continue_prompt);
			}
			LockState::Unlocked => {
				let enter_prompt = "Password correct!";
				let pos_x_enter = (context.get_width() - enter_prompt.len() as u16) / 2;
				let pos_x_continue = (context.get_width() - continue_prompt.len() as u16) / 2;
				context.print_at_position(pos_x_enter, vert_center, enter_prompt);
				context.print_at_position(pos_x_continue, vert_center + 1, continue_prompt);
			}
		}
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match self.lock_state {
			LockState::Locked => {
				match key_code {
					KeyCode::Char(c) => self.input_buffer.push(c),
					KeyCode::Backspace => self.remove_character(),
					KeyCode::Enter => self.test_password(),
					_ => (),
				};
			}
			LockState::Invalid => {
				if key_code == KeyCode::Enter {
					self.input_buffer.clear();
					self.lock_state = LockState::Locked;
				}
			}
			LockState::Unlocked => {
				if key_code == KeyCode::Enter {
					self.unlock_database();
					self.next_state = Some(Transition::ToMainMenu);
				}
			}
		}
	}
	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}