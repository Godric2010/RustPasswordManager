use std::fs;
use std::path::PathBuf;
use directories::BaseDirs;

fn get_base_dir() -> PathBuf {
	let base_dirs = BaseDirs::new().expect("Could not determine home directory");
	base_dirs.home_dir().join("RustyPasswordManager")
}

fn get_db_file_path() -> PathBuf {
	get_base_dir().join("rpm.db")
}

fn get_password_file_path() -> PathBuf {
	get_base_dir().join("pwd.key")
}

pub fn does_directory_and_files_exist() -> bool {
	let base_dir = get_base_dir();
	if !base_dir.exists() {
		return false;
	}

	let db_file_path = get_db_file_path();
	if !db_file_path.exists() {
		return false;
	}

	let pwd_file_path = get_password_file_path();
	if !pwd_file_path.exists() {
		return false;
	}
	true
}

pub fn delete_directory_and_files() {
	if get_base_dir().exists() {
		fs::remove_dir_all(get_base_dir().as_path()).expect("Could not delete RustyPasswordManager dir");
		return;
	}
}

pub fn write_password_to_disk(pwd_cipher: String) {
	fs::write(&get_password_file_path(), pwd_cipher).expect("Failed to write password file");
}

pub fn write_db_to_disk(db_cipher: Vec<u8>) {
	fs::write(&get_db_file_path(), db_cipher).expect("Failed to write db file");
}

pub fn read_password_from_disk() -> Option<String> {
	let data = match fs::read(&get_password_file_path()) {
		Ok(bytes) => bytes,
		Err(_) => return None,
	};

	let string = String::from_utf8(data).unwrap();
	Some(string)
}

pub fn read_db_from_disk() -> Option<Vec<u8>> {
	match fs::read(&get_db_file_path()) {
		Ok(bytes) => Some(bytes),
		Err(_) => None,
	}
}

pub fn create_directory_and_files(db_cipher: Vec<u8>, pwd_cipher: String) {
	if does_directory_and_files_exist() {
		delete_directory_and_files();
	}

	fs::create_dir(get_base_dir().as_path()).expect("Could not create RustyPasswordManager dir");
	write_password_to_disk(pwd_cipher);
	write_db_to_disk(db_cipher);
}