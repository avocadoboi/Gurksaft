use serde::{Deserialize, Serialize};

use tauri::Manager;

use tokio::sync::Mutex;

use crate::{
	learning_data::{
		FinishedTask,
		LearningData, 
		LearningTask,
		LearningWord, 
		SentenceId
	},
	options::{Options, WeightFactors, WordMemoryParameters},
	sentence_audio::AudioLoader,
	source_data::{
		SourceData,
		SourceDataDownloadStatus,
		SourceDataInfo,
		LANGUAGES,
	}
};

//----------------------------------------------------------------

struct AppState {
	options: Mutex<Options>,
	learning_data: Mutex<LearningData>,
	audio_loader: Mutex<AudioLoader>,
}

impl AppState {
	fn new(app: tauri::AppHandle, source_data: &SourceData) -> Self {
		let options = Options::new(source_data.language_index);

		let learning_data = LearningData::load_from_source_data(&source_data, &options);
		learning_data.save_sentences_to_file(source_data.language_index);

		Self {
			options: Mutex::new(options),
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
fn next_task(state: tauri::State<AppState>) -> LearningTask {
	state.learning_data.blocking_lock().next_task()
}

#[tauri::command]
fn finish_task(state: tauri::State<AppState>, task: FinishedTask) {
	state.learning_data.blocking_lock().finish_task(task, &state.options.blocking_lock())
}

//----------------------------------------------------------------

#[tauri::command]
async fn load_sentence_audio(app: tauri::AppHandle, window: tauri::Window, sentence_id: SentenceId, sentence: String) {
	app.state::<AppState>().audio_loader.lock().await
		.load_audio_for_sentence(&window, sentence_id, sentence).await;
}

//----------------------------------------------------------------

#[derive(Serialize, Deserialize)]
struct FrontendOptions {
	current_language: &'static str,
	saved_languages: Vec<&'static str>,
	weight_factors: WeightFactors,
	word_memory_parameters: WordMemoryParameters,
}

#[tauri::command]
fn get_options(state: tauri::State<AppState>) -> FrontendOptions {
	let options = state.options.blocking_lock();

	FrontendOptions { 
		current_language: LANGUAGES[options.language_index].name, 
		saved_languages: options.saved_languages.iter().map(|&i| LANGUAGES[i].name).collect(), 
		weight_factors: options.weight_factors, 
		word_memory_parameters: options.word_memory_parameters,
	}
}

#[tauri::command]
fn set_weight_factors(state: tauri::State<AppState>, factors: WeightFactors) {
	state.options.blocking_lock().weight_factors = factors;
}

#[tauri::command]
fn set_word_memory_parameters(state: tauri::State<AppState>, parameters: WordMemoryParameters) {
	state.options.blocking_lock().word_memory_parameters = parameters;
}

//----------------------------------------------------------------

#[tauri::command]
fn get_language_list() -> Vec<&'static str> {
	LANGUAGES.iter().map(|language| language.name).collect()
}

#[tauri::command]
async fn set_current_language(app: tauri::AppHandle, language_name: String) {
	let state = app.state::<AppState>();

	let mut options = state.options.lock().await;

	if language_name == LANGUAGES[options.language_index].name {
		return;
	}
	
	if let Some(&language_index) = options.saved_languages.iter().find(|&&i| LANGUAGES[i].name == language_name) {
		let mut learning_data = state.learning_data.lock().await;
		learning_data.save_words_to_file(options.language_index);
		options.language_index = language_index;
		*learning_data = LearningData::load_from_file(language_index);

		state.audio_loader.lock().await.set_language(language_index);
	}
}

//----------------------------------------------------------------

#[tauri::command]
async fn download_language_data(app: tauri::AppHandle, window: tauri::Window, info: SourceDataInfo) {
	let source_data = SourceData::download(info, |status| {
		window.emit("download_status", &status).unwrap();
	}).await;

	window.emit("download_status", &SourceDataDownloadStatus::Loading).unwrap();

	add_new_language_data(app, source_data).await;

	window.emit("download_status", &SourceDataDownloadStatus::Finished).unwrap();
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
		
		*learning_data = LearningData::load_from_source_data(&source_data, &options);
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
struct WordData {
	words: Vec<LearningWord>,
	max_weight: f64
}

#[tauri::command]
fn get_word_data(state: tauri::State<AppState>) -> WordData {
	let learning_data = state.learning_data.blocking_lock();
	let words = &learning_data.words().words;

	WordData {
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
			get_language_list,
			get_options,
			get_word_data,
			load_sentence_audio,
			next_task, 
			set_current_language,
			set_weight_factors,
			set_word_memory_parameters,
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
