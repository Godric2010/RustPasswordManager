use crate::views::view::View;

pub trait Controller<'a> {
	fn render(&'a self) -> Box<dyn View + 'a>;

	fn process_input(&mut self);
}