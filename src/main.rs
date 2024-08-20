mod state_item;
mod startup_state_item;
mod state_manager;
mod authentication_state_item;
mod transition;
mod main_menu_state_item;

fn main() {

   let mut state_manager = state_manager::StateManager::new();
   state_manager.run();
}
