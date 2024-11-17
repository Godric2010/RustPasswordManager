use std::cell::RefCell;
use crate::logic::controller::Controller;
use crate::models::account_model::Account;
use crate::views::test_view::TestView;
use crate::views::view::View;

pub struct TestController {
	account: RefCell<Account>,
}
impl TestController {
	pub fn new(model: &RefCell<Account>) -> Self {
		TestController {
			account: model.clone(),
		}
	}
}

impl Controller for TestController {
	fn render(&self) -> Box<dyn View> {
		let model_borrow = self.account.borrow();
		let view = TestView::new(&model_borrow.account_name);
		Box::new(view)
	}

	fn process_input(&mut self) {
		todo!()
	}
}