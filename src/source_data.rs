use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Language {
    pub name: &'static str,
    pub id: &'static str,
}

const LANGUAGES: &[Language] = &[
    Language {
        name: "afrikaans",
        id: "af",
    },
    Language {
        name: "arabic",
        id: "ar",
    },
    Language {
        name: "bulgarian",
        id: "bg",
    },
    Language {
        name: "english",
        id: "en",
    },
    Language {
        name: "finnish",
        id: "fi",
    },
    Language {
        name: "swedish",
        id: "sv",
    },
];

struct SourceData {
    
}

// https://raw.githubusercontent.com/hermitdave/FrequencyWords/master/content/2018/sv/sv_50k.txt
