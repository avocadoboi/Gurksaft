use std::{
	collections::HashMap,
	sync::{Arc, atomic, atomic::AtomicBool}
};
use once_cell::sync::Lazy;

use piper::{vits::VitsModel, synth::PiperSpeechSynthesizer};

use tauri::Manager;

use crate::{
	learning_data::{SentenceId, SAVE_DIRECTORY}, 
	source_data::LANGUAGES
};

//----------------------------------------------------------------

pub type AudioId = SentenceId;

struct AudioIdMap(HashMap<SentenceId, Vec<AudioId>>);

impl AudioIdMap {
	fn load(app: &tauri::AppHandle) -> Self {
		let path = app.path_resolver()
			.resolve_resource("audio_ids")
			.expect("audio IDs resource should exist");

		let file = std::fs::File::open(&path)
			.expect("audio IDs resource should exist");

		AudioIdMap(bincode::deserialize_from(&file).expect("audio ID data should be serialized properly by data_packing_utility"))
	}
}

//----------------------------------------------------------------

pub struct SentenceAudioRecordings {
	audio_id_map: AudioIdMap,
	client: reqwest::Client,
}

impl SentenceAudioRecordings {
	pub fn new(app: &tauri::AppHandle) -> Self {
		SentenceAudioRecordings { 
			audio_id_map: AudioIdMap::load(app), 
			client: reqwest::Client::new(),
		}
	}

	pub fn sentence_audio_ids(&self, sentence_id: SentenceId) -> Vec<AudioId> {
		if let Some(id_vector) = self.audio_id_map.0.get(&sentence_id) {
			id_vector.clone()
		}
		else {
			vec![]
		}
	}
	
	pub async fn download_audio(&self, audio_id: AudioId) -> Vec<u8> {
		let url = format!("https://tatoeba.org/audio/download/{}", audio_id);

		let response = self.client.get(url).send().await;
		
		if let Ok(response) = response && response.status().is_success() {
			response.bytes().await.unwrap().to_vec()
		}
		else {
			vec![]
		}
	}
}

//----------------------------------------------------------------

static ENVIRONMENT: Lazy<Arc<ort::Environment>> = Lazy::new(|| Arc::new(ort::Environment::default()));

pub struct NeuralSpeechModels {
	pub models: Vec<Arc<VitsModel>>,
	language_index: Option<usize>,
}

impl NeuralSpeechModels {
	pub fn new(language_index: usize) -> NeuralSpeechModels {
		NeuralSpeechModels { models: Self::load_models(language_index), language_index: None }
	}

	pub fn set_language(&mut self, language_index: usize) {
		if self.language_index == Some(language_index) {
			return;
		}
		self.language_index = Some(language_index);
		// The models are pretty memory intensive so we clear them first.
		self.models.clear();
		self.models = Self::load_models(language_index);
	}

	fn load_models(language_index: usize) -> Vec<Arc<VitsModel>> {
		let mut models = Vec::new();

		for model in &LANGUAGES[language_index].piper_voices {
			let onnx_path = format!("{}/voices/{}.onnx", SAVE_DIRECTORY, model.replace('/', "-"));
			models.push(Arc::new(VitsModel::new(format!("{}.json", onnx_path).into(), onnx_path.into(), &ENVIRONMENT).unwrap()));
		}

		models
	}
}

//----------------------------------------------------------------

struct AppListenGuard {
	app: tauri::AppHandle,
	handler_id: tauri::EventHandler
}

impl AppListenGuard {
	fn new<F>(app: tauri::AppHandle, event_name: &str, handler: F) -> Self 
		where F: Fn(tauri::Event) + Send + 'static
	{
		println!("Listen");
		let handler_id = app.listen_global(event_name, handler);
		Self { 
			app, 
			handler_id,
		}
	}
}

impl Drop for AppListenGuard {
	fn drop(&mut self) {
		println!("Unlisten");
		self.app.unlisten(self.handler_id);
	}
}

//----------------------------------------------------------------

pub struct AudioLoader {
	sentence_recordings: SentenceAudioRecordings,
	speech_models: NeuralSpeechModels,
	_listen_guard: AppListenGuard
}

static SHOULD_CANCEL_AUDIO_LOADING: AtomicBool = AtomicBool::new(false);

impl AudioLoader {
	pub fn new(app: tauri::AppHandle, language_index: usize) -> Self {
		Self {
			sentence_recordings: SentenceAudioRecordings::new(&app),
			speech_models: NeuralSpeechModels::new(language_index),
			_listen_guard: AppListenGuard::new(app, "cancel_sentence_audio", |_| {
				println!("Setting SHOULD_CANCEL_AUDIO_LOADING to true in listener");
				SHOULD_CANCEL_AUDIO_LOADING.store(true, atomic::Ordering::SeqCst);
			}),
		}
	}

	pub fn set_language(&mut self, language_index: usize) {
		self.speech_models.set_language(language_index);
	}
	
	pub async fn load_audio_for_sentence(&self, window: &tauri::Window, sentence_id: SentenceId, sentence: String) {
		println!("Started loading audio for '{}'.", &sentence);
		
		// Download recorded audio clips if available.

		for audio_id in self.sentence_recordings.sentence_audio_ids(sentence_id) {
			let file_data = self.sentence_recordings.download_audio(audio_id).await;
			if SHOULD_CANCEL_AUDIO_LOADING.load(atomic::Ordering::SeqCst) {
				SHOULD_CANCEL_AUDIO_LOADING.store(false, atomic::Ordering::SeqCst);
				println!("Stopped loading audio for '{}'.", &sentence);
				return;
			}
			window.emit("sentence_audio_data", file_data).unwrap();
		}

		// Generate neural speech with Piper.

		let load_sentence_audio_for_model = |model: Arc<VitsModel>| {
			let file_data = PiperSpeechSynthesizer::new(model).unwrap()
				.synthesize_to_wav_buffer(sentence.clone()).unwrap();

			if SHOULD_CANCEL_AUDIO_LOADING.load(atomic::Ordering::SeqCst) {
				SHOULD_CANCEL_AUDIO_LOADING.store(false, atomic::Ordering::SeqCst);
				return true;
			}

			window.emit("sentence_audio_data", file_data).unwrap();
			false
		};

		for model in &self.speech_models.models {
			if let Ok(speakers) = model.speakers() && !speakers.is_empty() {
				for (_, name) in speakers {
					if model.set_speaker(name).is_ok() {
						if load_sentence_audio_for_model(model.clone()) {
							println!("Stopped loading audio for '{}'.", &sentence);
							return;
						}
					}
				}
			}
			else if load_sentence_audio_for_model(model.clone()) {
				println!("Stopped loading audio for '{}'.", &sentence);
				return;
			}
		}
		println!("Stopped loading audio for '{}'.", &sentence);
	}
}

impl Drop for AudioLoader {
	fn drop(&mut self) {
		println!("Setting SHOULD_CANCEL_AUDIO_LOADING to true in AudioLoader::drop()");
		SHOULD_CANCEL_AUDIO_LOADING.store(true, atomic::Ordering::SeqCst);
	}
}
