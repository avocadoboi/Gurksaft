
// use crate::learning_data::{LearningData, LearningTask, FinishedTask};
use crate::options::{Options, WeightFactors};
use crate::source_data::{Language, LANGUAGES};

use std::sync::Mutex;

use tauri::{Manager, State};

//----------------------------------------------------------------

// struct App {
// 	options: Mutex<Options>,
// 	learning_data: Mutex<LearningData>,
// }

// impl App {
	// fn new() -> Self {
	// 	App {
	// 		options: Mutex::new(Options::load()),
	// 		learning_data: Mutex::new(None),
	// 	}
	// }
// 	fn save(&self) {
// 		self.options.lock().unwrap().save();
// 	}
// }

// #[tauri::command]
// fn next_task(app: State<App>) -> LearningTask {
// 	app.learning_data.lock().unwrap().next_task()
// }

// #[tauri::command]
// fn finish_task(app: State<App>, task: FinishedTask) {
// 	app.learning_data.lock().unwrap().finish_task(task)
// }

// #[tauri::command]
// fn get_weight_factors(app: State<App>) -> WeightFactors {
// 	app.options.lock().unwrap().weight_factors
// }

// #[tauri::command]
// fn set_success_weight_factor(app: State<App>, factor: f64) {
// 	app.learning_data.lock().unwrap().weight_factors.succeeded = factor;
// }
// #[tauri::command]
// fn set_failure_weight_factor(app: State<App>, factor: f64) {
// 	app.learning_data.lock().unwrap().weight_factors.failed = factor;
// }

#[tauri::command]
fn get_language_list() -> &'static [Language] {
	&LANGUAGES
}

pub fn run() {
	let options = Options::load();

	tauri::Builder::default()
		.setup(move |app| {
			if options.is_none() {
				let window = tauri::WindowBuilder::new(app, "add_language", tauri::WindowUrl::App("add_language/index.html".into()))
					.title("Add language")
					.build().unwrap();
				// window.once("download_language_data", |event| {
				// 	event.
				// });
			}
			Ok(())
		})
		// .manage()
		.invoke_handler(tauri::generate_handler![
			get_language_list
			// next_task, 
			// finish_task,
			// get_weight_factors,
			// set_success_weight_factor,
			// set_failure_weight_factor
		])
		// .on_window_event(|event| if let tauri::WindowEvent::Destroyed = event.event() {
		// 	let app: State<App> = event.window().state();
		// 	app.save();
		// })
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
