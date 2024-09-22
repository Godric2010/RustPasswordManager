use crate::clipboard_controller::ClipboardController;
use crate::database_context::{Account, DatabaseManager};
use crate::state_item::StateItem;
use crate::terminal_context::{StyleAttribute, TerminalContext};
use crate::transition::Transition;
use crossterm::event::KeyCode;
use std::sync::{Arc, Mutex};

enum ShowAccountState {
	ShowAccount,
	EditAccountName,
	EditPassword,
	EditEmail,
	CopyPassword,
	SaveChanges,
}

pub struct ShowAccountStateItem {
	account: Account,
	text_buffer: String,
	internal_state: Arc<Mutex<ShowAccountState>>,
	account_changed: bool,
	next_state: Option<Transition>,
	db_manager: Arc<Mutex<DatabaseManager>>,
	clipboard_controller: ClipboardController,
}

impl ShowAccountStateItem {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>, account: Account) -> Self {
		Self {
			account,
			text_buffer: String::new(),
			account_changed: false,
			internal_state: Arc::new(Mutex::new(ShowAccountState::ShowAccount)),
			db_manager,
			next_state: None,
			clipboard_controller: ClipboardController::new(),
		}
	}

	fn show_account(&self, context: &mut TerminalContext) {
		context.print_at_position(0, 2, "Name:");
		context.print_at_position(0, 3, &self.account.account_name);
		context.print_at_position(0, 5, "Email:");
		let email = match &self.account.email {
			Some(email) => email,
			None => "",
		};
		context.print_at_position(0, 6, &email);
		context.print_at_position(0, 8, "Password:");
		context.print_at_position(0, 9, &self.account.password);

		let bottom_position = context.get_height() - 1;
		context.print_at_position(0, bottom_position, "[E]dit  [C]opy password to clipboard  [Q]uit");
	}

	fn show_copy_password(&self, context: &mut TerminalContext) {
		context.print_at_position(0, 2, "Copied password to clipboard");
		let time_left = self.clipboard_controller.get_countdown_value();
		context.print_at_position(0,3, format!("Clearing clipboard in {}s", time_left).as_str())

	}

	fn show_save_changes(&self, context: &mut TerminalContext) {}

	fn show_edit_accountname(&self, context: &mut TerminalContext) {}

	fn show_edit_password_name(&self, context: &mut TerminalContext) {}

	fn show_edit_email(&self, context: &mut TerminalContext) {}



	// fn clipboard_countdown_thread(&mut self, duration: u8, tx: Sender<u8>) {
	// 	let mut ctx = ClipboardContext::new().unwrap();
	// 	ctx.set_contents(self.account.password.clone()).unwrap();
	// 	for i in (0..=duration).rev() {
	// 		tx.send(i).unwrap();
	// 		thread::sleep(Duration::from_secs(1));
	// 	}
	// 	ctx.set_contents("".to_string()).unwrap();
	// 	let mut state = self.next_state.lock().unwrap();
	// 	*state = Some(Transition::ToMainMenu);
	// }
}

impl StateItem for ShowAccountStateItem {
	fn display(&self, context: &mut TerminalContext) {
		context.print_styled_at_position(0, 0, "Account", StyleAttribute::Underline);

		let internal_state = self.internal_state.lock().unwrap();
		match &*internal_state {
			ShowAccountState::ShowAccount => self.show_account(context),
			ShowAccountState::EditAccountName => self.show_edit_accountname(context),
			ShowAccountState::EditPassword => self.show_edit_password_name(context),
			ShowAccountState::EditEmail => self.show_edit_email(context),
			ShowAccountState::CopyPassword => self.show_copy_password(context),
			ShowAccountState::SaveChanges => self.show_save_changes(context),
		};
	}

	fn register_input(&mut self, key_code: KeyCode) {

		match key_code {
			KeyCode::Char('c') => {
				self.internal_state = Arc::new(Mutex::new(ShowAccountState::CopyPassword));
				let state_ref = Arc::clone(&self.internal_state);
				self.clipboard_controller.copy_value_to_clipboard(&self.account.password, 5, move || {
					let mut state = state_ref.lock().unwrap();
					*state = ShowAccountState::ShowAccount;
				});

			}
			KeyCode::Char('q') => {
				 self.next_state = Some(Transition::ToMainMenu);
			}
			_ => {}
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}