use serde::{Deserialize, Serialize};

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

struct SourceData {
    
}

// https://raw.githubusercontent.com/hermitdave/FrequencyWords/master/content/2018/sv/sv_50k.txt
