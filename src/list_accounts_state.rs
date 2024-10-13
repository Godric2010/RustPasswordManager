use std::sync::{Arc, Mutex};
use crossterm::event::KeyCode;
use crate::database_context::{Account, DatabaseManager};
use crate::state_item::StateItem;
use crate::terminal_context::{StyleAttribute, TerminalContext};
use crate::transition::Transition;

pub struct ListAccountsState {
	entries: Vec<Account>,
	search_str: String,
	search_str_len: usize,
	selected_index: u16,
	selected_page: u16,
	pages: u16,
	page_entry_len: u16,
	next_state: Option<Transition>,
	database_manager: Arc<Mutex<DatabaseManager>>,
}

impl ListAccountsState {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
		let mut s = Self {
			entries: Vec::new(),
			search_str: String::new(),
			search_str_len: 0,
			selected_index: 0,
			selected_page: 0,
			pages: 0,
			page_entry_len: 10,
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

		let current_search_str_len = self.search_str.len();
		if current_search_str_len != self.search_str_len {
			self.search_str_len = current_search_str_len;
			self.selected_index = 0;
			self.selected_page = 0;
		}


		if current_search_str_len == 0 {
			self.entries = db_context.list_all_accounts().unwrap();
		} else {
			self.entries = db_context.search_accounts_by_name(&self.search_str).unwrap()
		}

		if self.selected_index > self.entries.len() as u16 {
			self.selected_index = 0;
		}

		self.pages = self.entries.len() as u16 / self.page_entry_len;
		if self.entries.len() % 10 > 0 {
			self.pages += 1;
		}

		if self.selected_page > self.pages {
			self.selected_page = 0;
		}
	}

	fn show_search_area(&self, context: &mut TerminalContext) {
		if self.search_str.len() > 0 {
			context.print_styled_at_position(0, 2, "Search:", StyleAttribute::InverseColor);
		} else {
			context.print_at_position(0, 2, "Search:");
		}

		context.print_at_position(0, 3, &self.search_str);
		context.print_line(0, 4, context.get_width() - 1);
	}

	fn show_list_of_accounts(&self, context: &mut TerminalContext) {
		let y_start = 5u16;
		let x_end = context.get_width() - 1;

		let page_text = format!("[{}/{}]", self.selected_page + 1, self.pages);
		let page_text_x_pos = x_end - page_text.len() as u16;

		context.print_at_position(page_text_x_pos, y_start, &page_text);

		let start_index = (self.page_entry_len * self.selected_page) as usize;
		let last_index = usize::min(start_index + self.page_entry_len as usize, self.entries.len());
		let entries_at_page = &self.entries[start_index..last_index];

		for list_index in 0..entries_at_page.len() as u16 {
			let account_index = self.selected_page * self.page_entry_len + list_index;
			let account_name = &self.entries[account_index as usize].account_name;
			if list_index == self.selected_index && self.search_str.len() == 0 {
				context.print_styled_at_position(0, y_start + list_index, account_name.as_str(), StyleAttribute::InverseColor);
			} else {
				context.print_at_position(0, y_start + list_index, account_name.as_str());
			}
		}
	}
}

impl StateItem for ListAccountsState {
	fn display(&self, context: &mut TerminalContext) {
		context.print_at_position(0, 0, "Accounts");
		self.show_search_area(context);

		self.show_list_of_accounts(context);

		let content = vec!["[\u{25b2}] to move down ".to_string(), "[\u{25BC}] to move up ".to_string(), "[\u{21B5}] to select".to_string()];
		context.draw_control_footer(content);
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
				if self.selected_index == self.page_entry_len - 1 {
					self.selected_index = 0;
				} else {
					self.selected_index += 1;
				}
			}
			KeyCode::Up => {
				if self.selected_index == 0 {
					self.selected_index = self.page_entry_len - 1;
				} else {
					self.selected_index -= 1;
				}
			}
			KeyCode::Left => {
				if self.selected_page > 0 {
					self.selected_page -= 1;
				}
			}
			KeyCode::Right => {
				if self.selected_page < self.pages - 1 {
					self.selected_page += 1;
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