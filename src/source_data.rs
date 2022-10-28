use serde::{Deserialize, Serialize};
use serde_json::Value;

//----------------------------------------------------------------

#[derive(Deserialize, Serialize)]
pub struct Language {
	pub name: &'static str,
	pub id: &'static str,
}

pub const LANGUAGES: &[Language] = &[
	Language {
		name: "Afrikaans",
		id: "af",
	},
	Language {
		name: "Albanian",
		id: "sq",
	},
	Language {
		name: "Arabic",
		id: "ar",
	},
	Language {
		name: "Armenian",
		id: "hy",
	},
	Language {
		name: "Basque",
		id: "eu",
	},
	Language {
		name: "Bengali",
		id: "bn",
	},
	Language {
		name: "Bosnian",
		id: "bs",
	},
	Language {
		name: "Breton",
		id: "br",
	},
	Language {
		name: "Bulgarian",
		id: "bg",
	},
	Language {
		name: "Catalan",
		id: "ca",
	},
	Language {
		name: "Croatian",
		id: "hr",
	},
	Language {
		name: "Czech",
		id: "cs",
	},
	Language {
		name: "Danish",
		id: "da",
	},
	Language {
		name: "Dutch",
		id: "nl",
	},
	Language {
		name: "English",
		id: "en",
	},
	Language {
		name: "Esperanto",
		id: "eo",
	},
	Language {
		name: "Estonian",
		id: "et",
	},
	Language {
		name: "Finnish",
		id: "fi",
	},
	Language {
		name: "French",
		id: "fr",
	},
	Language {
		name: "Galician",
		id: "gl",
	},
	Language {
		name: "Georgian",
		id: "ka",
	},
	Language {
		name: "German",
		id: "de",
	},
	Language {
		name: "Greek",
		id: "el",
	},
	Language {
		name: "Hebrew",
		id: "he",
	},
	Language {
		name: "Hindi",
		id: "hi",
	},
	Language {
		name: "Hungarian",
		id: "hu",
	},
	Language {
		name: "Icelandic",
		id: "is",
	},
	Language {
		name: "Indonesian",
		id: "id",
	},
	Language {
		name: "Italian",
		id: "it",
	},
	Language {
		name: "Japanese",
		id: "ja",
	},
	Language {
		name: "Kazakh",
		id: "kk",
	},
	Language {
		name: "Korean",
		id: "ko",
	},
	Language {
		name: "Latvian",
		id: "lv",
	},
	Language {
		name: "Lithuanian",
		id: "lt",
	},
	Language {
		name: "Macedonian",
		id: "mk",
	},
	Language {
		name: "Malay",
		id: "ms",
	},
	Language {
		name: "Malayalam",
		id: "ml",
	},
	Language {
		name: "Norwegian",
		id: "no",
	},
	Language {
		name: "Persian",
		id: "fa",
	},
	Language {
		name: "Polish",
		id: "pl",
	},
	Language {
		name: "Portuguese",
		id: "pt",
	},
	Language {
		name: "Romanian",
		id: "ro",
	},
	Language {
		name: "Russian",
		id: "ru",
	},
	Language {
		name: "Serbian",
		id: "sr",
	},
	Language {
		name: "Sinhala",
		id: "si",
	},
	Language {
		name: "Slovak",
		id: "sk",
	},
	Language {
		name: "Slovenian",
		id: "sl",
	},
	Language {
		name: "Spanish",
		id: "es",
	},
	Language {
		name: "Swedish",
		id: "sv",
	},
	Language {
		name: "Tagalog",
		id: "tl",
	},
	Language {
		name: "Tamil",
		id: "ta",
	},
	Language {
		name: "Telugu",
		id: "te",
	},
	Language {
		name: "Thai",
		id: "th",
	},
	Language {
		name: "Turkish",
		id: "tr",
	},
	Language {
		name: "Ukranian",
		id: "uk",
	},
	Language {
		name: "Urdu",
		id: "ur",
	},
	Language {
		name: "Vietnamese",
		id: "vi",
	},
];

//----------------------------------------------------------------

#[derive(Deserialize, Serialize)]
pub struct SourceDataInfo {
	pub target_language: String,
	pub translation_languages: Vec<String>,
}

pub struct SourceData {
	pub info: SourceDataInfo,
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
	info: SourceDataInfo,
	status_callback: F
}

impl<F: Fn(SourceDataDownloadStatus)> SourceDataDownloader<F> {
	fn new(info: SourceDataInfo, status_callback: F) -> Self {
		SourceDataDownloader {
			client: reqwest::Client::new(),
			info,
			status_callback
		}
	}

	async fn download(mut self) -> SourceData {		
		let word_list = self.download_words().await;
		let sentence_list = self.download_sentence_lists().await;

		SourceData {
			info: self.info,
			word_list,
			sentence_list,
		}
	}
	
	async fn download_words(&self) -> Vec<u8> {
		let language = LANGUAGES.iter().find(|language| language.name == self.info.target_language).unwrap();

		let words_url = "https://raw.githubusercontent.com/hermitdave/FrequencyWords/master/content/2018";

		let response = self.client.get(format!("{0}/{1}/{1}_50k.txt", words_url, language.id))
			.send().await;

		let mut response = 
			if let Ok(response) = response && response.status().is_success() { 
				response
			} else {
				self.client.get(format!("{0}/{1}/{1}_full.txt", words_url, language.id)).send().await.unwrap()
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

	async fn download_sentence_lists(&mut self) -> Vec<u8> {
		let mut sentence_lists = Vec::new();
		
		for translation_language in &self.info.translation_languages {
			(self.status_callback)(SourceDataDownloadStatus::PreparingSentenceFile { 
				translation_language: translation_language.clone()
			});
			sentence_lists.append(&mut self.download_sentence_list(&translation_language).await);
		}

		sentence_lists
	}

	async fn download_sentence_list(&self, translation_language: &str) -> Vec<u8> {
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
				translation_language: translation_language.to_owned(),
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

	async fn prepare_sentence_list(&self, translation_language: &str) -> i64 {
		let response = self.client.post("https://tatoeba.org/en/exports/add")
		    .form(&[
		        ("fields[]", "id"),
		        ("fields[]", "text"),
		        ("fields[]", "trans_id"),
		        ("fields[]", "trans_text"),
		        ("format", "tsv"),
		        ("from", &self.info.target_language[..3].to_lowercase()),
		        ("to", &translation_language[..3].to_lowercase()),
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
