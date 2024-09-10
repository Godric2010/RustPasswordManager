use crossterm::event::KeyCode;
use crate::encryption_controller::PasswordEncryption;
use crate::file_accesssor::{delete_directory_and_files, read_password_from_disk};
use crate::input_handler::{evaluate_yes_no_answer, get_text_input};
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;

enum WipeState {
	ConfirmWipe,
	EnterPassword,
	WipeSuccess,
	WipeFailure,
}
pub struct WipeDatabaseStateItem {
	next_state: Option<Transition>,
	wipe_state: WipeState,
	password_buffer: String,
}

impl WipeDatabaseStateItem {
	pub fn new() -> Self {
		Self { next_state: None, wipe_state: WipeState::ConfirmWipe, password_buffer: String::new() }
	}
}

impl StateItem for WipeDatabaseStateItem {
	fn display(&self, context: &mut TerminalContext) {
		let center_y = context.get_height() / 2;
		match self.wipe_state {
			WipeState::ConfirmWipe => {
				context.print_at_position(0, center_y, "Are you sure you want to wipe the database?");
				context.print_at_position(0, center_y + 1, "This action cannot be undone!");
				context.print_at_position(0, center_y + 2, "[Y]es/[N]o");
			}
			WipeState::EnterPassword => {
				context.print_at_position(0, center_y, "Enter master password to confirm:");
				context.print_at_position(0, center_y + 1, "");
			}
			WipeState::WipeSuccess => {
				context.print_at_position(0, center_y, "Database wiped successfully!");
				context.print_at_position(0, center_y + 1, "Press <Enter> quit the application.");
			}
			WipeState::WipeFailure => {
				context.print_at_position(0, center_y, "Master Password wrong. Failed to wipe the database!");
				context.print_at_position(0, center_y + 1, "Press <Enter> to go back to the main menu.");
			}
		}
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match self.wipe_state {
			WipeState::ConfirmWipe => {
				if let Some(confirm_wipe) = evaluate_yes_no_answer(key_code) {
					if confirm_wipe {
						self.wipe_state = WipeState::EnterPassword;
					} else {
						self.next_state = Some(Transition::ToMainMenu);
					}
				}
			}
			WipeState::EnterPassword => {
				if get_text_input(key_code, &mut self.password_buffer) {
					let pwd_string = read_password_from_disk();
					let master_password = PasswordEncryption::create_from_string(pwd_string.unwrap()).unwrap();
					if master_password.verify_string(self.password_buffer.trim()) {
						delete_directory_and_files();
						self.wipe_state = WipeState::WipeSuccess;
					} else {
						self.wipe_state = WipeState::WipeFailure;
					}
				}
			}
			WipeState::WipeSuccess => {
				if key_code == KeyCode::Enter {
					self.next_state = Some(Transition::ToExit);
				}
			}
			WipeState::WipeFailure => {
				self.next_state = Some(Transition::ToMainMenu);
			}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}