use crate::StateItem::StateItem;
use crate::Transition::Transition;

pub struct MainMenuStateItem {
	next_state: Option<Transition>,
}

impl MainMenuStateItem {
	pub fn new() -> Self {
		MainMenuStateItem {
			next_state: None,
		}
	}
}

impl StateItem for MainMenuStateItem {
	fn setup(&mut self) {}

	fn display(&self) {
		println!("Main Menu");
		println!("[1] Add new account");
		println!("[2] List all accounts");
		println!("[3] Search for account");
		println!("[4] Set new master password");
		println!("[5] Exit")
	}

	fn register_input(&mut self) {
		let mut user_input = String::new();
		std::io::stdin().read_line(&mut user_input).unwrap();

		match user_input.trim() {
			"1" => self.next_state = Some(Transition::ToAddEntry),
			"2" => self.next_state = Some(Transition::ToListEntries),
			"3" => self.next_state = Some(Transition::ToSearchEntry),
			"4" => println!("Setting master password coming soon!"),
			"5" => self.next_state = Some(Transition::ToExit),
			_ => println!("Invalid input! Please enter a number to select a menu item.")
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}