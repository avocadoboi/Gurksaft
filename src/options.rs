
const OPTIONS_SAVE_FILE: &str = "save_data/options";

pub struct Options {

}

impl Options {
    pub fn exists_save_file() -> bool {
        std::path::Path::new(OPTIONS_SAVE_FILE).is_file()
    }

    pub fn load() -> Self {

    }
}
