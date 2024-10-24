use crossterm::event::KeyCode;
use crate::state_item::StateItem;
use crate::terminal_context::{StyleAttribute, TerminalContext};
use crate::texts::get_texts;
use crate::transition::Transition;

pub struct MainMenuStateItem {
	next_state: Option<Transition>,
	menu_items: Vec<String>,
	menu_transitions: Vec<Transition>,
	selected_item: u8,
}

impl MainMenuStateItem {
	pub fn new() -> Self {
		let menu_items = get_texts().main_menu.get_menu_items();

		let menu_transitions = vec![
			Transition::ToAddAccount,
			Transition::ToListAccounts,
			Transition::ToChangeAuthentication,
			Transition::ToWipeDatabase,
			Transition::ToExit
		];

		MainMenuStateItem {
			next_state: None,
			menu_items,
			selected_item: 0,
			menu_transitions,
		}
	}
}

impl StateItem for MainMenuStateItem {
	fn display(&self, context: &mut TerminalContext) {
		let heading = &get_texts().main_menu.heading;

		let y_start_pos = context.get_height() / 2 - 4;
		let x_menu_pos = (context.get_width() - heading.len() as u16) / 2;

		context.print_at_position(x_menu_pos, y_start_pos, heading);
		for (index, text) in self.menu_items.iter().enumerate() {
			let content = text.clone();
			if self.selected_item == index as u8 {
				context.print_styled_at_position(0, y_start_pos + 2 + index as u16, content.as_str(), StyleAttribute::InverseColor);
			} else {
				context.print_at_position(0, y_start_pos + 2 + index as u16, content.as_str());
			}
		}

		let content = vec![&get_texts().input.down_arrow, &get_texts().input.up_arrow, &get_texts().input.enter];
		context.draw_control_footer(content);
	}

	fn update_display(&self) -> bool {
		false
	}

	fn register_input(&mut self, key_code: KeyCode) {
		match key_code {
			KeyCode::Char(c) => {
				if c.is_numeric() {
					let mut digit = c.to_digit(10).expect("Cannot convert char to digit");
					if digit >= self.menu_items.len() as u32 {
						digit = self.menu_items.len() as u32
					}
					self.selected_item = digit as u8 - 1;
				}
			}
			KeyCode::Enter => {
				let item_index = self.selected_item as usize;
				let transition = self.menu_transitions[item_index].clone();
				self.next_state = Some(transition);
			}
			KeyCode::Up => {
				if self.selected_item == 0 {
					self.selected_item = self.menu_items.len() as u8 - 1;
				} else {
					self.selected_item -= 1;
				}
			}
			KeyCode::Down => {
				if self.selected_item == self.menu_items.len() as u8 - 1 {
					self.selected_item = 0;
				} else {
					self.selected_item += 1;
				}
			}
			_ => (),
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}