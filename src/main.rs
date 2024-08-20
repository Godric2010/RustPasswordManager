mod StateItem;
mod StartupStateItem;
pub mod StateManager;
mod AuthenticationStateItem;
mod Transition;
mod MainMenuStateItem;

fn main() {

   let mut state_manager = StateManager::StateManager::new();
   state_manager.run();
}
