use crate::views::view::View;
use crate::widgets::render_info::{RenderInfo, RenderStyle};
use crossterm::cursor::MoveTo;
use crossterm::style::{Print, Stylize};
use crossterm::terminal::ClearType;
use crossterm::{execute, queue, terminal};
use std::io::{stdout, Stdout, Write};

pub struct TerminalContext {
	stdout: Stdout,
	origin_x: u16,
	origin_y: u16,
	width: u16,
	height: u16,
}

impl TerminalContext {
	pub fn new(x: u16, y: u16, w: u16, h: u16) -> TerminalContext {
		let mut context = TerminalContext {
			stdout: stdout(),
			origin_x: x,
			origin_y: y,
			width: w,
			height: h,
		};
		context.clear();
		context
	}

	pub fn render_view(&mut self, view: Box<dyn View>) {
		self.clear();
		let widgets = view.render();
		for widget in widgets {
			let widget_pos = widget.get_widget_position();
			let elements = widget.draw();
			for element in elements {
				self.draw_widget_element(widget_pos, &element)
			}
		}
		self.stdout.flush().unwrap();
	}

	fn clear(&mut self) {
		execute!(self.stdout, terminal::Clear(ClearType::All)).unwrap();
	}

	fn draw_widget_element(&mut self, widget_pos: (u16, u16), element: &RenderInfo) {
		let element_x = self.origin_x + widget_pos.0 + element.pos_x;
		let element_y = self.origin_y + widget_pos.1 + element.pos_y;
		let position = (element_x, element_y);
		match element.style {
			RenderStyle::Default => {
				self.print_with_default_style(position, &element.content)
			}
			RenderStyle::Bold => {
				self.print_bold(position, &element.content);
			}
			RenderStyle::Underline => {
				self.print_underline(position, &element.content);
			}
			RenderStyle::Inverse => {
				self.print_inverse(position, &element.content);
			}
		}
	}

	fn print_with_default_style(&mut self, position: (u16, u16), content: &str) {
		queue!(self.stdout, MoveTo(position.0, position.1), Print(content)).unwrap();
	}

	fn print_underline(&mut self, position: (u16, u16), content: &str) {
		queue!(self.stdout, MoveTo(position.0, position.1), Print(content.underlined())).expect("Could not move cursor!");
	}

	fn print_bold(&mut self, position: (u16, u16), content: &str) {
		queue!(self.stdout, MoveTo(position.0, position.1), Print(content.bold())).expect("Could not move cursor!");
	}

	fn print_inverse(&mut self, position: (u16, u16), content: &str) {
		queue!(self.stdout, MoveTo(position.0, position.1), Print(content.negative())).expect("Could not move cursor!");
	}
}