use std::sync::{Arc, Mutex};
use crossterm::event::KeyCode;
use crate::database_context::{Account, DatabaseManager};
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;

enum ShowAccountState {
	ShowAccount,
	EditAccountName,
	EditPassword,
	EditEmail,
	CopyPassword,
	SaveChanges,
}

pub struct ShowAccountStateItem {
	account: Account,
	text_buffer: String,
	internal_state: ShowAccountState,
	account_changed: bool,
	next_state: Option<Transition>,
	db_manager: Arc<Mutex<DatabaseManager>>,
}

impl ShowAccountStateItem {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>> ,account: Account) -> Self {
		Self {
			account,
			text_buffer: String::new(),
			account_changed: false,
			internal_state: ShowAccountState::ShowAccount,
			db_manager,
			next_state: None,
		}
	}
}

impl StateItem for ShowAccountStateItem {

	fn display(&self, context: &mut TerminalContext) {
		context.print_at_position(0, 0, "Account");

		context.print_at_position(0, 2, "Name:");
		context.print_at_position(0, 3, &self.account.account_name);

		context.print_at_position(0, 5, "Email:");
		let email = match &self.account.email {
			Some(email) => email,
			None => "",
		};
		context.print_at_position(0, 6, &email);

		context.print_at_position(0, 8, "Password:");

		context.print_at_position(0, 9, &self.account.password);
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match key_code {
			KeyCode::Char('q')=> {
				self.next_state = Some(Transition::ToMainMenu);
			},
			_=>{},
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}