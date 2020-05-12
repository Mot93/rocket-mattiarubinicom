// Templates 
use rocket_contrib::templates::Template;

// Serialization needed for templates
use serde_json;

//
use super::context;

#[get("/")]
pub fn index() -> Template {
    let mut min_context = context::MinContext::new(String::from("Home"), context::Language::English);
    min_context.add_stylesheet(String::from("css/pages/index.css"));
    let context = serde_json::json!(min_context);
    Template::render("pages/index", &context)
}