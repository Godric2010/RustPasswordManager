use crossterm::event::KeyCode;
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;

enum LockState {
	Locked,
	Invalid,
	Unlocked,
}

pub struct AuthenticationStateItem {
	next_state: Option<Transition>,
	master_password: String,
	lock_state: LockState,
	input_buffer: String,
}

impl AuthenticationStateItem {
	pub fn new() -> Self {
		AuthenticationStateItem {
			next_state: None,
			master_password: "Test".to_string(),
			lock_state: LockState::Locked,
			input_buffer: String::new(),
		}
	}

	fn test_password(&mut self) {
		if self.input_buffer.trim() == self.master_password {
			self.lock_state = LockState::Unlocked;
		} else {
			self.lock_state = LockState::Invalid;
		}
	}

	fn remove_character(&mut self) {
		self.input_buffer.pop();
	}
}

impl StateItem for AuthenticationStateItem {
	fn setup(&mut self) {}

	fn display(&self, context: &mut TerminalContext) {
		let vert_center = context.get_height() / 2;
		let continue_prompt = "Press <Enter> to continue";
		match self.lock_state {
			LockState::Locked => {
				let enter_prompt = "Please enter master password!";
				let pos_x = (context.get_width() - enter_prompt.len() as u16) / 2;
				context.print_at_position(pos_x, vert_center, enter_prompt);
				context.print_at_position(pos_x, vert_center + 1, " ");
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
					self.lock_state = LockState::Locked;
				}
			}
			LockState::Unlocked => {
				if key_code == KeyCode::Enter {
					self.next_state = Some(Transition::ToMainMenu);
				}
			}
		}
	}
	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}