use crossterm::event::KeyCode;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;

pub trait StateItem{
	fn display(&self, context: &mut TerminalContext);

	fn display_content(&self, context: &mut TerminalContext){
		context.draw_border();
		self.display(context);
	}

	fn register_input(&mut self, key_code: KeyCode);
	fn shutdown(&mut self){
		print!("{}[2J", 27 as char);
	}
	fn next_state(&self) -> Option<Transition>;
}