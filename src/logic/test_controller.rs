use crate::logic::controller::Controller;
use crate::models::account_model::Account;
use crate::views::test_view::TestView;
use crate::views::view::View;

pub struct TestController<'account> {
	account: &'account mut Account,
}
impl<'account> TestController<'account> {
	pub fn new(model: &'account mut Account) -> Self {
		TestController {
			account: model,
		}
	}
}

impl<'account> Controller<'account> for TestController<'account> {
	fn render(&'account self) -> Box<dyn View + 'account> {
		let view = TestView::new(&self.account);
		Box::new(view)
	}

	fn process_input(&mut self) {
		todo!()
	}
}