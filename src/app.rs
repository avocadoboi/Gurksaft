
use crate::{learning_data, learning_data::LearningData};
use crate::{options, options::Options};
use crate::{source_data, source_data::SourceData};

use std::sync::Mutex;

use tauri::{State, Manager};

//----------------------------------------------------------------

struct App {
	options: Mutex<Options>,
	learning_data: Mutex<LearningData>,
}

impl App {
	fn save(&self) {
		self.options.lock().unwrap().save();
	}
}

#[tauri::command]
fn next_task(app: State<App>) -> learning_data::LearningTask {
	app.learning_data.lock().unwrap().next_task()
}

#[tauri::command]
fn finish_task(app: State<App>, task: learning_data::FinishedTask) {
	app.learning_data.lock().unwrap().finish_task(task, app.options.lock().unwrap().weight_factors)
}

#[tauri::command]
fn get_weight_factors(app: State<App>) -> options::WeightFactors {
	app.options.lock().unwrap().weight_factors
}

#[tauri::command]
fn set_success_weight_factor(app: State<App>, factor: f64) {
	app.options.lock().unwrap().weight_factors.succeeded = factor;
}
#[tauri::command]
fn set_failure_weight_factor(app: State<App>, factor: f64) {
	app.options.lock().unwrap().weight_factors.failed = factor;
}

#[tauri::command]
fn get_language_list() -> &'static [source_data::Language] {
	&source_data::LANGUAGES
}

#[tauri::command]
async fn download_language_data(app: tauri::AppHandle, window: tauri::Window, info: source_data::SourceDataInfo) {
	let source_data = SourceData::download(info, |status| {
		window.emit("download_status", &status).unwrap();
	}).await;

	window.emit("download_status", &source_data::SourceDataDownloadStatus::Loading);

	app.manage(App {
		options: Mutex::new(options::Options::new(source_data::LANGUAGES.iter().position(|language| language.name == source_data.info.target_language).unwrap())),
		learning_data: Mutex::new(LearningData::load_from_source_data(source_data))
	});

	tauri::WindowBuilder::new(&app, "main_window", tauri::WindowUrl::App("main_window/index.html".into()))
		.center()
		.inner_size(700., 600.)
		.title("Language learning tool").build().unwrap();

	window.close().unwrap();
}

pub fn run() {
	let options = Options::load();

	tauri::Builder::default()
		.setup(move |app| {
			if options.is_none() {
				tauri::WindowBuilder::new(app, "add_language", tauri::WindowUrl::App("add_language/index.html".into()))
					.center()
					.inner_size(700., 450.)
					.title("Add language")
					.build().unwrap();
			}
			Ok(())
		})
		// .manage()
		.invoke_handler(tauri::generate_handler![
			get_language_list,
			download_language_data,
			next_task, 
			finish_task,
			get_weight_factors,
			set_success_weight_factor,
			set_failure_weight_factor
		])
		// .on_window_event(|event| if let tauri::WindowEvent::Destroyed = event.event() {
		// 	let app: State<App> = event.window().state();
		// 	app.save();
		// })
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
