use crate::views::view::View;

pub trait Controller {
	fn render(&self) -> Box<dyn View>;

	fn process_input(&mut self);
}