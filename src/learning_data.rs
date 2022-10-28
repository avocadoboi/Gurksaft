use crate::options;
use crate::source_data;
use crate::util;

use std::{collections::{HashMap, BTreeMap}, io::Write};

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use serde::{Serialize, Deserialize};

//----------------------------------------------------------------

const SAVE_DIRECTORY: &str = "save_data/";

//----------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct LearningWord {
	word: String,
	weight: f64,
}

impl LearningWord {
	fn from_frequency(word: String, frequency: u64, max_frequency: u64) -> Self {
		LearningWord {
			word,
			weight: f64::powi(frequency as f64 / max_frequency as f64, 2),
		}
	}
}

//----------------------------------------------------------------

#[derive(Deserialize, Serialize)]
struct LearningWords(Vec<LearningWord>);

impl LearningWords {
	// fn load_from_file() -> Option<Self> {
	// 	eprint!("Loading word save file... ");
	// 	std::io::stderr().flush().unwrap();
	// 	if let Ok(mut reader) = csv::ReaderBuilder::new().delimiter(b'\t').from_path(WORD_SAVE_FILE_NAME) {
	// 		let words = reader.deserialize::<LearningWord>().map(|result| result.expect("the save file should be properly formatted")).collect();
	// 		eprintln!("Done.");
	// 		Some(LearningWords(words))
	// 	}
	// 	else {
	// 		None
	// 	}
	// }
	
	fn load_from_source_data(data: &[u8]) -> Self {
		eprint!("Loading word source data... ");
		std::io::stderr().flush().unwrap();
		
		let mut reader = csv::ReaderBuilder::new().delimiter(b' ').has_headers(false).from_reader(data);

		let mut word_frequency_pairs = reader.deserialize::<(String, u64)>().map(|result| result.unwrap_or_default());
		
		let (first_word, max_frequency) = word_frequency_pairs.next().expect("the parsed word frequency list should not be empty");

		let the_rest = word_frequency_pairs.map(|(word, frequency)| LearningWord::from_frequency(word, frequency, max_frequency));
		
		let words = std::iter::once(LearningWord::from_frequency(first_word, max_frequency, max_frequency)).chain(the_rest).collect();
		
		eprintln!("Done.");

		LearningWords(words)
	}

	// fn load() -> Self {
	// 	if let Some(result) = Self::load_from_save() {
	// 		result
	// 	}
	// 	else {
	// 		let list = Self::load_from_frequency_list();
	// 		list.save();
	// 		list
	// 	}
	// }

	// fn save(&self) {
	// 	eprint!("Saving word file... ");
	// 	std::io::stderr().flush().unwrap();
		
	// 	util::create_parent_directory_if_nonexistent(WORD_SAVE_FILE_NAME);
		
	// 	let mut writer = csv::WriterBuilder::new().delimiter(b'\t')
	// 		.from_path(WORD_SAVE_FILE_NAME).unwrap();
	// 	for row in &self.0 {
	// 		writer.serialize(row).unwrap();
	// 	}

	// 	eprintln!("Done.");
	// }

	fn create_weighted_index(&self) -> WeightedIndex<f64> {
		WeightedIndex::new(self.0.iter().map(|word| word.weight)).unwrap()
	}
}

//----------------------------------------------------------------

pub type SentenceId = u32;

#[derive(Debug, Deserialize, Serialize)]
struct LearningSentence {
	original: String,
	translations: BTreeMap<SentenceId, String>,
}

//----------------------------------------------------------------

#[derive(Deserialize, Serialize)]
struct LearningSentences(HashMap<SentenceId, LearningSentence>);

impl LearningSentences {
	// fn load_from_save() -> Option<Self> {
	// 	eprint!("Loading sentence save file... ");
	// 	std::io::stderr().flush().unwrap();

	// 	if let Ok(file) = std::fs::File::open(SENTENCE_SAVE_FILE_NAME) {
	// 		if let Ok(map) = bincode::deserialize_from(file) {
	// 			eprintln!("Done.");
	// 			return Some(LearningSentences(map));
	// 		}
	// 	}
	// 	None
	// }
	
	fn load_from_source_data(data: &[u8]) -> Self {
		eprint!("Loading sentence source data... ");
		std::io::stderr().flush().unwrap();

		#[derive(Debug, Deserialize)]
		struct SentencePair {
			id_0: SentenceId,
			original: String,
			id_1: SentenceId,
			translation: String,
		}
		
		let mut reader = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(data);

		let pairs = reader.deserialize::<SentencePair>()
			.map(|result| result.expect("the sentence pair source file should be properly formatted"));

		let mut result = LearningSentences(HashMap::new());
		
		for pair in pairs {
			if let Some(sentence) = result.0.get_mut(&pair.id_0) {
				sentence.translations.insert(pair.id_1, pair.translation);
			}
			else {
				result.0.insert(pair.id_0, LearningSentence {
					original: pair.original,
					translations: BTreeMap::from([(pair.id_1, pair.translation)]),
				});
			}
		}

		eprintln!("Done.");

		result
	}
	
	// fn load() -> Self {
	// 	if let Some(result) = Self::load_from_save() {
	// 		result
	// 	}
	// 	else {
	// 		let list = Self::load_from_sentence_pair_source_file();
	// 		list.save();
	// 		list
	// 	}
	// }

	// fn save(&self) {
	// 	eprint!("Saving sentence file... ");
	// 	std::io::stderr().flush().unwrap();
		
	// 	util::create_parent_directory_if_nonexistent(SENTENCE_SAVE_FILE_NAME);
	// 	let file = std::fs::File::create(SENTENCE_SAVE_FILE_NAME).unwrap();
	// 	bincode::serialize_into(file, &self.0).unwrap();

	// 	eprintln!("Done.");
	// }
}

//----------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct LearningTask {
	pub word_id: usize,
	pub sentence_id: SentenceId,
	pub word: String,
	pub word_pos: usize,
	pub sentence: String,
	pub translations: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum TaskResult {
	Succeeded, 
	Failed
}

#[derive(Serialize, Deserialize)]
pub struct FinishedTask {
	pub word_id: usize,
	pub sentence_id: SentenceId,
	pub result: TaskResult,
}

// #[derive(Deserialize, Serialize)]
pub struct LearningData {
	words: LearningWords,
	sentences: LearningSentences,
	word_weighted_index: WeightedIndex<f64>,
}

impl LearningData {	
	pub fn next_task(&mut self) -> LearningTask {
		loop {
			let word_id = self.word_weighted_index.sample(&mut thread_rng());

			let matching_sentences: Vec<_> = self.sentences.0.iter()
				.filter(|(_id, sentence)| util::contains_word(&sentence.original, &self.words.0[word_id].word))
				.collect();
			if matching_sentences.is_empty() {
				// The word does not exist in any of the sentences, we can remove it.
				self.words.0.remove(word_id);
				self.word_weighted_index = self.words.create_weighted_index();
				continue;
			}

			let word = &self.words.0[word_id];

			let (&sentence_id, sentence) = matching_sentences.choose(&mut thread_rng()).unwrap();

			return LearningTask {
				word_id,
				sentence_id,
				word: word.word.clone(),
				word_pos: util::find_word_position(&sentence.original, &word.word).expect("the word should exist in the chosen sentence"),
				sentence: sentence.original.clone(),
				translations: sentence.translations.iter().map(|(_id, sentence)| sentence.clone()).collect(),
			}
		}
	}

	pub fn finish_task(&mut self, task: FinishedTask, weight_factors: options::WeightFactors) {
		let word = &mut self.words.0[task.word_id];
		word.weight *= match task.result {
			TaskResult::Succeeded => weight_factors.succeeded,
			TaskResult::Failed => weight_factors.failed
		};
		self.word_weighted_index.update_weights(&[(task.word_id, &word.weight)]).expect("should be able to update word weight");
	}
	
	// pub fn load_from_file(language: &source_data::Language) -> Self {
	// 	let filename = format!("{}/{}", SAVE_DIRECTORY, language.name);

		
	// 	let words = LearningWords::load_from_file();
	// 	let word_weighted_index = words.create_weighted_index();
	// 	Self { 
	// 		words, 
	// 		sentences: LearningSentences::load(), 
	// 		word_weighted_index,
	// 	}
	// }
	pub fn load_from_source_data(source_data: source_data::SourceData) -> Self {
		let words = LearningWords::load_from_source_data(&source_data.word_list);
		let word_weighted_index = words.create_weighted_index();
		Self { 
			words, 
			sentences: LearningSentences::load_from_source_data(&source_data.sentence_list),
			word_weighted_index,
		}
	}

	// pub fn save(&mut self) {
	// 	self.words.save();
	// 	self.sentences.save();
	// }
}
