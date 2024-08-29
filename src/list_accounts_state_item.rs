use ncurses::WINDOW;
use crate::state_item::StateItem;
use crate::transition::Transition;

pub struct ListAccountsStateItem{

	next_state: Option<Transition>,
	account_names: Vec<String>,
	selected_name_index: u16,

}

impl ListAccountsStateItem {
	pub fn new() -> Self{
		ListAccountsStateItem{
			next_state: None,
			account_names: vec!["Google".to_string(), "Instagram".to_string(), "Youtube".to_string(), "GitHub".to_string() ],
			selected_name_index: 0,
		}
	}
}

impl StateItem for ListAccountsStateItem{
	fn setup(&mut self) {
	}

	fn display(&self, window: WINDOW, size_x: i32, size_y: i32) {
		println!("Accounts: ");
		for account_name in self.account_names.iter() {
			println!("{}", account_name)
		}

		println!("\n[E]edit [R]emove [F]ind [B]ack")
	}

	fn register_input(&mut self, _char: i32) {


	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}