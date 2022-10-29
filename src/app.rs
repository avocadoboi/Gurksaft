
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

//----------------------------------------------------------------

#[tauri::command]
fn next_task(state: tauri::State<AppState>) -> learning_data::LearningTask {
	state.learning_data.lock().unwrap().next_task()
}

#[tauri::command]
fn finish_task(state: tauri::State<AppState>, task: learning_data::FinishedTask) {
	state.learning_data.lock().unwrap().finish_task(task, state.options.lock().unwrap().weight_factors)
}

#[tauri::command]
fn get_weight_factors(state: tauri::State<AppState>) -> options::WeightFactors {
	state.options.lock().unwrap().weight_factors
}

#[tauri::command]
fn set_success_weight_factor(state: tauri::State<AppState>, factor: f64) {
	state.options.lock().unwrap().weight_factors.succeeded = factor;
}
#[tauri::command]
fn set_failure_weight_factor(state: tauri::State<AppState>, factor: f64) {
	state.options.lock().unwrap().weight_factors.failed = factor;
}

#[tauri::command]
fn get_language_list() -> Vec<&'static str> {
	source_data::LANGUAGES.iter().map(|language| language.name).collect()
}

#[tauri::command]
fn get_saved_language_list(state: tauri::State<AppState>) -> Vec<&str> {
	state.options.lock().unwrap().saved_languages.iter().map(|&i| source_data::LANGUAGES[i].name).collect()
}

#[tauri::command]
fn get_current_language(state: tauri::State<AppState>) -> &str {
	source_data::LANGUAGES[state.options.lock().unwrap().language_index].name
}

#[tauri::command]
fn set_current_language(state: tauri::State<AppState>, language_name: String) {
	let mut options = state.options.lock().unwrap();
	
	if let Some(&language_index) = options.saved_languages.iter().find(|&&i| source_data::LANGUAGES[i].name == language_name) {
		let mut learning_data = state.learning_data.lock().unwrap();
		learning_data.save_to_file(&source_data::LANGUAGES[options.language_index]);
		options.language_index = language_index;

		*learning_data = LearningData::load_from_file(&source_data::LANGUAGES[language_index]);

	}

}

fn create_main_window(app: &tauri::AppHandle) {
	tauri::WindowBuilder::new(app, WindowLabel::MainWindow.to_string(), tauri::WindowUrl::App("main_window/index.html".into()))
		.center()
		.inner_size(700., 600.)
		.title("Language learning tool").build().unwrap();
}
fn create_add_language_window(app: &tauri::AppHandle) {
	tauri::WindowBuilder::new(app, WindowLabel::AddLanguage.to_string(), tauri::WindowUrl::App("add_language/index.html".into()))
		.center()
		.inner_size(700., 450.)
		.title("Add language")
		.build().unwrap();
}

fn add_new_language_data(app: &tauri::AppHandle, source_data: SourceData) {
	if let Some(state) = app.try_state::<AppState>() {
		let mut options = state.options.lock().unwrap();
		let mut learning_data = state.learning_data.lock().unwrap();

		learning_data.save_to_file(&source_data::LANGUAGES[options.language_index]);
		options.language_index = source_data.language_index;

		if let Err(i) = options.saved_languages.binary_search(&source_data.language_index) {
			options.saved_languages.insert(i, source_data.language_index);
		}
		
		*learning_data = LearningData::load_from_source_data(source_data);
	}
	else {
		app.manage(AppState {
			options: Mutex::new(options::Options::new(source_data.language_index)),
			learning_data: Mutex::new(LearningData::load_from_source_data(source_data))
		});
	}
}

#[tauri::command]
async fn download_language_data(app: tauri::AppHandle, window: tauri::Window, info: source_data::SourceDataInfo) {
	let source_data = SourceData::download(info, |status| {
		window.emit("download_status", &status).unwrap();
	}).await;

	window.emit("download_status", &source_data::SourceDataDownloadStatus::Loading).unwrap();

	add_new_language_data(&app, source_data);

	create_main_window(&app);

	window.close().unwrap();
}

#[tauri::command]
fn add_language(app: tauri::AppHandle, window: tauri::Window) {
	create_add_language_window(&app);
	
	window.close().unwrap();
}

fn start_app(app: &tauri::App) {
	if let Some(options) = Options::load() {
		let learning_data = Mutex::new(LearningData::load_from_file(&source_data::LANGUAGES[options.language_index]));
		app.manage(AppState {
			options: Mutex::new(options),
			learning_data
		});
		create_main_window(&app.handle());
	}
	else {
		create_add_language_window(&app.handle());
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
			get_saved_language_list,
			get_current_language,
			set_current_language,
			add_language,
			get_weight_factors,
			set_success_weight_factor,
			set_failure_weight_factor
		])
		.on_window_event(handle_window_event)
		.run(tauri::generate_context!())
		.unwrap();
}
