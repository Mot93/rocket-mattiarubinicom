use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use serde_json::json;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    let context = json!({"name": "foo"});
    Template::render("pages/landing_page", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // Mounting routes
        .mount("/", routes![index])
        // Serving static files
        .mount("/css", FileServer::from("./static/css"))
        .mount("/images", FileServer::from("./static/images"))
        .mount("/js", FileServer::from("./static/js"))
        // Ataching the template
        .attach(Template::fairing())
}