// Specifie the features used from the nightly build
#![feature(proc_macro_hygiene, decl_macro)]

// Extern the rocket crate and all of its macro
#[macro_use] extern crate rocket;

// Tamplates 
use rocket_contrib::templates::Template;

#[macro_use] extern crate serde_json;

#[get("/")]
fn index() -> Template {
    let context = json!({
        "title": "Home"
    });
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index]).launch();
}
