use std::collections::HashMap;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("pages/landing_page", context)
}

#[get("/about-me")]
fn aboutme() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("pages/about_me", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // Mounting routes
        .mount("/", routes![index])
        .mount("/", routes![aboutme])
        // Serving static files
        .mount("/css", FileServer::from("./static/css"))
        .mount("/images", FileServer::from("./static/images"))
        .mount("/js", FileServer::from("./static/js"))
        // Ataching the template
        .attach(Template::fairing())
}