use crate::widgets::widget::Widget;

pub trait View{
	fn render(&self) -> &Vec<Box<dyn Widget>>;
}