use crate::clipboard_controller::ClipboardController;
use crate::database_context::{Account, DatabaseManager};
use crate::input_handler::evaluate_yes_no_answer;
use crate::state_item::StateItem;
use crate::terminal_context::{StyleAttribute, TerminalContext};
use crate::transition::Transition;
use crossterm::event::KeyCode;
use std::cmp::PartialEq;
use std::sync::{Arc, Mutex};

#[derive(PartialEq)]
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
		self.show_account_name(context, false);
		self.show_email(context, false);
		self.show_password(context, false);

		let bottom_position = context.get_height() - 1;
		context.print_at_position(0, bottom_position, "[E]dit  [C]opy password to clipboard  [Q]uit");
	}

	fn show_account_input(&mut self, key_code: KeyCode) {
		match key_code {
			KeyCode::Char('c') => {
				self.internal_state = Arc::new(Mutex::new(ShowAccountState::CopyPassword));
				let state_ref = Arc::clone(&self.internal_state);
				self.clipboard_controller.copy_value_to_clipboard(&self.account.password, 30, move || {
					let mut state = state_ref.lock().unwrap();
					*state = ShowAccountState::ShowAccount;
				});
			}
			KeyCode::Char('e') => {
				self.internal_state = Arc::new(Mutex::new(ShowAccountState::EditAccountName));
			}
			KeyCode::Char('q') => {
				self.next_state = Some(Transition::ToMainMenu);
			}
			_ => {}
		}
	}

	fn show_copy_password(&self, context: &mut TerminalContext) {
		self.show_account_name(context, false);
		self.show_email(context, false);
		self.show_password(context, false);


		let bottom_position = context.get_height() - 1;
		let time_left = self.clipboard_controller.get_countdown_value();
		if self.clipboard_controller.get_countdown_duration() == time_left {
			context.print_styled_at_position(0, bottom_position, "Copied password to clipboard", StyleAttribute::InverseColor);
		} else {
			context.print_styled_at_position(0, bottom_position, format!("Clearing clipboard in {}s", time_left).as_str(), StyleAttribute::InverseColor);
		}
	}

	fn show_save_changes(&self, context: &mut TerminalContext) {
		self.show_account_name(context, false);
		self.show_email(context, false);
		self.show_password(context, false);


		let bottom_position = context.get_height() - 1;
		context.print_styled_at_position(0, bottom_position, "Save changes? [Y]es [N]o", StyleAttribute::InverseColor);
	}

	fn show_save_changes_input(&mut self, key_code: KeyCode) {
		if let Some(accept) = evaluate_yes_no_answer(key_code) {
			if accept {
				self.internal_state = Arc::new(Mutex::new(ShowAccountState::ShowAccount));
			} else {}
			self.internal_state = Arc::new(Mutex::new(ShowAccountState::ShowAccount));
		}
	}

	fn show_edit_accountname(&self, context: &mut TerminalContext) {
		self.show_account_name(context, true);
		self.show_email(context, false);
		self.show_password(context, false);
	}

	fn show_edit_accoutname_input(&mut self, key_code: KeyCode) {
		let mut account_name = self.account.account_name.clone();
		self.edit_account_input(key_code, &mut account_name, ShowAccountState::EditEmail, ShowAccountState::EditPassword);
		self.account.account_name = account_name;
	}

	fn show_edit_password_name(&self, context: &mut TerminalContext) {
		self.show_account_name(context, false);
		self.show_email(context, false);
		self.show_password(context, true);
	}

	fn show_edit_password_input(&mut self, key_code: KeyCode)
	{
		let mut password = self.account.password.clone();
		self.edit_account_input(key_code, &mut password, ShowAccountState::EditAccountName, ShowAccountState::EditEmail);
		self.account.password = password;
	}

	fn show_edit_email(&self, context: &mut TerminalContext) {
		self.show_account_name(context, false);
		self.show_email(context, true);
		self.show_password(context, false);
	}

	fn show_edit_email_input(&mut self, key_code: KeyCode){
		let mut email = match &self.account.email {
			Some(email) => email.clone(),
			None => "".to_string(),
		};

		self.edit_account_input(key_code, &mut email, ShowAccountState::EditPassword, ShowAccountState::EditAccountName );
		self.account.email = Some(email);
	}

	fn show_account_name(&self, context: &mut TerminalContext, highlighted: bool) {
		if highlighted {
			context.print_styled_at_position(0, 2, "Name:", StyleAttribute::Bold);
			context.print_styled_at_position(0, 3, &self.account.account_name, StyleAttribute::InverseColor);
		} else {
			context.print_at_position(0, 2, "Name:");
			context.print_at_position(0, 3, &self.account.account_name);
		}
	}

	fn show_email(&self, context: &mut TerminalContext, highlighted: bool) {
		let email = match &self.account.email {
			Some(email) => email,
			None => "",
		};
		if highlighted {
			context.print_styled_at_position(0, 5, "Email:", StyleAttribute::Bold);
			context.print_styled_at_position(0, 6, &email, StyleAttribute::InverseColor);
		} else {
			context.print_at_position(0, 5, "Email:");
			context.print_at_position(0, 6, &email);
		}
	}

	fn show_password(&self, context: &mut TerminalContext, highlighted: bool) {
		if highlighted {
			context.print_styled_at_position(0, 8, "Password:", StyleAttribute::Bold);
			context.print_styled_at_position(0, 9, &self.account.password, StyleAttribute::InverseColor);
		} else {
			context.print_at_position(0, 8, "Password:");
			context.print_at_position(0, 9, &self.account.password);
		}
	}

	fn edit_account_input(&mut self, key_code: KeyCode, text_buffer: &mut String, next_state: ShowAccountState, prev_state: ShowAccountState) {
		match key_code {
			KeyCode::Enter => { self.internal_state = Arc::new(Mutex::new(ShowAccountState::SaveChanges)) }
			KeyCode::Backspace => { text_buffer.pop(); }
			KeyCode::Char(c) => text_buffer.push(c),
			KeyCode::Up => { self.internal_state = Arc::new(Mutex::new(prev_state)) }
			KeyCode::Down => { self.internal_state = Arc::new(Mutex::new(next_state)) }

			_ => (),
		};
	}
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
		let state_clone = Arc::clone(&self.internal_state);
		let internal_state = state_clone.lock().unwrap();
		match &*internal_state {
			ShowAccountState::ShowAccount => self.show_account_input(key_code),
			ShowAccountState::EditAccountName => self.show_edit_accoutname_input(key_code),
			ShowAccountState::EditPassword => self.show_edit_password_input(key_code),
			ShowAccountState::EditEmail => self.show_edit_email_input(key_code),
			ShowAccountState::CopyPassword => {}
			ShowAccountState::SaveChanges => self.show_save_changes_input(key_code)
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}