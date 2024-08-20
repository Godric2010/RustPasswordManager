#[derive(Clone)]
pub enum Transition{
	ToAuthentication,
	ToMainMenu,
	ToAddEntry,
	ToSearchEntry,
	ToListEntries,
	ToExit,
}
