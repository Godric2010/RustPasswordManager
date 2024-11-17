use std::rc::Rc;
use crate::views::view::View;
use crate::widgets::label::Label;
use crate::widgets::widget::Widget;

pub struct TestView {
	account_name: Rc<String>,
	widgets: Vec<Box<dyn Widget>>,
}

impl TestView {
	pub fn new(account_name: &String) -> Self {
		let account_name = Rc::new(account_name.clone());
		let account_name_widget = Label::new((0, 0), &account_name);

		TestView {
			account_name,
			widgets: vec![Box::new(account_name_widget)],
		}
	}
}

impl View for TestView {
	fn render(&self) -> &Vec<Box<dyn Widget>> {
		&self.widgets
	}
}