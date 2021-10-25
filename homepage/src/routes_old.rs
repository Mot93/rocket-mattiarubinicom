// Templates 
use rocket_contrib::templates::Template;

// Serialization needed for templates
use serde_json;

//
use super::context;

//TODO: italian
#[get("/")]
pub fn landing_page() -> Template {
    let mut min_context = context::Context::new(String::from("Home"), context::Language::English);
    min_context.add_stylesheet(String::from("css/pages/landing_page.css"));
    let context = serde_json::json!(min_context);
    Template::render("pages/landing_page", &context)
}