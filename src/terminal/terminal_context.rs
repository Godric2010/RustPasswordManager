use std::io::{stdout, Stdout, Write};
use crossterm::{execute, queue, terminal};
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::ClearType;
use crate::views::view::View;

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

	pub fn render_view(&mut self, view: Box<dyn View >) {
		self.clear();
		let widgets = view.render();
		for widget in widgets {
			let (x, y) = widget.get_widget_position();
			let elements = widget.draw();
			for element in elements {
				let element_x = self.origin_x + x + element.pos_x;
				let element_y = self.origin_y + y + element.pos_y;
				queue!(self.stdout, MoveTo(element_x, element_y), Print(element.content)).unwrap();
			}
		}
		self.stdout.flush().unwrap();
	}

	fn clear(&mut self) {
		execute!(self.stdout, terminal::Clear(ClearType::All)).unwrap();
	}
}