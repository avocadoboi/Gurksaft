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
        name: "Arabic",
        id: "ar",
    },
    Language {
        name: "Bulgarian",
        id: "bg",
    },
    Language {
        name: "English",
        id: "en",
    },
    Language {
        name: "Finnish",
        id: "fi",
    },
    Language {
        name: "Swedish",
        id: "sv",
    },
];

struct SourceData {
    
}

// https://raw.githubusercontent.com/hermitdave/FrequencyWords/master/content/2018/sv/sv_50k.txt
