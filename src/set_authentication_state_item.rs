use crossterm::event::KeyCode;
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;

use crate::input_handler::*;
use crate::transition::Transition::{ToExit};

pub struct SetAuthenticationStateItem {
	next_state: Option<Transition>,
	master_password: String,
	master_password_confirmation: String,
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
			master_password: String::new(),
			master_password_confirmation: String::new(),
			internal_state: SetAuthState::EnterPassword,
		}
	}

	fn compare_inputs(&self) -> bool {
		if self.master_password == self.master_password_confirmation {
			return true;
		}
		false
	}
}


impl StateItem for SetAuthenticationStateItem {
	fn setup(&mut self) {
	}

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
				if get_text_input(key_code, &mut self.master_password) {
					self.internal_state = SetAuthState::ConfirmPassword;
				}
			}
			SetAuthState::ConfirmPassword => {
				if get_text_input(key_code, &mut self.master_password_confirmation) {
					if self.compare_inputs() {
						self.internal_state = SetAuthState::Success;
					} else {
						self.internal_state = SetAuthState::Failure;
					}
				}
			}
			SetAuthState::Success => {
				if get_enter_press(key_code){
					self.next_state = Some(ToExit);
				}
			}
			SetAuthState::Failure => {
				if get_enter_press(key_code){
					self.master_password.clear();
					self.master_password_confirmation.clear();
					self.internal_state = SetAuthState::EnterPassword;
				}
			}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}