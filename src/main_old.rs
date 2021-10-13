// Specifie the features used from the nightly build
#![feature(proc_macro_hygiene, decl_macro)]

// Extern the rocket crate and all of its macro
#[macro_use] extern crate rocket;
// Other rocket features needed
use rocket_contrib::templates::Template;
// Serving static files such as css or js
use rocket_contrib::serve::StaticFiles;

// Mods
mod context;
mod routes;
mod catchers;

fn main() {
    // Creating the rocket
    let spacex = rocket::ignite()
        // templates
        .attach(Template::fairing())
        // errors
        .register(catchers![catchers::not_found])
        // static files (css, js, images, etc)
        .mount("/", StaticFiles::from("static"))
        // routes
        .mount("/", routes![routes::landing_page]);
    // Starting the webserver
    spacex.launch();
}