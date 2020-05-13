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

// TODO: italian
#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    // Creating the minimum context
    let mut min_context = context::Context::new(String::from("404"), context::Language::English);
    // Adding stylesheet
    min_context.add_stylesheet(String::from("css/pages/errors/404.css"));
    // Adding page specific context
    let uri = format!("{}", req.uri());
    let page_context = serde_json::json!({
        "uri_not_found" : uri,
    });
    min_context.add_page_context(serde_json::json!(page_context));
    // Giving back the page
    let context = serde_json::json!(min_context);
    Template::render("pages/errors/404", &context)
}