
use std::sync::Mutex;

use tauri::{Manager, State};

use crate::learning_data::{LearningData, LearningTask, FinishedTask};

struct App {
	learning_data: Mutex<LearningData>,
}

impl App {
	fn new() -> Self {
		App {
			learning_data: Mutex::new(LearningData::load()),
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

pub fn run() {
	tauri::Builder::default()
		.manage(App::new())
		.invoke_handler(tauri::generate_handler![
			next_task, 
			finish_task
		])
		.on_window_event(|event| if let tauri::WindowEvent::Destroyed = event.event() {
			let app: State<App> = event.window().state();
			app.learning_data.lock().unwrap().save();
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
