use std::{fs, io::Write};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::learning_data::SAVE_DIRECTORY;

//----------------------------------------------------------------

#[derive(Deserialize, Serialize)]
pub struct Language {
	pub name: &'static str,
	// Two-letter ISO 639-1 language code.
	pub id_2: &'static str,
	// Three-letter ISO 639-3 language code.
	pub id_3: &'static str,
	/*
		PIPER voice model paths for the language, relative to https://huggingface.co/rhasspy/piper-voices/tree/v1.0.0/{id_2}/ where id_2 is 
		the two-letter ISO 639-1 language code and is not included in the strings. The file name is not included either and is retrieved by 
		replacing "/" by "-" and adding the appropriate extension. These are approximately ordered in order of quality from highest to 
		lowest.
	*/
	pub piper_voices: Vec<&'static str>
}

pub static LANGUAGES: Lazy<Vec<Language>> = Lazy::new(|| vec![
	Language {
		name: "Afrikaans",
		id_2: "af",
		id_3: "afr",
		piper_voices: vec![],
	},
	Language {
		name: "Albanian",
		id_2: "sq",
		id_3: "sqi",
		piper_voices: vec![],
	},
	Language {
		name: "Arabic",
		id_2: "ar",
		id_3: "ara",
		piper_voices: vec![],
	},
	Language {
		name: "Armenian",
		id_2: "hy",
		id_3: "hye",
		piper_voices: vec![],
	},
	Language {
		name: "Basque",
		id_2: "eu",
		id_3: "eus",
		piper_voices: vec![],
	},
	Language {
		name: "Bengali",
		id_2: "bn",
		id_3: "ben",
		piper_voices: vec![],
	},
	Language {
		name: "Bosnian",
		id_2: "bs",
		id_3: "bos",
		piper_voices: vec![],
	},
	Language {
		name: "Breton",
		id_2: "br",
		id_3: "bre",
		piper_voices: vec![],
	},
	Language {
		name: "Bulgarian",
		id_2: "bg",
		id_3: "bul",
		piper_voices: vec![],
	},
	Language {
		name: "Catalan",
		id_2: "ca",
		id_3: "cat",
		piper_voices: vec![
			"ca_ES/upc_ona/medium",
			"ca_ES/upc_pau/x_low",
		],
	},
	Language {
		name: "Croatian",
		id_2: "hr",
		id_3: "hrv",
		piper_voices: vec![],
	},
	Language {
		name: "Czech",
		id_2: "cs",
		id_3: "ces",
		piper_voices: vec![],
	},
	Language {
		name: "Danish",
		id_2: "da",
		id_3: "dan",
		piper_voices: vec![
			"da_DK/talesyntese/medium"
		],
	},
	Language {
		name: "Dutch",
		id_2: "nl",
		id_3: "nld",
		piper_voices: vec![
			"nl_BE/nathalie/medium",
			"nl_BE/rdh/medium",
			"nl_NL/mls_5809/low",
			"nl_NL/mls_7432/low",
		],
	},
	Language {
		name: "English",
		id_2: "en",
		id_3: "eng",
		piper_voices: vec![
			// The models are currently all loaded into memory at once which would be 1 GB with all of these.
			"en_US/ryan/high",
			"en_GB/semaine/medium",
			"en_GB/alba/medium",
			"en_US/lessac/high",
			"en_GB/alan/medium",
			"en_US/libritts/high",
			// "en_US/amy/medium",
			// "en_US/kusal/medium",
			// "en_GB/jenny_dioco/medium",
			// "en_GB/northern_english_male/medium",
			// "en_GB/aru/medium",
			// "en_US/danny/low",
			// "en_US/joe/medium",
			// "en_GB/southern_english_female/medium",
			// "en_US/kathleen/low",
			// "en_US/arctic/medium",
			// "en_US/l2arctic/medium",
		],
	},
	Language {
		name: "Esperanto",
		id_2: "eo",
		id_3: "epo",
		piper_voices: vec![],
	},
	Language {
		name: "Estonian",
		id_2: "et",
		id_3: "est",
		piper_voices: vec![],
	},
	Language {
		name: "Finnish",
		id_2: "fi",
		id_3: "fin",
		piper_voices: vec![
			"fi_FI/harri/medium"
		],
	},
	Language {
		name: "French",
		id_2: "fr",
		id_3: "fra",
		piper_voices: vec![
			"fr_FR/siwis/medium",
			"fr_FR/upmc/medium",
			"fr_FR/gilles/low",
			"fr_FR/mls_1840/low",
		],
	},
	Language {
		name: "Galician",
		id_2: "gl",
		id_3: "glg",
		piper_voices: vec![],
	},
	Language {
		name: "Georgian",
		id_2: "ka",
		id_3: "kat",
		piper_voices: vec![
			"ka_GE/natia/medium"
		],
	},
	Language {
		name: "German",
		id_2: "de",
		id_3: "deu",
		piper_voices: vec![
			"de_DE/thorsten_emotional/medium",
			"de_DE/pavoque/low",
			"de_DE/eva_k/x_low",
			"de_DE/karlsson/low",
			"de_DE/kerstin/low",
			"de_DE/ramona/low",
		],
	},
	Language {
		name: "Greek",
		id_2: "el",
		id_3: "ell",
		piper_voices: vec![
			"el_GR/rapunzelina/low"
		],
	},
	Language {
		name: "Hebrew",
		id_2: "he",
		id_3: "heb",
		piper_voices: vec![],
	},
	Language {
		name: "Hindi",
		id_2: "hi",
		id_3: "hin",
		piper_voices: vec![],
	},
	Language {
		name: "Hungarian",
		id_2: "hu",
		id_3: "hun",
		piper_voices: vec![],
	},
	Language {
		name: "Icelandic",
		id_2: "is",
		id_3: "isl",
		piper_voices: vec![
			"is_IS/steinn/medium",
			"is_IS/bui/medium",
			"is_IS/salka/medium",
			"is_IS/ugla/medium",
		],
	},
	Language {
		name: "Indonesian",
		id_2: "id",
		id_3: "ind",
		piper_voices: vec![],
	},
	Language {
		name: "Italian",
		id_2: "it",
		id_3: "ita",
		piper_voices: vec![
			"it_IT/riccardo/x_low",
		],
	},
	Language {
		name: "Japanese",
		id_2: "ja",
		id_3: "jpn",
		piper_voices: vec![],
	},
	Language {
		name: "Kazakh",
		id_2: "kk",
		id_3: "kaz",
		piper_voices: vec![
			"kk_KZ/issai/high",
			"kk_KZ/iseke/x_low",
			"kk_KZ/raya/x_low",
		],
	},
	Language {
		name: "Korean",
		id_2: "ko",
		id_3: "kor",
		piper_voices: vec![],
	},
	Language {
		name: "Latvian",
		id_2: "lv",
		id_3: "lat",
		piper_voices: vec![],
	},
	Language {
		name: "Lithuanian",
		id_2: "lt",
		id_3: "lit",
		piper_voices: vec![],
	},
	Language {
		name: "Macedonian",
		id_2: "mk",
		id_3: "mkd",
		piper_voices: vec![],
	},
	Language {
		name: "Malay",
		id_2: "ms",
		id_3: "zsm",
		piper_voices: vec![],
	},
	Language {
		name: "Malayalam",
		id_2: "ml",
		id_3: "mal",
		piper_voices: vec![],
	},
	Language {
		name: "Norwegian Bokmål",
		id_2: "no",
		id_3: "nob",
		piper_voices: vec![
			"no_NO/talesyntese/medium",
		],
	},
	Language {
		name: "Norwegian Nynorsk",
		id_2: "no",
		id_3: "nno",
		piper_voices: vec![
			"no_NO/talesyntese/medium",
		],
	},
	Language {
		name: "Persian",
		id_2: "fa",
		id_3: "pes",
		piper_voices: vec![],
	},
	Language {
		name: "Polish",
		id_2: "pl",
		id_3: "pol",
		piper_voices: vec![
			"pl_PL/gosia/medium",
			"pl_PL/darkman/medium",
			"pl_PL/mls_6892/low",
		],
	},
	Language {
		name: "Portuguese",
		id_2: "pt",
		id_3: "por",
		piper_voices: vec![
			"pt_BR/faber/medium",
			"pt_BR/edresson/low",
		],
	},
	Language {
		name: "Romanian",
		id_2: "ro",
		id_3: "ron",
		piper_voices: vec![
			"ro_RO/mihai/medium"
		],
	},
	Language {
		name: "Russian",
		id_2: "ru",
		id_3: "rus",
		piper_voices: vec![
			"ru_RU/irina/medium",
			"ru_RU/ruslan/medium",
			"ru_RU/dmitri/medium",
			"ru_RU/denis/medium",
		],
	},
	Language {
		name: "Serbian",
		id_2: "sr",
		id_3: "srp",
		piper_voices: vec![
			"sr_RS/serbski_institut/medium"
		],
	},
	Language {
		name: "Sinhala",
		id_2: "si",
		id_3: "sin",
		piper_voices: vec![],
	},
	Language {
		name: "Slovak",
		id_2: "sk",
		id_3: "slk",
		piper_voices: vec![],
	},
	Language {
		name: "Slovenian",
		id_2: "sl",
		id_3: "slv",
		piper_voices: vec![],
	},
	Language {
		name: "Spanish",
		id_2: "es",
		id_3: "spa",
		piper_voices: vec![
			"es_ES/sharvard/medium",
			"es_MX/ald/medium",
			"es_ES/davefx/medium",
			"es_ES/carlfm/x_low",
			"es_ES/mls_9972/low",
			"es_ES/mls_10246/low",
		],
	},
	Language {
		name: "Swedish",
		id_2: "sv",
		id_3: "swe",
		piper_voices: vec![
			"sv_SE/nst/medium",
		],
	},
	Language {
		name: "Tagalog",
		id_2: "tl",
		id_3: "tgl",
		piper_voices: vec![],
	},
	Language {
		name: "Tamil",
		id_2: "ta",
		id_3: "tam",
		piper_voices: vec![],
	},
	Language {
		name: "Telugu",
		id_2: "te",
		id_3: "tel",
		piper_voices: vec![],
	},
	Language {
		name: "Thai",
		id_2: "th",
		id_3: "tha",
		piper_voices: vec![],
	},
	Language {
		name: "Turkish",
		id_2: "tr",
		id_3: "tur",
		piper_voices: vec![
			"tr_TR/dfki/medium",
		],
	},
	Language {
		name: "Ukranian",
		id_2: "uk",
		id_3: "ukr",
		piper_voices: vec![
			"uk_UA/ukrainian_tts/medium",
		],
	},
	Language {
		name: "Urdu",
		id_2: "ur",
		id_3: "urd",
		piper_voices: vec![],
	},
	Language {
		name: "Vietnamese",
		id_2: "vi",
		id_3: "vie",
		piper_voices: vec![
			"vi_VN/vais1000/medium",
			"vi_VN/25hours_single/low",
			"vi_VN/vivos/x_low",
		],
	},
]);

//----------------------------------------------------------------

/*
	Used as input to the procedure that fetches word and sentence data for a particular target language and translation language(s).
*/
#[derive(Deserialize, Serialize)]
pub struct SourceDataInfo {
	// The language that the user wants to learn.
	pub target_language: String,
	// The language(s) that the user knows and wants to see translations in for this particular target language.
	pub translation_languages: Vec<String>,
}

/*
	Source file data with words and sentences for a particular target language.
	This data is parsed in learning_data.rs.
*/
pub struct SourceData {
	// The index of the target language in the LANGUAGES array above.
	pub language_index: usize,
	// The word frequency list file data fetched from the internet.
	pub word_list: Vec<u8>,
	// The concatenated file data for the lists of sentences in the target language together with translations in the translation languages.
	pub sentence_list: Vec<u8>,
}

impl SourceData {
	pub async fn download<F>(info: SourceDataInfo, status_callback: F) -> Self 
		where F: Fn(SourceDataDownloadStatus)
	{
		SourceDataDownloader::new(info, status_callback).download().await
	}
}

//----------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub enum SourceDataDownloadStatus {
	DownloadingWords { progress: f32 },
	PreparingSentenceFile { translation_language: String },
	DownloadingSentenceFile { translation_language: String, progress: f32 },
	DownloadingVoiceModel { index: usize, total: usize, progress: f32 },
	Loading,
	Finished,
}

struct SourceDataDownloader<F: Fn(SourceDataDownloadStatus)> {
	client: reqwest::Client,
	target_language_index: usize,
	info: SourceDataInfo,
	status_callback: F
}

fn vec_with_optional_capacity<T>(capacity: Option<u64>) -> Vec<T> {
	if let Some(capacity) = capacity { 
		Vec::with_capacity(capacity as usize) 
	} else {
		Vec::new()
	}
}
fn optional_progress(length_received: usize, total_length: Option<u64>) -> f32 {
	if let Some(total_length) = total_length {
		length_received as f32 / total_length as f32
	} else {
		length_received as f32
	} 
}

impl<F: Fn(SourceDataDownloadStatus)> SourceDataDownloader<F> {
	fn new(info: SourceDataInfo, status_callback: F) -> Self {
		SourceDataDownloader {
			client: reqwest::Client::new(),
			target_language_index: LANGUAGES.iter().position(|language| language.name == info.target_language).unwrap(),
			info,
			status_callback
		}
	}

	async fn download(&self) -> SourceData {
		let word_list = self.download_words().await;
		let sentence_list = self.download_sentence_lists().await;
		self.download_piper_voices().await;

		SourceData {
			language_index: self.target_language_index,
			word_list,
			sentence_list,
		}
	}
	
	async fn download_words(&self) -> Vec<u8> {
		let language = &LANGUAGES[self.target_language_index];

		// GitHub repository for the word frequency data by Hermit Dave: https://github.com/hermitdave/FrequencyWords/tree/master.
		let words_url = "https://raw.githubusercontent.com/hermitdave/FrequencyWords/master/content/2018";

		let response = self.client.get(format!("{0}/{1}/{1}_50k.txt", words_url, language.id_2)).send().await;

		let mut response = 
			if let Ok(response) = response && response.status().is_success() { 
				response
			} else {
				self.client.get(format!("{0}/{1}/{1}_full.txt", words_url, language.id_2)).send().await.unwrap()
			};

		let length = response.content_length();
			
		let mut word_list_data = vec_with_optional_capacity(length);

		while let Ok(Some(chunk)) = response.chunk().await {
			word_list_data.extend_from_slice(&chunk);

			(self.status_callback)(SourceDataDownloadStatus::DownloadingWords { 
				progress: optional_progress(word_list_data.len(), length)
			});
		}

		word_list_data
	}

	async fn download_sentence_lists(&self) -> Vec<u8> {
		let mut sentence_lists = Vec::new();
		
		let translation_languages = self.info.translation_languages.iter()
			.map(|name| LANGUAGES.iter().find(|language| language.name == name).unwrap());
		
		for translation_language in translation_languages {
			(self.status_callback)(SourceDataDownloadStatus::PreparingSentenceFile { 
				translation_language: translation_language.name.to_owned()
			});
			let list = self.download_sentence_list(translation_language).await;
			// Strip BOM.
			let list = list.strip_prefix("\u{feff}".as_bytes()).unwrap_or(&list);
			sentence_lists.extend_from_slice(list);
		}

		sentence_lists
	}

	async fn download_sentence_list(&self, translation_language: &Language) -> Vec<u8> {
		let list_id = self.prepare_sentence_list(translation_language).await;
		
		let filename = self.wait_for_sentence_list_filename(list_id).await;

		let mut response = self.client.get(format!("https://tatoeba.org/en/exports/download/{}/{}", list_id, filename))
			.header("cookie", "csrfToken=a")
			.header("x-csrf-token", "a")
			.send().await.unwrap();
		
		let length = response.content_length();
		
		let mut sentence_list_data = vec_with_optional_capacity(length);

		while let Ok(Some(chunk)) = response.chunk().await {
			sentence_list_data.extend_from_slice(&chunk);

			(self.status_callback)(SourceDataDownloadStatus::DownloadingSentenceFile {
				translation_language: translation_language.name.to_owned(),
				progress: optional_progress(sentence_list_data.len(), length)
			});
		}
		
		sentence_list_data
	}

	async fn prepare_sentence_list(&self, translation_language: &Language) -> i64 {
		let response = self.client.post("https://tatoeba.org/en/exports/add")
		    .form(&[
		        ("fields[]", "id"),
		        ("fields[]", "text"),
		        ("fields[]", "trans_id"),
		        ("fields[]", "trans_text"),
		        ("format", "tsv"),
		        ("from", &LANGUAGES[self.target_language_index].id_3),
		        ("to", &translation_language.id_3),
		        ("type", "pairs")
		    ])
		    .header("cookie", "csrfToken=a")
		    .header("x-csrf-token", "a")
		    .send().await.unwrap()
		    .json::<serde_json::Value>().await.unwrap();

		response["export"]["id"].as_i64().unwrap()
	}

	async fn wait_for_sentence_list_filename(&self, id: i64) -> String {
		loop {
			let response = self.client.get(format!("https://tatoeba.org/en/exports/get/{}", id))
				.header("cookie", "csrfToken=a")
				.header("x-csrf-token", "a")
				.send().await.unwrap()
				.json::<serde_json::Value>().await.unwrap();
			
			let export = &response["export"];

			if export["status"] == "online" {
				return export["pretty_filename"].to_string();
			}

			std::thread::sleep(std::time::Duration::from_secs(1));
		}
	}

	async fn download_piper_voices(&self) {
		let language = &LANGUAGES[self.target_language_index];

		for (i, model) in language.piper_voices.iter().enumerate() {
			let mut file_name = model.replace('/', "-") + ".onnx";
			let mut url = format!("https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/{}/{}/{}", language.id_2, model, file_name);
			self.download_piper_voice(&url, &file_name, i, false).await;

			file_name += ".json";
			url += ".json";
			self.download_piper_voice(&url, &file_name, i, true).await;
		}
	}

	async fn download_piper_voice(&self, url: &str, file_name: &str, index: usize, is_configuration: bool) {
		let mut response = self.client.get(url).send().await.unwrap();

		let length = response.content_length();

		let directory = format!("{}/voices", SAVE_DIRECTORY);
		fs::create_dir_all(&directory).unwrap();
		
		let file = fs::File::options().read(false).write(true).create_new(true).open(directory + "/" + file_name);
		let mut file = match file {
			Ok(file) => file,
			Err(error) => match error.kind() {
				std::io::ErrorKind::AlreadyExists => return,
				_ => panic!("Error creating piper voice file: {}", error)
			}
		};

		let mut file_data = vec_with_optional_capacity(length);

		while let Ok(Some(chunk)) = response.chunk().await {
			/*
				I am not sure if we should write the chunks directly to the file or read them in a buffer first.
				For now we do the latter to avoid partially written files if the program was shut down in the middle of downloading 
				or something.
			 */
			// file.write_all(&chunk).unwrap();
			file_data.extend_from_slice(&chunk);

			// Configuration files are so small in comparison that they don't need any progress feedback.
			if !is_configuration {
				(self.status_callback)(SourceDataDownloadStatus::DownloadingVoiceModel { 
					index, 
					total: LANGUAGES[self.target_language_index].piper_voices.len(), 
					progress: optional_progress(file_data.len(), length)
				});
			}
		}

		file.write_all(&file_data).unwrap();
	}
}
