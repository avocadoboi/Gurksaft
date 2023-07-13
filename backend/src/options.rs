use serde::{Deserialize, Serialize};

use crate::util;

//----------------------------------------------------------------

const OPTIONS_SAVE_FILE: &str = "save_data/options";

//----------------------------------------------------------------

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct WeightFactors {
	pub succeeded: f64,
	pub failed: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Options {
    pub language_index: usize,
    pub saved_languages: Vec<usize>,
    pub weight_factors: WeightFactors,
}

impl Options {
    pub fn save(&self) {
		util::create_parent_directory_if_nonexistent(OPTIONS_SAVE_FILE);
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
            weight_factors: WeightFactors {
                succeeded: 0.8,
                failed: 2.,
            }
        }
    }
}
