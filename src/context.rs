// Serializationj needed for templates
use serde::{Deserialize, Serialize};

pub enum Language{
    English,
    Italiano,
}

impl Language {

    pub fn language_code(&self) -> String {
        match self {
            Language::English => String::from("en"),
            Language::Italiano => String::from("it"),
        }
    }

}

// Context for each page
#[derive(Serialize, Deserialize)]
pub struct MinContext {
    title: String,
    styles: Vec<String>,
    scripts: Vec<String>,
    language_code: String,
}

impl MinContext {

    pub fn new(title: String, lang: Language) -> MinContext {
        MinContext{
            title: title,
            styles: Vec::new(),
            scripts: Vec::new(),
            language_code: lang.language_code(),
        }
    }

    pub fn add_stylesheet(& mut self, stylesheet: String) {
        self.styles.push(stylesheet);
    }

}