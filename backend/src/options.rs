use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

//----------------------------------------------------------------

const OPTIONS_SAVE_FILE: &str = "save_data/options";

//----------------------------------------------------------------

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct WeightFactors {
	pub succeeded: f64,
	pub failed: f64,
}

impl Default for WeightFactors {
    fn default() -> Self {
        Self {
            succeeded: 0.7,
            failed: 2.,
        }
    }
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Range<T: Copy> {
    pub min: T,
    pub max: T,
}

impl<T: Copy + std::ops::Sub<Output = T>> Range<T> {
    pub fn length(&self) -> T {
        self.max - self.min
    }
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct WordMemoryParameters {
    pub easy_threshold: f32,
    pub learned_threshold: f32,
    pub change_rate_range: Range<f32>,
    // In days
    pub change_rate_half_time: f32,
    pub initial_memory: f32,
}

impl Default for WordMemoryParameters {
    fn default() -> Self {
        Self {
            easy_threshold: 0.75,
            learned_threshold: 0.9,
            change_rate_range: Range { min: 0.08, max: 0.5 },
            change_rate_half_time: 4.,
            initial_memory: 0.3
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Options {
    pub language_index: usize,
    pub saved_languages: Vec<usize>,
    pub weight_factors: WeightFactors,
    pub word_memory_parameters: WordMemoryParameters,
    pub hide_translations_by_default: bool,
    pub skip_correct_feedback: bool,
    pub play_audio_automatically: bool,
}

impl Options {
    pub fn save(&self) {
        if let Some(directory) = Path::new(OPTIONS_SAVE_FILE).parent() {
            fs::create_dir_all(directory).unwrap();
        }
		let file = std::fs::File::create(OPTIONS_SAVE_FILE).unwrap();
		bincode::serialize_into(file, &self).unwrap();
    }
    
    pub fn load() -> Option<Self> {
        bincode::deserialize_from(std::fs::File::open(OPTIONS_SAVE_FILE).ok()?).ok()
    }
    pub fn new(language_index: usize) -> Self {
        Self {
            language_index,
            saved_languages: vec![language_index],
            weight_factors: WeightFactors::default(),
            word_memory_parameters: WordMemoryParameters::default(),
            hide_translations_by_default: false,
            skip_correct_feedback: false,
            play_audio_automatically: true,
        }
    }
}
