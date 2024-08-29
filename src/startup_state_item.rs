use std::thread;
use std::time::Duration;
use crossterm::event::KeyCode;
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
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
	fn setup(&mut self){
		self.next_state = Some(Transition::ToAuthentication)
	}

	fn display(&self, context: &mut TerminalContext) {
		let welcome_msg = "Rusty Password Manager";
		let pos_y = context.get_height() /2;
		let pos_x = (context.get_width() - welcome_msg.len() as u16) / 2;
		context.print_at_position(pos_x, pos_y, welcome_msg);
	}

	fn register_input(&mut self, key_code: KeyCode) {

	}

	fn next_state(&self) -> Option<Transition> {
		thread::sleep(Duration::from_secs(2));
		self.next_state.clone()
	}
}