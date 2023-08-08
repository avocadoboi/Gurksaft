
use crate::learning_data::SentenceId;

use std::collections::HashMap;

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

pub struct SentenceAudio {
	audio_id_map: AudioIdMap,
	client: reqwest::Client,
}

impl SentenceAudio {
	pub fn new(app: &tauri::AppHandle) -> Self {
		SentenceAudio { 
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
