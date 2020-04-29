// Specifie the features used from the nightly build
#![feature(proc_macro_hygiene, decl_macro)]

// Extern the rocket crate and all of its macro
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
