use crate::database_context::{Account};

#[derive(Clone)]
pub enum Transition{
	ToAuthentication,
	ToChangeAuthentication,
	ToMainMenu,
	ToAddAccount,
	ToShowAccount(Account),
	ToListAccounts,
	ToWipeDatabase,
	ToExit,
}
