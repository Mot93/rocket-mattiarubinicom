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
pub struct Context {
    // These are the required field for each page
    title: String,
    styles: Vec<String>,
    scripts: Vec<String>,
    language_code: String,
    // All the addition field required specifically for that page
    page_contex: Option<serde_json::Value>,
}

impl Context {

    /// TODO:
    pub fn new(title: String, lang: Language) -> Context {
        Context{
            title: title,
            styles: Vec::new(),
            scripts: Vec::new(),
            language_code: lang.language_code(),
            page_contex: None,
        }
    }

    /// TODO::
    pub fn add_stylesheet(& mut self, stylesheet: String) {
        self.styles.push(stylesheet);
    }

    /// TODO
    pub fn add_page_context(& mut self, context: serde_json::Value) {
        self.page_contex = Some(context);
    }

}