use std::{
	collections::HashMap,
	fs,
};

use chrono::prelude::*;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use serde::{Serialize, Deserialize};

use crate::{
	options::Options,
	source_data,
	util
};

//----------------------------------------------------------------

pub const SAVE_DIRECTORY: &str = "save_data";
const MAX_SENTENCE_LEN: usize = 100;
const MAX_WORD_COUNT: usize = 10_000;

//----------------------------------------------------------------

const MIN_HISTORY_SPACING_SECONDS: i64 = 60;

#[derive(Deserialize, Serialize)]
pub struct HistoryDataPoint<T> {
	time: DateTime<Utc>,
	value: T,
}

#[derive(Deserialize, Serialize)]
pub struct HistoryData<T>(Vec<HistoryDataPoint<T>>);

impl<T: PartialEq + Default + Copy> HistoryData<T> {
	fn new() -> Self {
		Self(vec![HistoryDataPoint {
			time: Utc::now(),
			value: T::default()
		}])
	}
	
	fn add_point(&mut self, value: T) {
		if let Some(last) = self.0.last_mut() {
			let now = Utc::now();
			if (now - last.time).num_seconds() < MIN_HISTORY_SPACING_SECONDS || last.value == value {
				last.time = now;
				last.value = value;
				return;
			}
			else {
				self.0.push(HistoryDataPoint { time: now, value });
			}
		}
		else {
			self.0.push(HistoryDataPoint { time: Utc::now(), value });
		}
	}

	fn get_latest(&self) -> T {
		self.0.last().map_or(T::default(), |last| last.value)
	}
}

//----------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LearningWord {
	pub word: String,
	/*
		A number that determines how likely the word is to be reviewed.
	*/
	pub weight: f64,
	/*
		A number between 0 and 1 representing how well the word has been learned.
	*/
	pub long_term_memory: f32,
	pub last_review: DateTime<Utc>,
}

impl LearningWord {
	fn from_frequency(word: String, frequency: u64, max_frequency: u64, options: &Options) -> Self {
		LearningWord {
			word,
			weight: f64::powi(frequency as f64 / max_frequency as f64, 2),
			long_term_memory: options.word_memory_parameters.initial_memory,
			last_review: Utc.timestamp_nanos(0),
		}
	}

	fn update(&mut self, result: WordReviewResult, options: &Options) {
		self.weight *= match result {
			WordReviewResult::Succeeded => options.weight_factors.succeeded,
			WordReviewResult::Failed => options.weight_factors.failed
		};
		
		/*
			Below we update our measure of the user's long-term memory of the word. The long-term memory is a weighted decaying average
			of previous word review results. Basically you have to prove that you learned the word for this to go up. After each review, the 
			long-term memory goes a certain percentage towards 1 or 0 depending on if you got the word correct or incorrect. The more time 
			that has passed since the last review of this word, the more the result is weighed. A result tells us more about the long-term 
			memory of the word if more time has passed since the user was tested on the word last time - it is a more accurate data point. 
			However, even if you know the word, there is a certain low probability that you get the word incorrect, and even if you don't 
			know the word there is a certain probability you get the word correct. This means that the long-term memory should never go 
			100% towards 0 or 1. change_rate_range gives the minimum and maximum decay/change rates. half_time is the time it takes (since 
			last review) for the change rate to go halfway between the minimum and the maximum.
		*/
		const SECONDS_PER_DAY: f32 = (60 * 60 * 24) as f32;

		let now = Utc::now();
		let days_since_last_review = (now - self.last_review).num_seconds() as f32 / SECONDS_PER_DAY;
		self.last_review = now;

		let change_rate_range = options.word_memory_parameters.change_rate_range;
		let half_time = options.word_memory_parameters.change_rate_half_time;
		let change_rate = change_rate_range.max - change_rate_range.length() * f32::exp2(-days_since_last_review / half_time);
		self.long_term_memory += ((result == WordReviewResult::Succeeded) as i32 as f32 - self.long_term_memory) * change_rate;

		println!("Long term memory for word {}: {}", self.word, self.long_term_memory);
	}

}

//----------------------------------------------------------------

#[derive(Deserialize, Serialize)]
pub struct LearningWords {
	pub words: Vec<LearningWord>,
	pub learned_word_count: HistoryData<u32>,
	pub easy_word_count: HistoryData<u32>,
}

impl LearningWords {
	fn load_from_source_data(data: &[u8], options: &Options) -> Self {
		let mut reader = csv::ReaderBuilder::new().delimiter(b' ').has_headers(false).from_reader(data);

		let mut word_frequency_pairs = reader.deserialize::<(String, u64)>().filter_map(|result| result.ok()).take(MAX_WORD_COUNT);
		
		let (first_word, max_frequency) = word_frequency_pairs.next().expect("the parsed word frequency list should not be empty");

		let the_rest = word_frequency_pairs.map(|(word, frequency)| LearningWord::from_frequency(word, frequency, max_frequency, options));
		
		let words = std::iter::once(LearningWord::from_frequency(first_word, max_frequency, max_frequency, options))
			.chain(the_rest).collect();
		
		LearningWords {
			words,
			learned_word_count: HistoryData::new(),
			easy_word_count: HistoryData::new()
		}
	}

	fn create_weighted_index(&self) -> WeightedIndex<f64> {
		WeightedIndex::new(self.words.iter().map(|word| word.weight)).unwrap()
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
	lowercase: String,
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
				let lowercase = pair.original.to_lowercase();
				result.0.insert(pair.id_0, LearningSentence {
					original: pair.original,
					lowercase,
					translations: vec![translation],
				});
			}
		}

		result
	}
}

//----------------------------------------------------------------

/*
	A word to be reviewed in a sentence.
*/
#[derive(Serialize, Deserialize)]
pub struct TaskWord {
	pub id: usize,
	pub word: String,
	pub position: usize,
}

#[derive(Serialize, Deserialize)]
pub struct LearningTask {
	pub sentence_id: SentenceId,
	pub sentence: String,
	pub translations: Vec<String>,
	pub review_words: Vec<TaskWord>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum WordReviewResult {
	Succeeded, 
	Failed,
}

#[derive(Serialize, Deserialize)]
pub struct FinishedWordReview {
	pub word_id: usize,
	pub result: WordReviewResult,
}

#[derive(Serialize, Deserialize)]
pub struct FinishedTask {
	pub word_reviews: Vec<FinishedWordReview>,
}

//----------------------------------------------------------------

pub struct LearningData {
	words: LearningWords,
	word_weighted_index: WeightedIndex<f64>,
	sentences: LearningSentences,
}

impl LearningData {
	pub fn words(&self) -> &LearningWords {
		&self.words
	}

	fn choose_sentence_with_word(&self, word_id: usize) -> Option<(&SentenceId, &LearningSentence)> {
		// Find sentences that contain the word.
		let matching_sentences: Vec<_> = self.sentences.0.iter()
			.filter(|(_id, sentence)| util::contains_word(&sentence.lowercase, &self.words.words[word_id].word))
			.collect();

		if matching_sentences.is_empty() {
			None
		}
		else {
			// Choose a sentence randomly among the ones that contained the word.
			Some(*matching_sentences.choose(&mut thread_rng()).unwrap())
		}
	}

	pub fn next_task(&mut self) -> LearningTask {
		loop {
			// First we stochastically select the main word to be reviewed based on the weight distribution.
			let word_id = self.word_weighted_index.sample(&mut thread_rng());

			// Choose a sentence randomly among the ones that contained the word.
			let Some((&sentence_id, sentence)) = self.choose_sentence_with_word(word_id)
			else {
				// The word does not exist in any of the sentences, we can remove it.
				self.words.words.remove(word_id);
				self.word_weighted_index = self.words.create_weighted_index();
				continue;
			};

			// Lowercase versions might not have the same number of characters as the original ones, so we need to use the original case
			// when calculating word position.
			let original_words: Vec<&str> = sentence.original.split_ascii_whitespace().collect();

			let mut review_words = Vec::new();

			let mut add_review_words = |id: usize, review_word: &LearningWord| {
				let matching_words = sentence.lowercase
					.split_ascii_whitespace()
					.enumerate()
					.filter(|(_i, word)| *word == review_word.word)
					.map(|(i, _word)| {
						TaskWord {
							id,
							word: original_words[i].to_string(),
							position: util::get_word_position(&sentence.original, original_words[i]),
						}
					});
				review_words.extend(matching_words);
			};
			
			const EASY_THRESHOLD: f32 = 0.75;
			
			// If the sentence contains any words that are easy enough, review them.
			for (i, learned_word) in self.words.words.iter().enumerate().filter(|(_i, word)| word.long_term_memory > EASY_THRESHOLD) {
				add_review_words(i, learned_word);
			}

			let word = &self.words.words[word_id];

			// We want to review the selected word no matter what. If it is an easy word, it has already been included in the previous step.
			if word.long_term_memory <= EASY_THRESHOLD {
				add_review_words(word_id, word);
			}

			// The front-end wants the review words to be in order of position in the sentence.
			review_words.sort_unstable_by_key(|word| word.position);
			
			return LearningTask {
				sentence_id,
				sentence: sentence.original.clone(),
				translations: sentence.translations.iter().map(|sentence| sentence.text.clone()).collect(),
				review_words,
			};
		}
	}

	pub fn finish_task(&mut self, task: FinishedTask, options: &Options) {
		let mut learned_word_count = self.words.learned_word_count.get_latest();
		let mut easy_word_count = self.words.easy_word_count.get_latest();
		
		for word_review in &task.word_reviews {
			let word = &mut self.words.words[word_review.word_id];
			let previous_memory = word.long_term_memory;
			
			word.update(word_review.result, options);
			
			let learned_threshold = options.word_memory_parameters.learned_threshold;
			learned_word_count += (word.long_term_memory >= learned_threshold) as u32 - (previous_memory >= learned_threshold) as u32;
			let easy_threshold = options.word_memory_parameters.easy_threshold;
			easy_word_count += (word.long_term_memory >= easy_threshold) as u32 - (previous_memory >= easy_threshold) as u32;
		}

		self.words.learned_word_count.add_point(learned_word_count);
		self.words.easy_word_count.add_point(easy_word_count);
		
		let mut updated_weights: Vec<_> = task.word_reviews.iter()
			.map(|review| (review.word_id, &self.words.words[review.word_id].weight))
			.collect();
	
		// The elements passed to updated_weights must be sorted by index (first element in the tuple), and I assume not contain duplicates.
		updated_weights.sort_by_key(|(i, _weight)| *i);
		updated_weights.dedup_by_key(|(i, _weight)| *i);

		self.word_weighted_index.update_weights(&updated_weights).expect("should be able to update word weights");
	}
	
	pub fn load_from_source_data(source_data: &source_data::SourceData, options: &Options) -> Self {
		let words = LearningWords::load_from_source_data(&source_data.word_list, options);
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
		fs::create_dir_all(SAVE_DIRECTORY).unwrap();
		fs::write(
			Self::sentences_file_name(language_index), 
			bincode::serialize(&self.sentences).unwrap()
		).unwrap();
	}
	pub fn save_words_to_file(&self, language_index: usize) {
		fs::create_dir_all(SAVE_DIRECTORY).unwrap();
		fs::write(
			Self::words_file_name(language_index), 
			bincode::serialize(&self.words).unwrap()
		).unwrap();
	}
}
