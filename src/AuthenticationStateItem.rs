use std::thread;
use std::time::Duration;
use crate::StateItem::StateItem;
use crate::Transition::Transition;

enum LockState {
	Locked,
	Invalid,
	Unlocked,
}

pub struct AuthenticationStateItem {
	next_state: Option<Transition>,
	master_password: String,
	lock_state: LockState,
}

impl AuthenticationStateItem {
	pub fn new() -> Self {
		AuthenticationStateItem {
			next_state: None,
			master_password: "Test".to_string(),
			lock_state: LockState::Locked,
		}
	}

	fn read_and_test_password(&mut self) {
		let mut user_input = String::new();
		std::io::stdin().read_line(&mut user_input).unwrap();

		if user_input.trim() == self.master_password {
			self.lock_state = LockState::Unlocked;
		} else {
			self.lock_state = LockState::Invalid;
		}
	}
}

impl StateItem for AuthenticationStateItem {
	fn setup(&mut self) {
		println!("Setup authentication state...");
	}

	fn display(&self) {
		match self.lock_state {
			LockState::Locked => {
				println!("Please enter master password!");
			}
			LockState::Invalid => {
				println!("Wrong password!");
			}
			LockState::Unlocked => {
				println!("Master password correct!");
			}
		}
	}

	fn register_input(&mut self) {
		match self.lock_state {
			LockState::Locked => self.read_and_test_password(),
			LockState::Invalid => self.lock_state = LockState::Locked,
			LockState::Unlocked => {
				thread::sleep(Duration::from_secs(3));
				self.next_state = Some(Transition::ToMainMenu);
			}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}