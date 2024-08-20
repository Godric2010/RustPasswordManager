use crate::Transition::Transition;

pub trait StateItem{
	fn setup(&mut self);
	fn display(&self);
	fn register_input(&mut self);
	fn shutdown(&mut self){
		print!("{}[2J", 27 as char);
	}
	fn next_state(&self) -> Option<Transition>;
}