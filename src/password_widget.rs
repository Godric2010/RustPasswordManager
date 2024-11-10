use rand::seq::SliceRandom;
use crate::terminal_context::TerminalContextOld;
use crate::texts::get_texts;
use crate::widget::Widget;

pub struct PasswordWidget {
	password_string: String,
	hidden_string: String,
	is_visible: bool,
}


impl Widget for PasswordWidget {
	fn display(&self, context: &mut TerminalContextOld, pos_x: u16, pos_y: u16) {
		if let Some(str) = self.get_password_to_display().clone() {
			context.print_at_position(pos_x, pos_y, str)
		}
	}

	fn display_as_footer(&self, contxt: &mut TerminalContextOld) {
		let heading = &get_texts().account.password;
		let content = if let Some(str) = self.get_password_to_display() {
			str.clone()
		} else {
			String::new()
		};
		contxt.draw_input_footer(heading, content)
	}
}

impl PasswordWidget {
	pub fn new(password_string: String) -> PasswordWidget {
		let hidden_string = build_new_hidden_string();
		PasswordWidget {
			password_string,
			hidden_string,
			is_visible: false,
		}
	}

	pub fn update_password(&mut self, password_string: String) {
		self.password_string = password_string;
		self.hidden_string = build_new_hidden_string();
	}

	pub fn change_visibility(&mut self, is_visible: bool) {
		self.is_visible = is_visible;
	}

	pub fn get_password_to_display(&self) -> Option<&String> {
		if self.is_visible {
			Some(&self.password_string)
		} else if self.password_string.len() > 0 {
			Some(&self.hidden_string)
		} else {
			None
		}
	}
}


fn build_new_hidden_string() -> String {
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
