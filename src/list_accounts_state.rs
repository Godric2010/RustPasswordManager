use std::sync::{Arc, Mutex};
use crossterm::event::KeyCode;
use crate::database_context::{Account, DatabaseManager};
use crate::state_item::StateItem;
use crate::terminal_context::{StyleAttribute, TerminalContext};
use crate::transition::Transition;

pub struct ListAccountsState {
	entries: Vec<Account>,
	search_str: String,
	selected_index: u16,
	next_state: Option<Transition>,
	database_manager: Arc<Mutex<DatabaseManager>>,
}

impl ListAccountsState {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
		let mut s = Self {
			entries: Vec::new(),
			search_str: String::new(),
			selected_index: 0,
			next_state: None,
			database_manager: db_manager.clone(),
		};
		s.filter_entries();
		s
	}

	fn filter_entries(&mut self) {
		let db_manager = self.database_manager.lock().unwrap();
		let db_context = match db_manager.get_database_context() {
			Some(context) => context,
			None => panic!("Database not initialized"),
		};
		if self.search_str.len() == 0 {
			self.entries = db_context.list_all_accounts().unwrap();
		} else {
			self.entries = db_context.search_accounts_by_name(&self.search_str).unwrap()
		}

		if self.selected_index > self.entries.len() as u16 {
			self.selected_index = 0;
		}
	}
}

impl StateItem for ListAccountsState {
	fn display(&self, context: &mut TerminalContext) {
		context.print_at_position(0, 0, "Accounts");
		if self.search_str.len() > 0 {
			context.print_styled_at_position(0, 2, "Search:", StyleAttribute::InverseColor);
		} else {
			context.print_at_position(0, 2, "Search:");
		}
		context.print_at_position(0, 3, &self.search_str);

		let y = 5u16;
		for (index, entry) in self.entries.iter().enumerate() {
			let idx = index as u16;
			let y_pos = y + idx;
			let account_name = entry.account_name.clone();
			if idx == self.selected_index && self.search_str.len() == 0 {
				context.print_styled_at_position(0, y_pos, account_name.as_str(), StyleAttribute::InverseColor);
			} else {
				context.print_at_position(0, y_pos, account_name.as_str());
			}
		}
	}

	fn update_display(&self) -> bool {
		false
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match key_code {
			KeyCode::Char(c) => self.search_str.push(c),
			KeyCode::Backspace => { self.search_str.pop(); }
			KeyCode::Enter => {
				let account = self.entries.get(self.selected_index as usize).unwrap();
				self.next_state = Some(Transition::ToShowAccount(account.clone()))
			}
			KeyCode::Down => {
				if self.selected_index == self.entries.len() as u16 - 1 {
					self.selected_index = 0;
				} else {
					self.selected_index += 1;
				}
			}
			KeyCode::Up => {
				if self.selected_index == 0 {
					self.selected_index = self.entries.len() as u16 - 1;
				} else {
					self.selected_index -= 1;
				}
			}
			_ => {}
		}
		self.filter_entries();
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}