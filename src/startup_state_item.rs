use std::thread;
use std::time::Duration;
use crate::state_item::StateItem;
use crate::transition::Transition;

pub struct StartupStateItem {
	next_state: Option<Transition>
}

impl StartupStateItem {
	pub fn new() -> Self {
		StartupStateItem {
			next_state: None,
		}
	}
}
impl StateItem for StartupStateItem {
	fn setup(&mut self) {
		println!("setting up welcome state")
	}

	fn display(&self) {
		println!("Display welcome state");
	}

	fn register_input(&mut self) {
		thread::sleep(Duration::from_secs(1));
		self.next_state = Some(Transition::ToAuthentication);
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}