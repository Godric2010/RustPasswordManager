#[derive(Clone)]
pub enum Transition{
	ToAuthentication,
	ToChangeAuthentication,
	ToMainMenu,
	ToAddEntry,
	ToGetAccount,
	ToListEntries,
	ToExit,
}
