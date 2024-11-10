use crate::database_context::Account;
use crate::terminal_context::{StyleAttribute, TerminalContextOld};

struct Entry {
	account_id: i32,
	account_name: String,
}

const MAX_ENTRIES_PER_PAGE: usize = 10;
struct Page {
	entries: Vec<Entry>,
	selected_idx: usize,
}

pub struct PageView {
	pages: Vec<Page>,
	selected_idx: usize,
}

impl Entry {
	pub fn new(account: &Account) -> Self
	{
		Entry {
			account_id: account.id,
			account_name: account.account_name.clone(),
		}
	}

	pub fn get_account_id(&self) -> i32 {
		self.account_id
	}

	pub fn get_account_name(&self) -> &str {
		&self.account_name
	}
}

impl Page {
	pub fn new(accounts: &[Account]) -> Self {
		let entries: Vec<Entry> = accounts.iter().map(|account| Entry::new(account)).collect();
		Page {
			entries,
			selected_idx: 0,
		}
	}

	pub fn display_entries(&self, context: &mut TerminalContextOld, pos_x: u16, pos_y: u16) {
		for (idx, entry) in self.entries.iter().enumerate() {
			if idx == self.selected_idx {
				context.print_styled_at_position(pos_x, pos_y + idx as u16, &entry.get_account_name(), StyleAttribute::InverseColor);
			} else {
				context.print_at_position(pos_x, pos_y + idx as u16, &entry.get_account_name());
			}
		}
	}

	pub fn increase_selected_idx(&mut self) {
		if self.selected_idx == self.entries.len() - 1 {
			return;
		}
		self.selected_idx += 1;
	}

	pub fn decrease_selected_idx(&mut self) {
		if self.selected_idx == 0 {
			return;
		}
		self.selected_idx -= 1;
	}

	pub fn get_selected_account_id(&self) -> i32 {
		self.entries[self.selected_idx].get_account_id()
	}
}


impl PageView {
	pub fn new(accounts: &[Account]) -> Self {
		let mut pages_needed = accounts.len() / MAX_ENTRIES_PER_PAGE;
		if accounts.len() % 10 > 0 {
			pages_needed += 1;
		}

		let mut pages: Vec<Page> = vec![];
		for i in 0..pages_needed {
			let start = i * MAX_ENTRIES_PER_PAGE;
			let end = usize::min(start + 10, accounts.len());
			let accounts_at_page = &accounts[start..end];
			pages.push(Page::new(accounts_at_page));
		}

		Self {
			pages,
			selected_idx: 0,
		}
	}

	pub fn new_empty() -> Self {
		Self {
			pages: Vec::new(),
			selected_idx: 0,
		}
	}

	pub fn display_page(&self, context: &mut TerminalContextOld, pos_x: u16, pos_y: u16) {
		let page_text = format!("[{}/{}]", self.selected_idx + 1, self.pages.len());
		let page_text_x_pos = context.get_width() - 1 - page_text.len() as u16;

		context.print_at_position(page_text_x_pos, pos_y, &page_text);

		if self.pages.len() == 0 {
			return;
		}

		self.pages[self.selected_idx].display_entries(context, pos_x, pos_y);
	}

	pub fn next_page(&mut self) {
		if self.pages.len() == 0 || self.selected_idx == self.pages.len() - 1 {
			return;
		}
		self.selected_idx += 1;
	}

	pub fn prev_page(&mut self) {
		if self.selected_idx == 0  || self.pages.len() == 0{
			return;
		}
		self.selected_idx -= 1;
	}

	pub fn next_account(&mut self) {
		if self.pages.len() == 0{
			return;
		}
		self.pages[self.selected_idx].increase_selected_idx();
	}

	pub fn prev_account(&mut self) {
		if self.pages.len() == 0{
			return;
		}
		self.pages[self.selected_idx].decrease_selected_idx();
	}

	pub fn get_selected_account_id(&self) -> Option<i32> {
		if self.pages.len() == 0{
			return None;
		}
		Some(self.pages[self.selected_idx].get_selected_account_id())
	}
}