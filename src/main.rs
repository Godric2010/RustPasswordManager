use crate::terminal_context::TerminalContext;
use crossterm::terminal::size;

mod state_item;
mod startup_state_item;
mod state_manager;
mod authentication_state_item;
mod transition;
mod main_menu_state_item;
mod add_entry_state_item;
mod terminal_context;
mod set_authentication_state_item;
mod input_handler;
mod file_accesssor;
mod encryption_controller;
mod database_context;
mod list_entries_state;

fn main() {
	println!("cargo:rustc-link-lib=sqlcipher");
	let width = 100;
	let height = 10;

	let (size_x, size_y) = size().expect("Could not read terminal size");

	let origin_x = (size_x - width) / 2;
	let origin_y = (size_y - height) / 2;

	let mut context = TerminalContext::new(origin_x, origin_y, width, height);
	let mut state_manager = state_manager::StateManager::new();
	state_manager.run(&mut context);

	context.destroy();
}
