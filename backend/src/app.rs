// use std::sync::{, MutexGuard};

use serde::{Deserialize, Serialize};

use tauri::Manager;

use tokio::sync::Mutex;

use crate::{
	learning_data, 
	learning_data::{
		LearningData, 
		LearningWord, 
		SentenceId
	},
	options, 
	options::Options,
	sentence_audio::AudioLoader,
	source_data, 
	source_data::SourceData
};

//----------------------------------------------------------------

struct AppState {
	options: Mutex<Options>,
	learning_data: Mutex<LearningData>,
	audio_loader: Mutex<AudioLoader>,
}

impl AppState {
	fn new(app: tauri::AppHandle, source_data: &SourceData) -> Self {
		let learning_data = LearningData::load_from_source_data(&source_data);
		learning_data.save_sentences_to_file(source_data.language_index);

		Self {
			options: Mutex::new(options::Options::new(source_data.language_index)),
			learning_data: Mutex::new(learning_data),
			audio_loader: Mutex::new(AudioLoader::new(app, source_data.language_index)),
		}
	}
	fn load(app: tauri::AppHandle, options: Options) -> Self {
		let learning_data = Mutex::new(LearningData::load_from_file(options.language_index));
		let audio_loader = Mutex::new(AudioLoader::new(app, options.language_index));
		Self {
			options: Mutex::new(options),
			learning_data,
			audio_loader,
		}
	}
	fn save(&self) {
		let options = self.options.blocking_lock();
		options.save();
		self.learning_data.blocking_lock().save_words_to_file(options.language_index)
	}
}

//----------------------------------------------------------------

#[tauri::command]
fn next_task(state: tauri::State<AppState>) -> learning_data::LearningTask {
	state.learning_data.blocking_lock().next_task()
}

#[tauri::command]
fn finish_task(state: tauri::State<AppState>, task: learning_data::FinishedTask) {
	state.learning_data.blocking_lock().finish_task(task, state.options.blocking_lock().weight_factors)
}

//----------------------------------------------------------------

#[tauri::command]
async fn load_sentence_audio(app: tauri::AppHandle, window: tauri::Window, sentence_id: SentenceId, sentence: String) {
	// let _listen_guard = WindowListenGuard::new(&window, "cancel_sentence_audio", |_| {
	// 	println!("Setting should_cancel to true");
	// 	SHOULD_CANCEL_AUDIO_LOADING.store(true, Ordering::SeqCst);
	// });

	let state = app.state::<AppState>();
	let audio_loader = state.audio_loader.lock().await;
	audio_loader.load_audio_for_sentence(&window, sentence_id, sentence).await;
	// tokio::task::block_in_place(audio_loader.load_audio_for_sentence(&window, sentence_id, sentence));
}

//----------------------------------------------------------------

#[tauri::command]
fn get_weight_factors(state: tauri::State<AppState>) -> options::WeightFactors {
	state.options.blocking_lock().weight_factors
}

#[tauri::command]
fn set_success_weight_factor(state: tauri::State<AppState>, factor: f64) {
	state.options.blocking_lock().weight_factors.succeeded = factor;
}
#[tauri::command]
fn set_failure_weight_factor(state: tauri::State<AppState>, factor: f64) {
	state.options.blocking_lock().weight_factors.failed = factor;
}

//----------------------------------------------------------------

#[tauri::command]
fn get_language_list() -> Vec<&'static str> {
	source_data::LANGUAGES.iter().map(|language| language.name).collect()
}

#[tauri::command]
fn get_saved_language_list(state: tauri::State<AppState>) -> Vec<&str> {
	state.options.blocking_lock().saved_languages.iter().map(|&i| source_data::LANGUAGES[i].name).collect()
}

#[tauri::command]
fn get_current_language(state: tauri::State<AppState>) -> &str {
	source_data::LANGUAGES[state.options.blocking_lock().language_index].name
}

#[tauri::command]
async fn set_current_language(app: tauri::AppHandle, language_name: String) {
	let state = app.state::<AppState>();

	let mut options = state.options.lock().await;

	if language_name == source_data::LANGUAGES[options.language_index].name {
		return;
	}
	
	if let Some(&language_index) = options.saved_languages.iter().find(|&&i| source_data::LANGUAGES[i].name == language_name) {
		let mut learning_data = state.learning_data.lock().await;
		learning_data.save_words_to_file(options.language_index);
		options.language_index = language_index;
		*learning_data = LearningData::load_from_file(language_index);

		state.audio_loader.lock().await.set_language(language_index);
	}
}

//----------------------------------------------------------------

#[tauri::command]
async fn download_language_data(app: tauri::AppHandle, window: tauri::Window, info: source_data::SourceDataInfo) {
	let source_data = SourceData::download(info, |status| {
		window.emit("download_status", &status).unwrap();
	}).await;

	window.emit("download_status", &source_data::SourceDataDownloadStatus::Loading).unwrap();

	add_new_language_data(app, source_data).await;

	window.emit("download_status", &source_data::SourceDataDownloadStatus::Finished).unwrap();
}

async fn add_new_language_data(app: tauri::AppHandle, source_data: SourceData) {
	if let Some(state) = app.try_state::<AppState>() {
		let mut options = state.options.lock().await;
		let mut learning_data = state.learning_data.lock().await;

		learning_data.save_words_to_file(options.language_index);
		options.language_index = source_data.language_index;
		
		if let Err(i) = options.saved_languages.binary_search(&source_data.language_index) {
			options.saved_languages.insert(i, source_data.language_index);
		}
		
		*learning_data = LearningData::load_from_source_data(&source_data);
		// Save sentences immediately.
		// Sentences are saved only when necessary while words and their weights are saved every time the app closes.
		learning_data.save_sentences_to_file(options.language_index);

		state.audio_loader.lock().await.set_language(options.language_index);
	}
	else {
		app.manage(AppState::new(app.clone(), &source_data));
	}
}

//----------------------------------------------------------------

#[derive(Serialize, Deserialize)]
struct Weights {
	words: Vec<LearningWord>,
	max_weight: f64
}

#[tauri::command]
fn get_weights(state: tauri::State<AppState>) -> Weights {
	let learning_data = state.learning_data.blocking_lock();
	let words = &learning_data.words().0;

	Weights {
		words: words.clone(),
		max_weight: words.iter()
			.max_by(|&a, &b| a.weight.total_cmp(&b.weight))
			.map_or(1., |word| word.weight)
	}
}

//----------------------------------------------------------------

pub fn run() {
	tauri::Builder::default()
		.setup(|app| { start_app(app); Ok(()) })
		.invoke_handler(tauri::generate_handler![
			download_language_data,
			finish_task,
			get_current_language,
			get_language_list,
			get_saved_language_list,
			get_weights,
			get_weight_factors,
			load_sentence_audio,
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
		app.manage(AppState::load(app.app_handle(), options));
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
