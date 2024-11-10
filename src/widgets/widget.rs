use crate::widgets::render_info::RenderInfo;

pub trait Widget{
	fn get_widget_position(&self) -> (u16, u16);
	fn draw(&self) -> Vec<RenderInfo>;
}