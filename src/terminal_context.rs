use crate::texts::get_texts;
use crossterm::cursor::{MoveTo, SetCursorStyle};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, event, execute, queue, terminal::{self, ClearType}, ExecutableCommand};
use rand::prelude::SliceRandom;
use std::io::{stdout, Stdout, Write};
use std::time::Duration;

pub enum StyleAttribute {
	Underline,
	Bold,
	InverseColor,
}

pub enum Visibility {
	Hidden,
	Visible(String),
}

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
		execute!(context.stdout, SetCursorStyle::BlinkingUnderScore).unwrap();
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

	pub fn draw_border(&mut self) {
		execute!(self.stdout, MoveTo(self.origin_x -1, self.origin_y -1)).unwrap();
		execute!(self.stdout, Print('┌')).unwrap();
		for _ in 0..self.width {
			execute!(self.stdout, Print('─')).unwrap();
		}
		execute!(self.stdout, Print('┐')).unwrap();

		execute!(self.stdout, MoveTo(self.origin_x -1, self.origin_y + self.height)).unwrap();
		execute!(self.stdout, Print('└')).unwrap();
		for _ in 0..self.width {
			execute!(self.stdout, Print('─')).unwrap();
		}
		execute!(self.stdout, Print('┘')).unwrap();

		for row in 0..self.height {
			execute!(self.stdout,MoveTo(self.origin_x - 1, self.origin_y + row),Print('│')).unwrap();
			execute!(self.stdout,MoveTo(self.origin_x + self.width, self.origin_y + row),Print('│')).unwrap();
		}
	}

	pub fn print_line(&mut self, x: u16, y: u16, width: u16) {
		execute!(self.stdout, MoveTo(self.origin_x + x, self.origin_y + y)).unwrap();
		for _ in 0..width {
			execute!(self.stdout, Print('─')).unwrap();
		}
		self.stdout.flush().unwrap();
	}

	pub fn draw_control_footer(&mut self, content: Vec<&String>) {
		let divider_y = self.height - 2;
		let content_y = self.height - 1;
		self.print_line(0, divider_y, self.width - 1);

		let mut content_string = String::new();
		for idx in 0..content.len() {
			content_string.insert_str(content_string.len(), content[idx].as_str());
			if idx < content.len() - 1 {
				content_string.insert(content_string.len(), '|');
			}
		}

		if content_string.len() > (self.width - 1) as usize {
			content_string = content_string.split_at((self.width - 1) as usize).0.to_string();
		}


		queue!(self.stdout, MoveTo(self.origin_x , self.origin_y + content_y), cursor::Hide, Print(content_string)).expect("Could not move cursor!");
		self.stdout.flush().unwrap();
	}

	pub fn draw_input_footer(&mut self, heading: &String, input_buffer: Visibility) {
		let divider_y = self.height - 3;
		let heading_y = self.height - 2;
		let input_buffer_y = self.height - 1;

		let input_buffer = match input_buffer {
			Visibility::Hidden => self.generate_hidden_pwd_string(),
			Visibility::Visible(str) => str,
		};

		self.print_line(0, divider_y, self.width - 1);
		queue!(self.stdout, MoveTo(self.origin_x, self.origin_y + heading_y), Print(heading)).expect("Could not execute queue!");
		queue!(self.stdout, MoveTo(self.origin_x, self.origin_y + input_buffer_y), Print(&input_buffer)).expect("Could not execute queue!");
		self.stdout.flush().unwrap();
		self.move_cursor_to_position(input_buffer.len() as u16, input_buffer_y);
	}

	pub fn draw_request_footer(&mut self, heading: &String) {
		let divider_y = self.height - 3;
		let heading_y = self.height - 2;
		let options_y = self.height - 1;

		self.print_line(0, divider_y, self.width - 1);
		queue!(self.stdout, MoveTo(self.origin_x, self.origin_y + heading_y), Print(heading)).expect("Could not execute queue!");
		queue!(self.stdout, MoveTo(self.origin_x, self.origin_y + options_y), Print(&get_texts().misc.confirm_input)).expect("Could not execute queue!");
		self.stdout.flush().unwrap();
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

		self.stdout.execute(MoveTo(self.origin_x + x, self.origin_y + y)).expect("Could not move cursor!");
		self.stdout.execute(cursor::Hide).expect("Could not hide cursor");
		self.stdout.execute(Print(content)).expect("Could not print text!");
	}

	pub fn print_styled_at_position(&mut self, x: u16, y: u16, content: &str, attribute: StyleAttribute) {
		if x > self.width || y > self.height {
			panic!("Position exceeds context width or height!");
		}

		let styled_content = match attribute {
			StyleAttribute::Underline => Print(content.underlined()),
			StyleAttribute::Bold => Print(content.bold()),
			StyleAttribute::InverseColor => Print(content.negative()),
		};

		queue!(self.stdout, MoveTo(self.origin_x + x, self.origin_y + y), cursor::Hide, styled_content).expect("Could not move cursor!");
		self.stdout.flush().expect("Could not flush stdout");
	}

	pub fn print_hidden_password_at_position(&mut self, pos_x: u16, pos_y: u16, pwd_len: usize) {
		if pwd_len == 0 {
			return;
		}
		let hidden_string = self.generate_hidden_pwd_string();
		self.print_at_position(pos_x, pos_y, &hidden_string)
	}

	fn generate_hidden_pwd_string(&self) -> String {
		let hide_chars = &get_texts().password.get_symbols();
		let length = 30;
		let mut rng = rand::thread_rng();

		let mut hidden_char_vec = vec![];
		for _ in 0..length {
			let special_char = *hide_chars.choose(&mut rng).unwrap();
			hidden_char_vec.push(special_char.clone());
		}
		let hidden_string: String = hidden_char_vec.into_iter().collect();
		hidden_string
	}

	pub fn move_cursor_to_position(&mut self, x: u16, y: u16) {
		self.stdout.execute(MoveTo(self.origin_x + x, self.origin_y + y)).expect("Could not move cursor");
		self.stdout.execute(cursor::Show).expect("Could not show cursor");
	}

	pub fn read_input(&mut self) -> Option<KeyCode> {
		if enable_raw_mode().is_err() {
			return None;
		}

		let mut key_code = None;
		if event::poll(Duration::from_millis(50)).unwrap() {
			key_code = match event::read() {
				Ok(event::Event::Key(KeyEvent { code, modifiers, .. })) => {
					match code {
						KeyCode::Char(c) => {
							let key_code = if modifiers.contains(KeyModifiers::SHIFT) {
								c.to_ascii_uppercase()
							} else {
								c.to_ascii_lowercase()
							};
							self.clear_input_buffer();
							Some(KeyCode::Char(key_code))
						}
						KeyCode::Enter => {
							self.clear_input_buffer();
							Some(KeyCode::Enter)
						}
						_ => {
							self.clear_input_buffer();
							Some(code)
						}
					}
				}
				Ok(_) => None,
				Err(_) => None,
			};
		}

		let _ = disable_raw_mode();
		key_code
	}

	fn clear_input_buffer(&mut self) {
		while event::poll(Duration::from_millis(50)).unwrap() {
			let _ = event::read();
		}
	}
}