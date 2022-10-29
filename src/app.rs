
use crate::{learning_data, learning_data::LearningData};
use crate::{options, options::Options};
use crate::{source_data, source_data::SourceData};

use std::fmt;
use std::sync::Mutex;

use tauri::Manager;

//----------------------------------------------------------------

struct AppState {
	options: Mutex<Options>,
	learning_data: Mutex<LearningData>,
}

impl AppState {
	fn save(&self) {
		let options = self.options.lock().unwrap();
		options.save();
		self.learning_data.lock().unwrap().save_to_file(&source_data::LANGUAGES[options.language_index])
	}
}

enum WindowLabel {
	AddLanguage,
	MainWindow,
}
impl fmt::Display for WindowLabel {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			WindowLabel::AddLanguage => write!(f, "add_language"),
			WindowLabel::MainWindow => write!(f, "main_window"),
		}
	}
}

#[tauri::command]
fn next_task(app: tauri::State<AppState>) -> learning_data::LearningTask {
	app.learning_data.lock().unwrap().next_task()
}

#[tauri::command]
fn finish_task(app: tauri::State<AppState>, task: learning_data::FinishedTask) {
	app.learning_data.lock().unwrap().finish_task(task, app.options.lock().unwrap().weight_factors)
}

#[tauri::command]
fn get_weight_factors(app: tauri::State<AppState>) -> options::WeightFactors {
	app.options.lock().unwrap().weight_factors
}

#[tauri::command]
fn set_success_weight_factor(app: tauri::State<AppState>, factor: f64) {
	app.options.lock().unwrap().weight_factors.succeeded = factor;
}
#[tauri::command]
fn set_failure_weight_factor(app: tauri::State<AppState>, factor: f64) {
	app.options.lock().unwrap().weight_factors.failed = factor;
}

#[tauri::command]
fn get_language_list() -> &'static [source_data::Language] {
	&source_data::LANGUAGES
}

fn create_main_window(app: tauri::AppHandle) {
	tauri::WindowBuilder::new(&app, WindowLabel::MainWindow.to_string(), tauri::WindowUrl::App("main_window/index.html".into()))
		.center()
		.inner_size(700., 600.)
		.title("Language learning tool").build().unwrap();
}

#[tauri::command]
async fn download_language_data(app: tauri::AppHandle, window: tauri::Window, info: source_data::SourceDataInfo) {
	let source_data = SourceData::download(info, |status| {
		window.emit("download_status", &status).unwrap();
	}).await;

	window.emit("download_status", &source_data::SourceDataDownloadStatus::Loading).unwrap();

	app.manage(AppState {
		options: Mutex::new(options::Options::new(source_data.language_index)),
		learning_data: Mutex::new(LearningData::load_from_source_data(source_data))
	});

	create_main_window(app);

	window.close().unwrap();
}

fn start_app(app: &tauri::App) {
	if let Some(options) = Options::load() {
		let learning_data = Mutex::new(LearningData::load_from_file(&source_data::LANGUAGES[options.language_index]));
		app.manage(AppState {
			options: Mutex::new(options),
			learning_data
		});
		create_main_window(app.handle());
	}
	else {
		tauri::WindowBuilder::new(app, WindowLabel::AddLanguage.to_string(), tauri::WindowUrl::App("add_language/index.html".into()))
			.center()
			.inner_size(700., 450.)
			.title("Add language")
			.build().unwrap();
	}
}

fn handle_window_event(event: tauri::GlobalWindowEvent) {
	if let tauri::WindowEvent::Destroyed = event.event() {
		if event.window().label() == WindowLabel::MainWindow.to_string() {
			let app: tauri::State<AppState> = event.window().state();
			app.save();
		}
	}
}

pub fn run() {
	tauri::Builder::default()
		.setup(|app| { start_app(app); Ok(()) })
		.invoke_handler(tauri::generate_handler![
			get_language_list,
			download_language_data,
			next_task, 
			finish_task,
			get_weight_factors,
			set_success_weight_factor,
			set_failure_weight_factor
		])
		.on_window_event(handle_window_event)
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
