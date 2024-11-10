use crate::models::account_model::Account;
use crate::views::view::View;
use crate::widgets::label::Label;
use crate::widgets::widget::Widget;

pub struct TestView<'account> {
	account: &'account Account,
	widgets: Vec<Box<dyn Widget>>,
}

impl<'account> TestView<'account> {
	pub fn new(account: &'account Account) -> Self {
		let account_name_widget = Label::new((0, 0), &account.account_name.clone());

		TestView {
			account,
			widgets: vec![Box::new(account_name_widget)],
		}
	}
}

impl<'account> View for TestView<'account> {
	fn render(&self) -> &Vec<Box<dyn Widget>> {
		&self.widgets
	}
}