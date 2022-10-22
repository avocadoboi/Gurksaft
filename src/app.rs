
use crate::learning_data::{LearningData, LearningTask, FinishedTask, WeightFactors, self};
use crate::options::Options;

use std::sync::Mutex;

use tauri::{Manager, State};

//----------------------------------------------------------------

struct App {
	options: Mutex<Options>,
	learning_data: Mutex<Option<LearningData>>,
}

impl App {
	fn new() -> Self {
		App {
			options: Mutex::new(Options::load()),
			learning_data: Mutex::new(None),
		}
	}
}

#[tauri::command]
fn next_task(app: State<App>) -> LearningTask {
	app.learning_data.lock().unwrap().next_task()
}

#[tauri::command]
fn finish_task(app: State<App>, task: FinishedTask) {
	app.learning_data.lock().unwrap().finish_task(task)
}

#[tauri::command]
fn get_weight_factors(app: State<App>) -> WeightFactors {
	app.learning_data.lock().unwrap().weight_factors
}

#[tauri::command]
fn set_success_weight_factor(app: State<App>, factor: f64) {
	app.learning_data.lock().unwrap().weight_factors.succeeded = factor;
}
#[tauri::command]
fn set_failure_weight_factor(app: State<App>, factor: f64) {
	app.learning_data.lock().unwrap().weight_factors.failed = factor;
}

pub fn run() {
	tauri::Builder::default()
		.setup(|app| {
			if Options::exists_save_file() {
				
			}
			else {

			}
			Ok(())
		})
		.manage(App::new())
		.invoke_handler(tauri::generate_handler![
			next_task, 
			finish_task,
			get_weight_factors,
			set_success_weight_factor,
			set_failure_weight_factor
		])
		.on_window_event(|event| if let tauri::WindowEvent::Destroyed = event.event() {
			let app: State<App> = event.window().state();
			app.learning_data.lock().unwrap().save();
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
