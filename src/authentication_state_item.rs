use crate::database_context::DatabaseManager;
use crate::encryption_controller::PasswordEncryption;
use crate::file_accesssor::read_password_from_disk;
use crate::input_handler::get_text_input;
use crate::state_item::{wait_for_seconds, StateItem};
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;
use crossterm::event::KeyCode;
use std::sync::{Arc, Mutex};
use crate::password_widget::PasswordWidget;
use crate::texts::get_texts;
use crate::widget::Widget;

enum LockState {
	Locked,
	Invalid,
	Unlocked,
}

pub struct AuthenticationStateItem {
	next_state_ready: Arc<Mutex<bool>>,
	master_password: PasswordEncryption,
	lock_state: LockState,
	input_buffer: String,
	db_manager: Arc<Mutex<DatabaseManager>>,
	password_widget: PasswordWidget,
}

impl AuthenticationStateItem {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
		let pwd_string = read_password_from_disk();
		let master_password = PasswordEncryption::create_from_string(pwd_string.unwrap()).unwrap();
		let input_buffer = String::new();

		db_manager.lock().unwrap().load_database_from_disk();
		AuthenticationStateItem {
			next_state_ready: Arc::new(Mutex::new(false)),
			master_password,
			password_widget: PasswordWidget::new(input_buffer.clone()),
			lock_state: LockState::Locked,
			input_buffer,
			db_manager,
		}
	}

	fn test_password(&mut self) {
		if self.master_password.verify_string(self.input_buffer.trim()) {
			self.lock_state = LockState::Unlocked;
			wait_for_seconds(1, Arc::clone(&self.next_state_ready));
			self.unlock_database();
		} else {
			self.lock_state = LockState::Invalid;
		}
	}

	fn unlock_database(&self) {
		self.db_manager.lock().unwrap().unlock(&self.master_password.get_encrypted_string());
	}
}

impl StateItem for AuthenticationStateItem {
	fn display(&self, context: &mut TerminalContext) {
		let vert_center = context.get_height() / 2;
		match self.lock_state {
			LockState::Locked => {
				let enter_prompt = &get_texts().auth.enter_pwd_promt;
				let pos_x = (context.get_width() - enter_prompt.len() as u16) / 2;
				context.print_at_position(pos_x, vert_center, enter_prompt);
				self.password_widget.display(context, pos_x, vert_center + 1);
				context.draw_control_footer(vec![&get_texts().input.enter]);
				context.move_cursor_to_position(pos_x, vert_center + 1);
			}
			LockState::Invalid => {
				let enter_prompt = &get_texts().auth.invalid_pwd;
				let pos_x_enter = (context.get_width() - enter_prompt.len() as u16) / 2;
				context.print_at_position(pos_x_enter, vert_center, enter_prompt);
				context.draw_control_footer(vec![&get_texts().input.enter])
			}
			LockState::Unlocked => {
				let enter_prompt = &get_texts().auth.valid_pwd;
				let pos_x_enter = (context.get_width() - enter_prompt.len() as u16) / 2;
				context.print_at_position(pos_x_enter, vert_center, enter_prompt);
			}
		}
	}

	fn update_display(&self) -> bool {
		false
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match self.lock_state {
			LockState::Locked => {
				if get_text_input(key_code, &mut self.input_buffer) {
					self.test_password();
				}
				self.password_widget.update_password(self.input_buffer.clone());
			}
			LockState::Invalid => {
				if key_code == KeyCode::Enter {
					self.input_buffer.clear();
					self.lock_state = LockState::Locked;
				}
			}
			LockState::Unlocked => {}
		}
	}
	fn next_state(&self) -> Option<Transition> {
		if *self.next_state_ready.lock().unwrap() {
			Some(Transition::ToMainMenu)
		} else {
			None
		}
	}
}