// Serializationj needed for templates
use serde::{Deserialize, Serialize};

// Context for each page
#[derive(Serialize, Deserialize)]
pub struct MinContext {
    title: String,
    styles: Vec<String>,
    scripts: Vec<String>,
}

impl MinContext {

    pub fn new(title: String) -> MinContext {
        MinContext{
            title: title,
            styles: Vec::new(),
            scripts: Vec::new(),
        }
    }

    pub fn add_stylesheet(& mut self, stylesheet: String) {
        self.styles.push(stylesheet);
    }

}