use crate::util;

use std::collections::{HashMap, BTreeMap};

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use serde::{Serialize, Deserialize};

const WORD_FREQUENCY_SOURCE_FILE_NAME: &str = "source_data/words_by_frequency.txt";
const SENTENCE_PAIR_SOURCE_FILE_NAME: &str = "source_data/sentence_pairs.tsv";

const WORD_SAVE_FILE_NAME: &str = "save_data/words.tsv";
const SENTENCE_SAVE_FILE_NAME: &str = "save_data/sentences";

#[derive(Debug, Serialize, Deserialize)]
pub struct LearningWord {
	pub word: String,
	pub weight: f64,
}

impl LearningWord {
	fn from_frequency(word: String, frequency: u64, max_frequency: u64) -> Self {
		LearningWord {
			word,
			weight: f64::powi(frequency as f64 / max_frequency as f64, 2),
		}
	}
}

pub struct LearningWords(pub Vec<LearningWord>);

impl LearningWords {
	fn load_from_save() -> Option<Self> {
		if let Ok(mut reader) = csv::ReaderBuilder::new().delimiter(b'\t').from_path(WORD_SAVE_FILE_NAME) {
			println!("Loaded word save file.");
			Some(LearningWords(reader.deserialize::<LearningWord>().map(|result| result.expect("the save file should be properly formatted")).collect()))
		}
		else {
			None
		}
	}
	
	fn load_from_frequency_list() -> Self {
		let mut reader = csv::ReaderBuilder::new().delimiter(b' ')
			.from_path(WORD_FREQUENCY_SOURCE_FILE_NAME).expect("the word frequency source file should exist");

		let mut word_frequency_pairs = reader.deserialize::<(String, u64)>().map(|result| result.unwrap_or_default());
		
		let (first_word, max_frequency) = word_frequency_pairs.next().expect("the parsed word frequency list should not be empty");

		let the_rest = word_frequency_pairs.map(|(word, frequency)| LearningWord::from_frequency(word, frequency, max_frequency));
		
		println!("Loaded word frequency file.");

		LearningWords(std::iter::once(LearningWord::from_frequency(first_word, max_frequency, max_frequency)).chain(the_rest).collect())
	}

	pub fn load() -> Self {
		if let Some(result) = Self::load_from_save() {
			result
		}
		else {
			let list = Self::load_from_frequency_list();
			list.save();
			list
		}
	}

	pub fn save(&self) {
		util::create_parent_directory_if_nonexistent(WORD_SAVE_FILE_NAME);
		
		let mut writer = csv::WriterBuilder::new().delimiter(b'\t')
			.from_path(WORD_SAVE_FILE_NAME).unwrap();
		for row in &self.0 {
			writer.serialize(row).unwrap();
		}
	}
}

type SentenceId = u32;

pub struct Translation {
	pub id: SentenceId,
	pub text: String,
}

impl PartialEq for Translation {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for Translation {}

#[derive(Debug, Deserialize, Serialize)]
pub struct LearningSentence {
	pub original: String,
	pub translations: BTreeMap<SentenceId, String>,
	pub weight: f64,
}

impl LearningSentence {

}

pub struct LearningSentences(pub HashMap<SentenceId, LearningSentence>);

impl LearningSentences {
	fn load_from_save() -> Option<Self> {
		if let Ok(file) = std::fs::File::open(SENTENCE_SAVE_FILE_NAME) {
			if let Ok(map) = bincode::deserialize_from(file) {
				println!("Loaded sentence save file.");
				return Some(LearningSentences(map));
			}
		}
		None
	}
	
	fn load_from_sentence_pair_source_file() -> Self {
		#[derive(Debug, Deserialize)]
		struct SentencePair {
			id_0: SentenceId,
			original: String,
			id_1: SentenceId,
			translation: String,
		}
		
		let mut reader = csv::ReaderBuilder::new().delimiter(b'\t')
			.from_path(SENTENCE_PAIR_SOURCE_FILE_NAME).expect("the sentence pair source file should exist");

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
					weight: 1f64
				});
			}
		}

		println!("Loaded sentence pair file.");

		result
	}
	
	pub fn load() -> Self {
		if let Some(result) = Self::load_from_save() {
			result
		}
		else {
			let list = Self::load_from_sentence_pair_source_file();
			list.save();
			list
		}
	}

	pub fn save(&self) {
		util::create_parent_directory_if_nonexistent(SENTENCE_SAVE_FILE_NAME);
		let file = std::fs::File::create(SENTENCE_SAVE_FILE_NAME).unwrap();
		bincode::serialize_into(file, &self.0).unwrap();
	}
}

pub struct LearningData {
	pub words: LearningWords,
	pub sentences: LearningSentences,
	word_weighted_index: WeightedIndex<f64>,
	random: ThreadRng,
}

pub struct LearningTask {
	pub word_index: usize,
	pub sentence_id: SentenceId,
	pub word_position: usize,
}

impl LearningData {
	pub fn next_task(&mut self) -> LearningTask {
		loop {
			let word_index = self.word_weighted_index.sample(&mut self.random);

			let matching_sentences: Vec<_> = self.sentences.0.iter()
				.filter(|(_id, sentence)| util::contains_word(&sentence.original, &self.words.0[word_index].word))
				.collect();
			if matching_sentences.is_empty() {
				// The word does not exist in any of the sentences, we can remove it.
				self.words.0.remove(word_index);
				self.words.save();
				continue;
			}

			let word = &self.words.0[word_index];

			let sentence_weighted_index = WeightedIndex::new(
				matching_sentences.iter().map(|(_id, sentence)| sentence.weight * word.weight)
			).unwrap();
			let (sentence_id, sentence) = matching_sentences[sentence_weighted_index.sample(&mut self.random)];

			return LearningTask {
				word_index,
				sentence_id: *sentence_id,
				word_position: util::find_word_position(&sentence.original, &word.word).expect("the word should exist in the chosen sentence"),
			}
		}
	}
	
	pub fn load() -> Self {
		let words = LearningWords::load();
		let word_weighted_index = WeightedIndex::new(words.0.iter().map(|word| word.weight)).unwrap();
		LearningData { 
			words, 
			sentences: LearningSentences::load(), 
			word_weighted_index,
			random: thread_rng()
		}
	}
}
