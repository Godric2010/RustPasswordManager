use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossterm::event::KeyCode;
use crate::file_accesssor::does_directory_and_files_exist;
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;

pub struct StartupStateItem {
	next_state: Transition,
	next_state_ready: Arc<Mutex<bool>>,
}

impl StartupStateItem {
	pub fn new() -> Self {
		let next_state;
		if does_directory_and_files_exist() {
			next_state = Transition::ToAuthentication;
		} else {
			next_state = Transition::ToChangeAuthentication;
		}
		let mut state_item = StartupStateItem {
			next_state,
			next_state_ready: Arc::new(Mutex::new(false)),
		};
		state_item.wait_for_seconds(2);


		state_item
	}

	fn wait_for_seconds(&mut self, duration: u64) {
		let next_state_ready_clone = Arc::clone(&self.next_state_ready);
		thread::spawn(move || {
			thread::sleep(Duration::from_secs(duration));
			*next_state_ready_clone.lock().unwrap() = true;
		});
	}
}
impl StateItem for StartupStateItem {
	fn display(&self, context: &mut TerminalContext) {
		let welcome_msg = "Rusty Password Manager";
		let pos_y = context.get_height() / 2;
		let pos_x = (context.get_width() - welcome_msg.len() as u16) / 2;
		context.print_at_position(pos_x, pos_y, welcome_msg);
	}

	fn update_display(&self) -> bool {
		false
	}

	fn register_input(&mut self, _: KeyCode) {}

	fn next_state(&self) -> Option<Transition> {
		if *self.next_state_ready.lock().unwrap() {
			Some(self.next_state.clone())
		} else {
			None
		}
	}
}