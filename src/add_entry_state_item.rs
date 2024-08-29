use std::thread;
use std::time::Duration;
use crossterm::event::KeyCode;
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;
use crate::transition::Transition::ToMainMenu;

enum AddEntryState {
	SetAccount,
	AccountExists,
	GeneratePasswordRequest,
	EnterPassword,
	PasswordGenerated,
	PasswordSet,
}

pub struct AddEntryStateItem {
	next_state: Option<Transition>,
	internal_state: AddEntryState,
	account_name: String,
}

impl AddEntryStateItem {
	pub fn new() -> Self {
		AddEntryStateItem {
			next_state: None,
			internal_state: AddEntryState::SetAccount,
			account_name: String::new(),
		}
	}

	fn print_state(&self) {
		let account_name_heading = "Account name:";
		let account_exists = "The account already exists!";
		let generate_password_promt = "Do you want to automatically generate a secure password? [Y]es | [N]o";
		let enter_password_heading = "Enter password:";
		let password_generated = "Password generated!";
		let password_entered = "Password set!";

		let output_text = match self.internal_state {
			AddEntryState::SetAccount => account_name_heading,
			AddEntryState::AccountExists => &*(account_exists.to_string() + "\n"),
			AddEntryState::GeneratePasswordRequest => &*(account_name_heading.to_string() + "\n" + &self.account_name + generate_password_promt),
			AddEntryState::EnterPassword => &*(account_name_heading.to_string() + "\n" + &self.account_name + "\n" + enter_password_heading),
			AddEntryState::PasswordGenerated => &*(account_name_heading.to_string() + "\n" + &self.account_name + "\n" + password_generated),
			AddEntryState::PasswordSet => &*(account_name_heading.to_string() + "\n" + &self.account_name + "\n********\n" + password_entered),
		};

		println!("{}", output_text);
	}

	fn set_account_name(&mut self, account_name: &str) {
		self.account_name = account_name.to_string();
		self.internal_state = AddEntryState::GeneratePasswordRequest;
	}

	fn set_password(&mut self, password: &str) {
		self.internal_state = AddEntryState::PasswordSet;
	}

	fn evaluate_password_generation_request(&mut self, input: &str) {
		match input.to_ascii_lowercase().trim() {
			"y" => {
				self.internal_state = AddEntryState::PasswordGenerated;
			}
			"n" => {
				self.internal_state = AddEntryState::EnterPassword;
			}
			_ => self.internal_state = AddEntryState::GeneratePasswordRequest,
		}
	}
}

impl StateItem for AddEntryStateItem {
	fn setup(&mut self) {}

	fn display(&self, context: &mut TerminalContext) {
		print!("{}[2J", 27 as char);
		self.print_state();
	}

	fn register_input(&mut self, key_code: KeyCode) {
		let mut user_input = String::new();

		match self.internal_state {
			AddEntryState::SetAccount => {
				std::io::stdin().read_line(&mut user_input).unwrap();
				self.set_account_name(&user_input.as_str());
			}
			AddEntryState::AccountExists => {
				thread::sleep(Duration::from_secs(3));
				self.next_state = Some(ToMainMenu);
			}
			AddEntryState::GeneratePasswordRequest => {
				std::io::stdin().read_line(&mut user_input).unwrap();
				self.evaluate_password_generation_request(&user_input.as_str());
			}
			AddEntryState::EnterPassword => {
				std::io::stdin().read_line(&mut user_input).unwrap();
				self.set_password(&user_input.as_str());
			}
			AddEntryState::PasswordGenerated => {
				thread::sleep(Duration::from_secs(3));
				self.next_state = Some(ToMainMenu);
			}
			AddEntryState::PasswordSet => {
				thread::sleep(Duration::from_secs(3));
				self.next_state = Some(ToMainMenu);
			}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}