use crossterm::event::KeyCode;

pub fn get_text_input(key_code: KeyCode, text_buffer: &mut String) -> bool {
	match key_code {
		KeyCode::Enter => return true,
		KeyCode::Backspace => { text_buffer.pop(); }
		KeyCode::Char(c) => text_buffer.push(c),

		_ => (),
	};

	false
}

pub fn get_enter_press(key_code: KeyCode) -> bool {
	if key_code == KeyCode::Enter {
		return true;
	};
	false
}

pub fn evaluate_yes_no_answer(key_code: KeyCode) -> Option<bool> {
	match key_code {
		KeyCode::Char(c) => {
			let char_lower = c.to_ascii_lowercase();
			if char_lower == 'y' {
				return Some(true);
			} else if char_lower == 'n' {
				return Some(false)
			}
			None
		}
		_ => None
	}
}
