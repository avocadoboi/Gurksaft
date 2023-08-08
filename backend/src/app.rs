
use crate::{
	learning_data, 
	learning_data::{
		LearningData, 
		LearningWord, 
		SentenceId
	}
};
use crate::{options, options::Options};
use crate::sentence_audio::{
	AudioId,
	SentenceAudio
};
use crate::{source_data, source_data::SourceData};

use std::sync::{Mutex, MutexGuard};

use tauri::Manager;

use serde::{Serialize, Deserialize};

//----------------------------------------------------------------

struct AppState {
	options: Mutex<Options>,
	learning_data: Mutex<LearningData>,
	sentence_audio: SentenceAudio,
}

impl AppState {
	fn new(app: &tauri::AppHandle, source_data: &SourceData) -> Self {
		let learning_data = LearningData::load_from_source_data(&source_data);
		learning_data.save_sentences_to_file(source_data.language_index);

		Self {
			options: Mutex::new(options::Options::new(source_data.language_index)),
			learning_data: Mutex::new(learning_data),
			sentence_audio: SentenceAudio::new(&app)
		}
	}
	fn load(app: &tauri::AppHandle, options: Options) -> Self {
		let learning_data = Mutex::new(LearningData::load_from_file(options.language_index));
		Self {
			options: Mutex::new(options),
			learning_data,
			sentence_audio: SentenceAudio::new(&app)
		}
	}
	fn save(&self) {
		let options = self.options();
		options.save();
		self.learning_data().save_words_to_file(options.language_index)
	}

	fn options(&self) -> MutexGuard<'_, Options> {
		self.options.lock().unwrap()
	}
	fn learning_data(&self) -> MutexGuard<'_, LearningData> {
		self.learning_data.lock().unwrap()
	}
	// fn audio_id_map(&self, sentence_id: SentenceId) -> &[AudioId] {
	// 	self.audio_id_map
	// }
}

//----------------------------------------------------------------

#[tauri::command]
fn next_task(state: tauri::State<AppState>) -> learning_data::LearningTask {
	state.learning_data().next_task()
}

#[tauri::command]
fn finish_task(state: tauri::State<AppState>, task: learning_data::FinishedTask) {
	state.learning_data().finish_task(task, state.options().weight_factors)
}

#[tauri::command]
fn get_audio_ids(state: tauri::State<AppState>, sentence_id: SentenceId) -> Vec<AudioId> {
	state.sentence_audio.sentence_audio_ids(sentence_id)
}

#[tauri::command]
async fn download_sentence_audio(app: tauri::AppHandle, audio_id: u32) -> Vec<u8> {
	app.state::<AppState>().sentence_audio.download_audio(audio_id).await
}

#[tauri::command]
fn get_weight_factors(state: tauri::State<AppState>) -> options::WeightFactors {
	state.options().weight_factors
}

#[tauri::command]
fn set_success_weight_factor(state: tauri::State<AppState>, factor: f64) {
	state.options().weight_factors.succeeded = factor;
}
#[tauri::command]
fn set_failure_weight_factor(state: tauri::State<AppState>, factor: f64) {
	state.options().weight_factors.failed = factor;
}

#[tauri::command]
fn get_language_list() -> Vec<&'static str> {
	source_data::LANGUAGES.iter().map(|language| language.name).collect()
}

#[tauri::command]
fn get_saved_language_list(state: tauri::State<AppState>) -> Vec<&str> {
	state.options().saved_languages.iter().map(|&i| source_data::LANGUAGES[i].name).collect()
}

#[tauri::command]
fn get_current_language(state: tauri::State<AppState>) -> &str {
	source_data::LANGUAGES[state.options().language_index].name
}

#[tauri::command]
async fn set_current_language(app: tauri::AppHandle, language_name: String) {
	let state = app.state::<AppState>();
	
	let mut options = state.options();
	
	if let Some(&language_index) = options.saved_languages.iter().find(|&&i| source_data::LANGUAGES[i].name == language_name) {
		let mut learning_data = state.learning_data();
		learning_data.save_words_to_file(options.language_index);
		options.language_index = language_index;

		*learning_data = LearningData::load_from_file(language_index);
	}
}

#[tauri::command]
async fn download_language_data(app: tauri::AppHandle, window: tauri::Window, info: source_data::SourceDataInfo) {
	let source_data = SourceData::download(info, |status| {
		window.emit("download_status", &status).unwrap();
	}).await;

	window.emit("download_status", &source_data::SourceDataDownloadStatus::Loading).unwrap();

	add_new_language_data(&app, source_data);

	window.emit("download_status", &source_data::SourceDataDownloadStatus::Finished).unwrap();
}

fn add_new_language_data(app: &tauri::AppHandle, source_data: SourceData) {
	if let Some(state) = app.try_state::<AppState>() {
		let mut options = state.options();
		let mut learning_data = state.learning_data();

		learning_data.save_words_to_file(options.language_index);
		options.language_index = source_data.language_index;

		if let Err(i) = options.saved_languages.binary_search(&source_data.language_index) {
			options.saved_languages.insert(i, source_data.language_index);
		}
		
		*learning_data = LearningData::load_from_source_data(&source_data);
		// Save sentences immediately.
		// Sentences are saved only when necessary while words and their weights are saved every time the app closes.
		learning_data.save_sentences_to_file(options.language_index);
	}
	else {
		app.manage(AppState::new(app, &source_data));
	}
}

#[derive(Serialize, Deserialize)]
struct Weights {
	words: Vec<LearningWord>,
	max_weight: f64
}

#[tauri::command]
fn get_weights(state: tauri::State<AppState>) -> Weights {
	let learning_data = state.learning_data();
	let words = &learning_data.words().0;

	Weights {
		words: words.clone(),
		max_weight: words.iter()
			.max_by(|&a, &b| a.weight.total_cmp(&b.weight))
			.map_or(1., |word| word.weight)
	}
}

pub fn run() {
	tauri::Builder::default()
		.setup(|app| { start_app(app); Ok(()) })
		.invoke_handler(tauri::generate_handler![
			download_language_data,
			download_sentence_audio,
			finish_task,
			get_audio_ids,
			get_current_language,
			get_language_list,
			get_saved_language_list,
			get_weights,
			get_weight_factors,
			next_task, 
			set_current_language,
			set_failure_weight_factor,
			set_success_weight_factor,
		])
		.on_window_event(handle_window_event)
		.run(tauri::generate_context!())
		.unwrap();
}

fn start_app(app: &tauri::App) {
	if let Some(options) = Options::load() {
		app.manage(AppState::load(&app.app_handle(), options));
		tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("learn".into()))
			.center()
			.inner_size(700., 600.)
			.title("Gurksaft").build().unwrap();
	}
	else {
		tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("add-language".into()))
			.center()
			.inner_size(700., 450.)
			.title("Gurksaft")
			.build().unwrap();
	}
}

fn handle_window_event(event: tauri::GlobalWindowEvent) {
	if let tauri::WindowEvent::Destroyed = event.event() {
		if let Some(app) = event.window().try_state::<AppState>() {
			app.save();
		}
	}
}
