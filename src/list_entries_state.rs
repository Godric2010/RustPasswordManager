use std::sync::{Arc, Mutex};
use crossterm::event::KeyCode;
use crate::database_context::{Account, DatabaseManager};
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;

pub struct ListEntriesState {
	entries: Vec<Account>,
	next_state: Option<Transition>,
}

impl ListEntriesState {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
		let database_manager = db_manager.lock().unwrap();
		let db_context = match database_manager.get_database_context() {
			Some(context) => context,
			None => panic!("Database not initialized"),
		};
		let entries = db_context.list_all_accounts().unwrap();
		Self {
			entries,
			next_state: None,
		}
	}
}

impl StateItem for ListEntriesState {
	fn setup(&mut self) {}

	fn display(&self, context: &mut TerminalContext) {
		let mut y = 0u16;
		for (index, entry) in self.entries.iter().enumerate() {
			context.print_at_position(0, y + index as u16, entry.account_name.as_str());
			y += 1;
		}
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match key_code {
			KeyCode::Enter => self.next_state = Some(Transition::ToMainMenu),
			_ => {}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}