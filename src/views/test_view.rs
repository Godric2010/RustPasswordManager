use std::rc::Rc;
use crate::models::account_model::Account;
use crate::views::view::View;
use crate::widgets::label::Label;
use crate::widgets::widget::Widget;

pub struct TestView {
	account_name: Rc<String>,
	password: Rc<String>,
	widgets: Vec<Box<dyn Widget>>,
}

impl TestView {
	pub fn new(account: &Account) -> Self {
		let account_name = Rc::new(account.account_name.clone());
		let password = Rc::new(account.password.clone());
		let mut  account_name_widget = Label::new((0, 0), &account_name);
		account_name_widget.set_heading(String::from("Account name:"));
		let password_widget = Label::new((0, 2), &password);

		TestView {
			account_name,
			password,
			widgets: vec![Box::new(account_name_widget), Box::new(password_widget)],
		}
	}
}

impl View for TestView {
	fn render(&self) -> &Vec<Box<dyn Widget>> {
		&self.widgets
	}
}