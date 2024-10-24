use crate::file_accesssor::does_directory_and_files_exist;
use crate::state_item::{wait_for_seconds, StateItem};
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;
use crossterm::event::KeyCode;
use std::sync::{Arc, Mutex};
use crate::texts::get_texts;

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
		let state_item = StartupStateItem {
			next_state,
			next_state_ready: Arc::new(Mutex::new(false)),
		};
		wait_for_seconds(2, Arc::clone(&state_item.next_state_ready));

		state_item
	}
}

impl StateItem for StartupStateItem {
	fn display(&self, context: &mut TerminalContext) {
		let welcome_msg = &get_texts().misc.welcome;
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