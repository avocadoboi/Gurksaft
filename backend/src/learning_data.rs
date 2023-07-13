use crate::options;
use crate::source_data;
use crate::util;

use std::collections::HashMap;
use std::fs;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use serde::{Serialize, Deserialize};

//----------------------------------------------------------------

const SAVE_DIRECTORY: &str = "save_data/";
const MAX_SENTENCE_LEN: usize = 100;

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
	fn load_from_source_data(data: &[u8]) -> Self {
		let mut reader = csv::ReaderBuilder::new().delimiter(b' ').has_headers(false).from_reader(data);

		let mut word_frequency_pairs = reader.deserialize::<(String, u64)>().filter_map(|result| result.ok());
		
		let (first_word, max_frequency) = word_frequency_pairs.next().expect("the parsed word frequency list should not be empty");

		let the_rest = word_frequency_pairs.map(|(word, frequency)| LearningWord::from_frequency(word, frequency, max_frequency));
		
		let words = std::iter::once(LearningWord::from_frequency(first_word, max_frequency, max_frequency)).chain(the_rest).collect();
		
		LearningWords(words)
	}

	fn create_weighted_index(&self) -> WeightedIndex<f64> {
		WeightedIndex::new(self.0.iter().map(|word| word.weight)).unwrap()
	}
}

//----------------------------------------------------------------

pub type SentenceId = u32;

#[derive(Deserialize, Serialize)]
struct Translation {
	id: SentenceId,
	text: String,
}

#[derive(Deserialize, Serialize)]
struct LearningSentence {
	original: String,
	translations: Vec<Translation>,
}

//----------------------------------------------------------------

#[derive(Deserialize, Serialize)]
struct LearningSentences(HashMap<SentenceId, LearningSentence>);

impl LearningSentences {
	fn load_from_source_data(data: &[u8]) -> Self {
		#[derive(Debug, Deserialize)]
		struct SentencePair {
			id_0: SentenceId,
			original: String,
			id_1: SentenceId,
			translation: String,
		}
		
		let mut reader = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(data);
		
		let pairs = reader.deserialize::<SentencePair>()
			.filter_map(|result| result.ok())
			.filter(|sentence| sentence.original.len() < MAX_SENTENCE_LEN);

		let mut result = LearningSentences(HashMap::with_capacity(100_000));
		
		for pair in pairs {
			let translation = Translation { id: pair.id_1, text: pair.translation };
			if let Some(sentence) = result.0.get_mut(&pair.id_0) {
				sentence.translations.push(translation);
			}
			else {
				result.0.insert(pair.id_0, LearningSentence {
					original: pair.original,
					translations: vec![translation],
				});
			}
		}

		result
	}
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

pub struct LearningData {
	words: LearningWords,
	word_weighted_index: WeightedIndex<f64>,
	sentences: LearningSentences,
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
				translations: sentence.translations.iter().map(|sentence| sentence.text.clone()).collect(),
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
	
	pub fn load_from_source_data(source_data: &source_data::SourceData) -> Self {
		let words = LearningWords::load_from_source_data(&source_data.word_list);
		let word_weighted_index = words.create_weighted_index();
		Self { 
			words, 
			word_weighted_index,
			sentences: LearningSentences::load_from_source_data(&source_data.sentence_list),
		}
	}

	fn words_file_name(language_index: usize) -> String {
		format!("{}/{}_words", SAVE_DIRECTORY, source_data::LANGUAGES[language_index].name)
	}
	fn sentences_file_name(language_index: usize) -> String {
		format!("{}/{}_sentences", SAVE_DIRECTORY, source_data::LANGUAGES[language_index].name)
	}
	
	pub fn load_from_file(language_index: usize) -> Self {
		let words: LearningWords = bincode::deserialize(&fs::read(Self::words_file_name(language_index)).unwrap()).unwrap();
		let word_weighted_index = words.create_weighted_index();
		Self { 
			words, 
			word_weighted_index,
			sentences: bincode::deserialize(&fs::read(Self::sentences_file_name(language_index)).unwrap()).unwrap(),
		}
	}

	pub fn save_sentences_to_file(&self, language_index: usize) {
		util::create_directory_if_nonexistent(SAVE_DIRECTORY);
		fs::write(
			Self::sentences_file_name(language_index), 
			bincode::serialize(&self.sentences).unwrap()
		).unwrap();
	}
	pub fn save_words_to_file(&self, language_index: usize) {
		util::create_directory_if_nonexistent(SAVE_DIRECTORY);
		fs::write(
			Self::words_file_name(language_index), 
			bincode::serialize(&self.words).unwrap()
		).unwrap();
	}
}
