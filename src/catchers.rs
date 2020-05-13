// Templates 
use rocket_contrib::templates::Template;

// Serialization needed for templates
use serde_json;

// 
use rocket::Request;

use super::context;

/* TODO:
#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}
*/

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut min_context = context::MinContext::new(String::from("404"), context::Language::English);
    min_context.add_stylesheet(String::from("css/pages/errors/404.css"));
    let context = serde_json::json!(min_context);
    Template::render("pages/errors/404", &context)
}