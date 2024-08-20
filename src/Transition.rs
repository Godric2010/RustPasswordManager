#[derive(Clone)]
pub enum Transition{
	ToStartup,
	ToAuthentication,
	ToMainMenu,
	ToAddEntry,
	ToSearchEntry,
	ToListEntries,
	ToExit,
}
