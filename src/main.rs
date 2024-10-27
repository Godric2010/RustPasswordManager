use crate::terminal_context::TerminalContext;
use crossterm::terminal::size;

mod state_item;
mod startup_state_item;
mod state_manager;
mod authentication_state_item;
mod transition;
mod main_menu_state_item;
mod add_account_state_item;
mod terminal_context;
mod set_authentication_state_item;
mod input_handler;
mod file_accesssor;
mod encryption_controller;
mod database_context;
mod list_accounts_state;
mod show_account_state_item;
mod wipe_database_state_item;
mod clipboard_controller;
mod page_list_view;
mod texts;
mod password_widget;
mod widget;

fn main() {
	println!("cargo:rustc-link-lib=sqlcipher");

	texts::init_texts();

	if let Some(mut context) = create_terminal_context() {
		let mut state_manager = state_manager::StateManager::new();
		state_manager.run(&mut context);
		context.destroy();
	}

}

fn create_terminal_context() -> Option<TerminalContext> {
	let (terminal_width, terminal_height) = size().expect("Could not read terminal size");
	let min_width: u16 = 60;
	let min_height: u16 = 20;
	let aspect_ratio = 3.0 / 1.0;

	if terminal_width < min_width || terminal_height < min_height {
		println!("Please resize the terminal to at least {}x{} for proper display!", min_width, min_height);
		return None;
	}

	let mut frame_width = (terminal_width as f32 * 0.75).min(terminal_width as f32) as u16;
	let mut frame_height = (frame_width as f32 / aspect_ratio).min(terminal_height as f32 - 2.0) as u16;

	if frame_height > terminal_height {
		frame_height = (terminal_height as f32 * 0.75).min(terminal_height as f32) as u16;
		frame_width = (frame_height as f32 * aspect_ratio).min(terminal_width as f32) as u16;
	}

	let origin_x = (terminal_width - frame_width) / 2;
	let origin_y = (terminal_height - frame_height) / 2;

	let context = TerminalContext::new(origin_x, origin_y, frame_width, frame_height);
	Some(context)
}
