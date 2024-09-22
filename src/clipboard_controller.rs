use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use clipboard::{ClipboardContext, ClipboardProvider};

pub struct ClipboardController {
	clipboard: Arc<Mutex<ClipboardContext>>,
	countdown_value: Arc<Mutex<u8>>,
}

impl ClipboardController {
	pub fn new() -> Self {
		ClipboardController {
			clipboard: Arc::new(Mutex::new(ClipboardProvider::new().unwrap())),
			countdown_value: Arc::new(Mutex::new(0)),
		}
	}

	pub fn copy_value_to_clipboard<F>(&mut self, content: &str, time_available: u8, on_complete: F)
	where
		F: FnOnce() + Send + 'static,
	{
		self.set_clipboard_context(content);

		*self.countdown_value.lock().unwrap() = time_available;

		let clipboard_clone = Arc::clone(&self.clipboard);
		let countdown_clone = Arc::clone(&self.countdown_value);
		thread::spawn(move || {
			let mut duration = time_available;

			while duration > 0 {
				duration -= 1;
				thread::sleep(Duration::from_secs(1));
				*countdown_clone.lock().unwrap() = duration;
			}

			let mut clipboard = clipboard_clone.lock().unwrap();
			clipboard.set_contents("".to_string()).unwrap();
			on_complete();
		});
	}

	fn set_clipboard_context(&mut self, content: &str){
		let mut clipboard = self.clipboard.lock().unwrap();
		clipboard.set_contents(content.to_string()).unwrap();
	}

	pub fn get_countdown_value(&self) -> u8{
		*self.countdown_value.lock().unwrap()
	}
}