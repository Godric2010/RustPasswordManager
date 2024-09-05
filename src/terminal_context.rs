use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, event, execute, terminal::{self, ClearType}, ExecutableCommand};
use std::io::{stdout, Stdout};

pub struct TerminalContext {
	stdout: Stdout,
	origin_x: u16,
	origin_y: u16,
	width: u16,
	height: u16,
}

impl TerminalContext {
	pub fn new(origin_x: u16, origin_y: u16, width: u16, height: u16) -> Self {
		let mut context = Self {
			stdout: stdout(),
			origin_x,
			origin_y,
			width,
			height,
		};
		context.clear_screen();
		context
	}

	pub fn destroy(&mut self) {
		self.clear_screen();
		match execute!(self.stdout, terminal::LeaveAlternateScreen, cursor::Show) {
			Ok(_) => (),
			Err(e) => panic!("Could not destroy context! {}", e),
		};
		let _ = disable_raw_mode();
	}

	pub fn get_width(&self) -> u16 {
		self.width
	}

	pub fn get_height(&self) -> u16 {
		self.height
	}

	pub fn clear_screen(&mut self) {
		execute!(self.stdout, terminal::Clear(ClearType::All)).expect("Failed to clear context");
	}
	pub fn print_at_position(&mut self, x: u16, y: u16, content: &str) {
		if x > self.width || y > self.height {
			panic!("Position exceeds context width or height!");
		}

		self.stdout.execute(cursor::MoveTo(/*self.origin_x +*/ x, /*self.origin_y +*/ y)).expect("Could not move cursor!");
		self.stdout.execute(Print(content)).expect("Could not print text!");
	}

	pub fn read_input(&mut self) -> Option<KeyCode> {
		if enable_raw_mode().is_err() {
			return None;
		}

		let key_code = match event::read() {
			Ok(event::Event::Key(KeyEvent { code, modifiers, .. })) => {
				match code {
					KeyCode::Char(c) => {
						if modifiers.contains(KeyModifiers::SHIFT) {
							Some(KeyCode::Char(c.to_ascii_uppercase()))
						} else {
							Some(KeyCode::Char(c.to_ascii_lowercase()))
						}
					}
					_ => Some(code)
				}
			}
			Ok(_) => None,
			Err(_) => None,
		};

		let _ = disable_raw_mode();
		key_code
	}
}