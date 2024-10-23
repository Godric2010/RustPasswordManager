use std::sync::{Arc, Mutex};
use crossterm::event::KeyCode;
use crate::database_context::{Account, DatabaseManager};
use crate::page_list_view::PageView;
use crate::state_item::StateItem;
use crate::terminal_context::{StyleAttribute, TerminalContext};
use crate::transition::Transition;


enum ListState {
	List,
	Search,
}

pub struct ListAccountsState {
	entries: Vec<Account>,
	search_str: String,
	page_view: PageView,
	internal_state: ListState,
	next_state: Option<Transition>,
	database_manager: Arc<Mutex<DatabaseManager>>,
}

impl ListAccountsState {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
		let mut s = Self {
			entries: Vec::new(),
			search_str: String::new(),
			internal_state: ListState::List,
			page_view: PageView::new_empty(),
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

		self.entries = match &self.internal_state {
			ListState::List => {
				db_context.list_all_accounts().unwrap()
			}
			ListState::Search => {
				db_context.search_accounts_by_name(&self.search_str).unwrap()
			}
		};

		self.page_view = PageView::new(&self.entries);
	}


	fn show_search_area(&self, context: &mut TerminalContext, search_str: &String) {
		context.print_styled_at_position(0, 2, "Search:", StyleAttribute::InverseColor);
		context.print_at_position(0, 3, search_str);
		context.print_line(0, 4, context.get_width() - 1);
	}

	fn show_list_of_accounts(&self, context: &mut TerminalContext) {
		let y_start = 5u16;
		self.page_view.display_page(context, 0, y_start);
	}

	fn select_account(&mut self) {
		let selected_account_id = self.page_view.get_selected_account_id();
		let id = match selected_account_id {
			Some(id) => id,
			None => return,
		};

		let account = self.entries.iter().find(|account| account.id == id);
		let selected_account = match account {
			Some(account) => account,
			None => return,
		};

		self.next_state = Some(Transition::ToShowAccount(selected_account.clone()))
	}

	fn input_list_state(&mut self, key_code: KeyCode) {
		match key_code {
			KeyCode::Enter => {
				self.select_account();
			}
			KeyCode::Down => {
				self.page_view.next_account();
			}
			KeyCode::Up => {
				self.page_view.prev_account();
			}
			KeyCode::Left => {
				self.page_view.prev_page();
			}
			KeyCode::Right => {
				self.page_view.next_page();
			}
			KeyCode::Char('q') => {
				self.next_state = Some(Transition::ToMainMenu);
			}
			KeyCode::Char('s') => {
				self.internal_state = ListState::Search;
			}
			_ => {}
		}
	}

	fn input_search_state(&mut self, key_code: KeyCode) {
		match key_code {
			KeyCode::Char(c) => {
				self.search_str.push(c);
				self.filter_entries();
			}
			KeyCode::Backspace => {
				self.search_str.pop();
				self.filter_entries();
			}
			KeyCode::Esc => {
				self.search_str.clear();
				self.filter_entries();
				self.internal_state = ListState::List;
			}
			KeyCode::Enter => {
				self.select_account();
			}
			KeyCode::Down => {
				self.page_view.next_account();
			}
			KeyCode::Up => {
				self.page_view.prev_account();
			}
			KeyCode::Left => {
				self.page_view.prev_page();
			}
			KeyCode::Right => {
				self.page_view.next_page();
			}
			_ => {}
		}
	}
}

impl StateItem for ListAccountsState {
	fn display(&self, context: &mut TerminalContext) {
		context.print_at_position(0, 0, "Accounts");

		let control_footer_help;
		match &self.internal_state {
			ListState::List => {
				control_footer_help = vec!["[S]earch".to_string(), "[Q]uit".to_string(), "[\u{25b2}] down".to_string(), "[\u{25BC}] up".to_string(), "[\u{25C0}] left".to_string(), "[\u{25B6}] right".to_string(), "[\u{21B5}] select".to_string()];
			}
			ListState::Search => {
				control_footer_help = vec!["[Esc] end search".to_string(), "[\u{25b2}] down".to_string(), "[\u{25BC}] up".to_string(), "[\u{25C0}] left".to_string(), "[\u{25B6}] right".to_string(), "[\u{21B5}] select".to_string()];
				self.show_search_area(context, &self.search_str);
			}
		}

		self.show_list_of_accounts(context);

		context.draw_control_footer(control_footer_help);
	}

	fn update_display(&self) -> bool {
		false
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match &self.internal_state {
			ListState::List => {
				self.input_list_state(key_code);
			}
			ListState::Search => {
				self.input_search_state(key_code)
			}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}