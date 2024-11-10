use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossterm::event::KeyCode;
use crate::terminal_context::TerminalContextOld;
use crate::transition::Transition;

pub trait StateItem{
	fn display(&self, context: &mut TerminalContextOld);

	fn update_display(&self) -> bool;

	fn display_content(&self, context: &mut TerminalContextOld){
		context.clear_screen();
		context.draw_border();
		self.display(context);
	}

	fn register_input(&mut self, key_code: KeyCode);
	fn shutdown(&mut self){
		print!("{}[2J", 27 as char);
	}
	fn next_state(&self) -> Option<Transition>;
}

pub fn wait_for_seconds(duration: u64, time_is_up: Arc<Mutex<bool>>){
	thread::spawn(move || {
		thread::sleep(Duration::from_secs(duration));
		*time_is_up.lock().unwrap() = true;
	});
}