use std::rc::Rc;
use crate::widgets::render_info::{RenderInfo, RenderStyle};
use crate::widgets::widget::Widget;

pub struct Label{
	content: Rc<String>,
	heading: Option<String>,
	style: RenderStyle,
	pos_x: u16,
	pos_y: u16,
}

impl Label{
	pub fn new(position: (u16, u16), content: &Rc<String>) -> Label{
		Label{
			pos_x: position.0,
			pos_y: position.1,
			content: Rc::clone(content),
			heading: None,
			style: RenderStyle::Default,
		}
	}

	pub fn set_heading(&mut self, heading: String){
		self.heading = Some(heading.clone())
	}

	pub fn set_style(&mut self, style: RenderStyle){
		self.style = style;
	}
}

impl Widget for Label{
	fn get_widget_position(&self) -> (u16, u16) {
		(self.pos_x, self.pos_y)
	}

	fn draw(&self) -> Vec<RenderInfo> {
		let mut render_infos: Vec<RenderInfo> = vec![];

		let mut content_y = 1u16;
		if let Some(heading) = &self.heading{
			let heading_info = RenderInfo {
				pos_x: 1,
				pos_y: 1,
				content: heading.clone(),
				style: RenderStyle::Bold
			};
			content_y +=1;

			render_infos.push(heading_info);
		}

		let content_info = RenderInfo{
			pos_x: 1,
			pos_y: content_y,
			content: (*self.content).clone(),
			style: self.style.clone(),
		};
		render_infos.push(content_info);
		render_infos
	}
}
