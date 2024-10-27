use crate::clipboard_controller::ClipboardController;
use crate::database_context::{Account, DatabaseManager};
use crate::input_handler::evaluate_yes_no_answer;
use crate::state_item::StateItem;
use crate::terminal_context::{StyleAttribute, TerminalContext};
use crate::transition::Transition;
use crossterm::event::KeyCode;
use std::cmp::PartialEq;
use std::sync::{Arc, Mutex};
use crate::password_widget::PasswordWidget;
use crate::texts::get_texts;
use crate::widget::Widget;

#[derive(PartialEq)]
enum ShowAccountState {
	ShowAccount,
	EditAccountName,
	EditPassword,
	EditEmail,
	DeleteAccount,
	CopyPassword,
	SaveChanges,
}

pub struct ShowAccountStateItem {
	account: Account,
	internal_state: Arc<Mutex<ShowAccountState>>,
	next_state: Option<Transition>,
	db_manager: Arc<Mutex<DatabaseManager>>,
	clipboard_controller: ClipboardController,
	password_widget: PasswordWidget,
}

impl ShowAccountStateItem {
	pub fn new(db_manager: Arc<Mutex<DatabaseManager>>, account: Account) -> Self {
		Self {
			password_widget: PasswordWidget::new(account.password.clone()),
			account,
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

		context.draw_control_footer(vec![&get_texts().show_account.edit_input, &get_texts().show_account.delete_input, &get_texts().show_account.copy_input, &get_texts().show_account.quit_input])
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
			KeyCode::Char('d') => {
				self.internal_state = Arc::new(Mutex::new(ShowAccountState::DeleteAccount));
			}
			_ => {}
		}
	}

	fn show_copy_password(&self, context: &mut TerminalContext) {
		self.show_account_name(context, false);
		self.show_email(context, false);
		self.show_password(context, false);


		let time_left = self.clipboard_controller.get_countdown_value();
		if self.clipboard_controller.get_countdown_duration() == time_left {
			context.draw_control_footer(vec![&get_texts().show_account.copy_msg])
		} else {
			let countdown = format!("{} {}s", &get_texts().show_account.copy_countdown, time_left);
			context.draw_control_footer(vec![&countdown])
		};
	}

	fn show_save_changes(&self, context: &mut TerminalContext) {
		self.show_account_name(context, false);
		self.show_email(context, false);
		self.show_password(context, false);

		context.draw_request_footer(&get_texts().show_account.save_question);
	}

	fn show_save_changes_input(&mut self, key_code: KeyCode) {
		if let Some(accept) = evaluate_yes_no_answer(key_code) {
			let database_manager = self.db_manager.lock().unwrap();
			let db_context = match database_manager.get_database_context() {
				Some(context) => context,
				None => return,
			};
			if accept {
				db_context.update_account(&self.account);
				database_manager.safe_database();
			} else {
				let account_result = db_context.get_account_by_id(self.account.id);
				let account_optional = match account_result {
					Ok(result) => result,
					Err(e) => panic!("Fetching account failed! {}", e.to_string()),
				};
				let account = match account_optional {
					Some(account) => account,
					None => panic!("Account id was unknown!"),
				};
				self.account = account;
			}
			self.internal_state = Arc::new(Mutex::new(ShowAccountState::ShowAccount));
		}
	}

	fn show_edit_accountname(&self, context: &mut TerminalContext) {
		self.show_account_name(context, true);
		self.show_email(context, false);
		self.show_password(context, false);

		let content = vec![&get_texts().input.down_arrow, &get_texts().input.up_arrow, &get_texts().input.enter];
		context.draw_control_footer(content);
		context.move_cursor_to_position(self.account.account_name.len() as u16, 3);
	}

	fn show_edit_accountname_input(&mut self, key_code: KeyCode) {
		let mut account_name = self.account.account_name.clone();
		self.edit_account_input(key_code, &mut account_name, ShowAccountState::EditEmail, ShowAccountState::EditPassword);
		self.account.account_name = account_name;
	}

	fn show_edit_password_name(&self, context: &mut TerminalContext) {
		self.show_account_name(context, false);
		self.show_email(context, false);
		self.show_password(context, true);

		let content = vec![&get_texts().input.down_arrow, &get_texts().input.up_arrow, &get_texts().input.enter];
		context.draw_control_footer(content);
		context.move_cursor_to_position(0, 9);
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

		let content = vec![&get_texts().input.down_arrow, &get_texts().input.up_arrow, &get_texts().input.enter];
		context.draw_control_footer(content);
		if let Some(email) = &self.account.email {
			context.move_cursor_to_position(email.len() as u16, 6);
		}
	}

	fn show_edit_email_input(&mut self, key_code: KeyCode) {
		let mut email = match &self.account.email {
			Some(email) => email.clone(),
			None => "".to_string(),
		};

		self.edit_account_input(key_code, &mut email, ShowAccountState::EditPassword, ShowAccountState::EditAccountName);
		self.account.email = Some(email);
	}

	fn show_delete_account(&self, context: &mut TerminalContext) {
		self.show_account_name(context, false);
		self.show_email(context, false);
		self.show_password(context, false);

		context.draw_request_footer(&get_texts().show_account.delete_question);
	}

	fn show_delete_account_input(&mut self, key_code: KeyCode) {
		if let Some(accept) = evaluate_yes_no_answer(key_code) {
			let database_manager = self.db_manager.lock().unwrap();
			let db_context = match database_manager.get_database_context() {
				Some(context) => context,
				None => return,
			};
			if accept {
				db_context.remove_account(self.account.id).unwrap();
				database_manager.safe_database();
				self.next_state = Some(Transition::ToMainMenu);
				return;
			}
			self.internal_state = Arc::new(Mutex::new(ShowAccountState::ShowAccount));
		}
	}

	fn show_account_name(&self, context: &mut TerminalContext, highlighted: bool) {
		if highlighted {
			context.print_styled_at_position(0, 2, &get_texts().account.account_name, StyleAttribute::Bold);
			context.print_styled_at_position(0, 3, &self.account.account_name, StyleAttribute::InverseColor);
		} else {
			context.print_at_position(0, 2, &get_texts().account.account_name);
			context.print_at_position(0, 3, &self.account.account_name);
		}
	}

	fn show_email(&self, context: &mut TerminalContext, highlighted: bool) {
		let email = match &self.account.email {
			Some(email) => email,
			None => "",
		};
		if highlighted {
			context.print_styled_at_position(0, 5, &get_texts().account.email, StyleAttribute::Bold);
			context.print_styled_at_position(0, 6, &email, StyleAttribute::InverseColor);
		} else {
			context.print_at_position(0, 5, &get_texts().account.email);
			context.print_at_position(0, 6, &email);
		}
	}

	fn show_password(&self, context: &mut TerminalContext, highlighted: bool) {
		if highlighted {
			context.print_styled_at_position(0, 8, &get_texts().account.password, StyleAttribute::Bold);
			self.password_widget.display(context, 0, 9);
		} else {
			context.print_at_position(0, 8, &get_texts().account.password);
			self.password_widget.display(context, 0, 9)
		}
	}

	fn edit_account_input(&mut self, key_code: KeyCode, text_buffer: &mut String, next_state: ShowAccountState, prev_state: ShowAccountState) {
		match key_code {
			KeyCode::Enter => { self.change_internal_state(ShowAccountState::SaveChanges) }
			KeyCode::Backspace => { text_buffer.pop(); }
			KeyCode::Char(c) => text_buffer.push(c),
			KeyCode::Up => { self.change_internal_state(prev_state) }
			KeyCode::Down => { self.change_internal_state(next_state) }

			_ => (),
		};
	}

	fn change_internal_state(&mut self, new_state: ShowAccountState) {
		self.password_widget.change_visibility(new_state == ShowAccountState::EditPassword);
		self.internal_state = Arc::new(Mutex::new(new_state));
	}
}

impl StateItem for ShowAccountStateItem {
	fn display(&self, context: &mut TerminalContext) {
		context.print_styled_at_position(0, 0, &get_texts().show_account.heading, StyleAttribute::Underline);

		let internal_state = self.internal_state.lock().unwrap();
		match &*internal_state {
			ShowAccountState::ShowAccount => self.show_account(context),
			ShowAccountState::EditAccountName => self.show_edit_accountname(context),
			ShowAccountState::EditPassword => self.show_edit_password_name(context),
			ShowAccountState::EditEmail => self.show_edit_email(context),
			ShowAccountState::CopyPassword => self.show_copy_password(context),
			ShowAccountState::SaveChanges => self.show_save_changes(context),
			ShowAccountState::DeleteAccount => self.show_delete_account(context),
		};
	}

	fn update_display(&self) -> bool {
		let internal_state = self.internal_state.lock().unwrap();
		if &*internal_state == &ShowAccountState::CopyPassword {
			return true;
		}
		false
	}

	fn register_input(&mut self, key_code: KeyCode) {
		let state_clone = Arc::clone(&self.internal_state);
		let internal_state = state_clone.lock().unwrap();
		match &*internal_state {
			ShowAccountState::ShowAccount => self.show_account_input(key_code),
			ShowAccountState::EditAccountName => self.show_edit_accountname_input(key_code),
			ShowAccountState::EditPassword => self.show_edit_password_input(key_code),
			ShowAccountState::EditEmail => self.show_edit_email_input(key_code),
			ShowAccountState::CopyPassword => {}
			ShowAccountState::SaveChanges => self.show_save_changes_input(key_code),
			ShowAccountState::DeleteAccount => self.show_delete_account_input(key_code),
		}
	}

	fn next_state(&self) -> Option<Transition> {
		self.next_state.clone()
	}
}