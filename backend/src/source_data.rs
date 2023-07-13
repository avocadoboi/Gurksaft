use serde::{Deserialize, Serialize};

//----------------------------------------------------------------

#[derive(Deserialize, Serialize)]
pub struct Language {
	pub name: &'static str,
	// Two-letter ISO 639-1 language code.
	pub id_2: &'static str,
	// Three-letter ISO 639-2 language code.
	pub id_3: &'static str,
}

pub const LANGUAGES: &[Language] = &[
	Language {
		name: "Afrikaans",
		id_2: "af",
		id_3: "afr",
	},
	Language {
		name: "Albanian",
		id_2: "sq",
		id_3: "sqi",
	},
	Language {
		name: "Arabic",
		id_2: "ar",
		id_3: "ara",
	},
	Language {
		name: "Armenian",
		id_2: "hy",
		id_3: "hye",
	},
	Language {
		name: "Basque",
		id_2: "eu",
		id_3: "eus",
	},
	Language {
		name: "Bengali",
		id_2: "bn",
		id_3: "ben",
	},
	Language {
		name: "Bosnian",
		id_2: "bs",
		id_3: "bos",
	},
	Language {
		name: "Breton",
		id_2: "br",
		id_3: "bre",
	},
	Language {
		name: "Bulgarian",
		id_2: "bg",
		id_3: "bul",
	},
	Language {
		name: "Catalan",
		id_2: "ca",
		id_3: "cat",
	},
	Language {
		name: "Croatian",
		id_2: "hr",
		id_3: "hrv",
	},
	Language {
		name: "Czech",
		id_2: "cs",
		id_3: "ces",
	},
	Language {
		name: "Danish",
		id_2: "da",
		id_3: "dan",
	},
	Language {
		name: "Dutch",
		id_2: "nl",
		id_3: "nld",
	},
	Language {
		name: "English",
		id_2: "en",
		id_3: "eng",
	},
	Language {
		name: "Esperanto",
		id_2: "eo",
		id_3: "epo",
	},
	Language {
		name: "Estonian",
		id_2: "et",
		id_3: "est",
	},
	Language {
		name: "Finnish",
		id_2: "fi",
		id_3: "fin",
	},
	Language {
		name: "French",
		id_2: "fr",
		id_3: "fra",
	},
	Language {
		name: "Galician",
		id_2: "gl",
		id_3: "glg",
	},
	Language {
		name: "Georgian",
		id_2: "ka",
		id_3: "kat",
	},
	Language {
		name: "German",
		id_2: "de",
		id_3: "deu",
	},
	Language {
		name: "Greek",
		id_2: "el",
		id_3: "ell",
	},
	Language {
		name: "Hebrew",
		id_2: "he",
		id_3: "heb",
	},
	Language {
		name: "Hindi",
		id_2: "hi",
		id_3: "hin",
	},
	Language {
		name: "Hungarian",
		id_2: "hu",
		id_3: "hun",
	},
	Language {
		name: "Icelandic",
		id_2: "is",
		id_3: "isl",
	},
	Language {
		name: "Indonesian",
		id_2: "id",
		id_3: "ind",
	},
	Language {
		name: "Italian",
		id_2: "it",
		id_3: "ita",
	},
	Language {
		name: "Japanese",
		id_2: "ja",
		id_3: "jpn",
	},
	Language {
		name: "Kazakh",
		id_2: "kk",
		id_3: "kaz",
	},
	Language {
		name: "Korean",
		id_2: "ko",
		id_3: "kor",
	},
	Language {
		name: "Latvian",
		id_2: "lv",
		id_3: "lav",
	},
	Language {
		name: "Lithuanian",
		id_2: "lt",
		id_3: "lit",
	},
	Language {
		name: "Macedonian",
		id_2: "mk",
		id_3: "mkd",
	},
	Language {
		name: "Malay",
		id_2: "ms",
		id_3: "msa",
	},
	Language {
		name: "Malayalam",
		id_2: "ml",
		id_3: "mal",
	},
	Language {
		name: "Norwegian",
		id_2: "no",
		id_3: "nor",
	},
	Language {
		name: "Persian",
		id_2: "fa",
		id_3: "fas",
	},
	Language {
		name: "Polish",
		id_2: "pl",
		id_3: "pol",
	},
	Language {
		name: "Portuguese",
		id_2: "pt",
		id_3: "por",
	},
	Language {
		name: "Romanian",
		id_2: "ro",
		id_3: "ron",
	},
	Language {
		name: "Russian",
		id_2: "ru",
		id_3: "rus",
	},
	Language {
		name: "Serbian",
		id_2: "sr",
		id_3: "srp",
	},
	Language {
		name: "Sinhala",
		id_2: "si",
		id_3: "sin",
	},
	Language {
		name: "Slovak",
		id_2: "sk",
		id_3: "slk",
	},
	Language {
		name: "Slovenian",
		id_2: "sl",
		id_3: "slv",
	},
	Language {
		name: "Spanish",
		id_2: "es",
		id_3: "spa",
	},
	Language {
		name: "Swedish",
		id_2: "sv",
		id_3: "swe",
	},
	Language {
		name: "Tagalog",
		id_2: "tl",
		id_3: "tgl",
	},
	Language {
		name: "Tamil",
		id_2: "ta",
		id_3: "tam",
	},
	Language {
		name: "Telugu",
		id_2: "te",
		id_3: "tel",
	},
	Language {
		name: "Thai",
		id_2: "th",
		id_3: "tha",
	},
	Language {
		name: "Turkish",
		id_2: "tr",
		id_3: "tur",
	},
	Language {
		name: "Ukranian",
		id_2: "uk",
		id_3: "ukr",
	},
	Language {
		name: "Urdu",
		id_2: "ur",
		id_3: "urd",
	},
	Language {
		name: "Vietnamese",
		id_2: "vi",
		id_3: "vie",
	},
];

//----------------------------------------------------------------

#[derive(Deserialize, Serialize)]
pub struct SourceDataInfo {
	pub target_language: String,
	pub translation_languages: Vec<String>,
}

pub struct SourceData {
	pub language_index: usize,
	pub word_list: Vec<u8>,
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
	DownloadingWords { progress: f64 },
	PreparingSentenceFile { translation_language: String },
	DownlodingSentenceFile { translation_language: String, progress: f64 },
	Loading,
}

struct SourceDataDownloader<F: Fn(SourceDataDownloadStatus)> {
	client: reqwest::Client,
	target_language_index: usize,
	info: SourceDataInfo,
	status_callback: F
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

		SourceData {
			language_index: self.target_language_index,
			word_list,
			sentence_list,
		}
	}
	
	async fn download_words(&self) -> Vec<u8> {
		let language = &LANGUAGES[self.target_language_index];

		let words_url = "https://raw.githubusercontent.com/hermitdave/FrequencyWords/master/content/2018";

		let response = self.client.get(format!("{0}/{1}/{1}_50k.txt", words_url, language.id_2)).send().await;

		let mut response = 
			if let Ok(response) = response && response.status().is_success() { 
				response
			} else {
				self.client.get(format!("{0}/{1}/{1}_full.txt", words_url, language.id_2)).send().await.unwrap()
			};

		let length = response.content_length();
			
		let mut word_list_data = 
			if let Some(length) = length { 
				Vec::with_capacity(length as usize) 
			} else {
				Vec::new()
			};

		while let Ok(Some(chunk)) = response.chunk().await {
			word_list_data.extend_from_slice(&chunk);

			(self.status_callback)(SourceDataDownloadStatus::DownloadingWords { 
				progress: 
					if let Some(length) = length {
						word_list_data.len() as f64 / length as f64
					} else {
						word_list_data.len() as f64
					} 
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
		
		let mut sentence_list_data = 
			if let Some(length) = length { 
				Vec::with_capacity(length as usize) 
			} else {
				Vec::new()
			};

		while let Ok(Some(chunk)) = response.chunk().await {
			sentence_list_data.extend_from_slice(&chunk);

			(self.status_callback)(SourceDataDownloadStatus::DownlodingSentenceFile {
				translation_language: translation_language.name.to_owned(),
				progress: 
					if let Some(length) = length {
						sentence_list_data.len() as f64 / length as f64
					} else {
						sentence_list_data.len() as f64
					} 
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
}
