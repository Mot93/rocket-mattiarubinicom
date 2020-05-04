// Specifie the features used from the nightly build
#![feature(proc_macro_hygiene, decl_macro)]

// Extern the rocket crate and all of its macro
#[macro_use] extern crate rocket;

// Tamplates 
use rocket_contrib::templates::Template;

// Serializationj needed for templates
#[macro_use] extern crate serde_json;

// Serving static files such as css or js
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> Template {
    let context = json!({
        "title": "Home"
    });
    Template::render("index", &context)
}

fn main() {
    // Creating the rocket
    let spacex = rocket::ignite()
        // templates
        .attach(Template::fairing())
        // static files (css, js, images, etc)
        .mount("/", StaticFiles::from("static"))
        // routes
        .mount("/", routes![index]);
    // Starting the webserver
    spacex.launch();
}