use std::sync::{Arc, Mutex};
use crate::add_account_state_item::AddEntryStateItem;
use crate::authentication_state_item::AuthenticationStateItem;
use crate::database_context::DatabaseManager;
use crate::list_accounts_state::ListAccountsState;
use crate::main_menu_state_item::MainMenuStateItem;
use crate::set_authentication_state_item::SetAuthenticationStateItem;
use crate::show_account_state_item::ShowAccountStateItem;
use crate::startup_state_item::StartupStateItem;
use crate::state_item::StateItem;
use crate::terminal_context::TerminalContext;
use crate::transition::Transition;
use crate::wipe_database_state_item::WipeDatabaseStateItem;

pub struct StateManager {
	state: Option<Box<dyn StateItem>>,
	db_manager: Arc<Mutex<DatabaseManager>>,
	active: bool,
}

impl StateManager {
	pub fn new() -> Self {
		StateManager {
			state: Some(Box::new(StartupStateItem::new())),
			db_manager: Arc::new(Mutex::new(DatabaseManager::new())),
			active: true,
		}
	}


	fn transition_to(&mut self, next_state: Box<dyn StateItem>) {
		if let Some(state) = &mut self.state {
			state.shutdown();
		}
		self.state = Some(next_state);
	}

	pub fn run(&mut self, context: &mut TerminalContext) {
		loop {
			if !self.active {
				break;
			}

			if let Some(state) = &mut self.state {
				context.clear_screen();
				state.display(context);
				if let Some(transition) = state.next_state() {
					self.transition(transition);
					continue;
				}
				if let Some(key_code) = context.read_input() {
					state.register_input(key_code);
				}
			}
		}
	}

	fn transition(&mut self, transition: Transition) {
		match transition {
			Transition::ToAuthentication => self.transition_to(Box::new(AuthenticationStateItem::new(Arc::clone(&self.db_manager)))),
			Transition::ToAddAccount => self.transition_to(Box::new(AddEntryStateItem::new(Arc::clone(&self.db_manager)))),
			Transition::ToListAccounts => self.transition_to(Box::new(ListAccountsState::new(Arc::clone(&self.db_manager)))),
			Transition::ToChangeAuthentication => self.transition_to(Box::new(SetAuthenticationStateItem::new())),
			Transition::ToShowAccount(account) => self.transition_to(Box::new(ShowAccountStateItem::new(Arc::clone(&self.db_manager), account))),
			Transition::ToMainMenu => self.transition_to(Box::new(MainMenuStateItem::new())),
			Transition::ToWipeDatabase => self.transition_to(Box::new(WipeDatabaseStateItem::new())),
			Transition::ToExit => self.active = false,
		}
	}
}
