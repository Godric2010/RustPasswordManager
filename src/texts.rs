use once_cell::sync::OnceCell;
use std::fs;
use serde::Deserialize;
use toml::from_str;

#[derive(Deserialize, Debug)]
pub struct Misc {
	pub welcome: String,
	pub confirm_input: String,
}

#[derive(Deserialize, Debug)]
pub struct Input{
	pub up_arrow: String,
	pub down_arrow: String,
	pub left_arrow: String,
	pub right_arrow: String,
	pub enter: String,
	pub escape: String,
}

#[derive(Deserialize, Debug)]
pub struct MainMenu {
	pub heading: String,
	add_account: String,
	list_accounts: String,
	set_master_pwd: String,
	wipe_database: String,
	exit: String,
}

impl MainMenu {
	pub fn get_menu_items(&self) -> Vec<String> {
		vec![self.add_account.clone(), self.list_accounts.clone(), self.set_master_pwd.clone(), self.wipe_database.clone(), self.exit.clone()]
	}
}

#[derive(Deserialize, Debug)]
pub struct Account{
	pub account_name: String,
	pub email: String,
	pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct AddAccount {
	pub heading: String,
	pub account_exists: String,
	pub add_email_question: String,
	pub generate_pwd_question: String,
	pub pwd_generated: String,
	pub pwd_set: String,
	pub cancel_question: String,
}

#[derive(Deserialize, Debug)]
pub struct ListAccounts{
	pub heading: String,
	pub search: String,
	pub search_input: String,
	pub quit_input: String,
}

#[derive(Deserialize, Debug)]
pub struct ShowAccount{
	pub heading: String,
	pub delete_question: String,
	pub save_question: String,
	pub copy_msg: String,
	pub copy_countdown: String,
	pub copy_input: String,
	pub edit_input: String,
	pub delete_input: String,
	pub quit_input: String,
}

#[derive(Deserialize, Debug)]
pub struct Auth{
	pub enter_pwd_promt:String,
	pub invalid_pwd: String,
	pub valid_pwd: String,
	pub set_new_master_pwd: String,
	pub confirm_new_master_pwd: String,
	pub master_password_set: String,
	pub confirm_failed: String,
	pub cancel_question: String,
}

#[derive(Deserialize, Debug)]
pub struct Wipe{
	pub are_you_sure_question: String,
	pub warning: String,
	pub delete_msg: String,
	pub enter_pwd_request: String,
	pub success_msg: String,
	pub failure_msg: String,
}


#[derive(Deserialize, Debug)]
pub struct Texts {
	pub misc: Misc,
	pub input: Input,
	pub main_menu: MainMenu,
	pub account: Account,
	pub add_account: AddAccount,
	pub list_accounts: ListAccounts,
	pub show_account: ShowAccount,
	pub auth: Auth,
	pub wipe: Wipe,
}

pub fn load_texts() -> Texts {
	let file_content = fs::read_to_string("texts_eng.toml").expect("Unable to read text files");
	from_str(&file_content).expect("Error parsing TOML")
}

static TEXTS: OnceCell<Texts> = OnceCell::new();

pub fn init_texts() {
	TEXTS.set(load_texts()).unwrap()
}

pub fn get_texts() -> &'static Texts {
	TEXTS.get().expect("Texts not initialized!")
}