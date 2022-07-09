use rustweb::{routes, catchers};

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes()).register("/", catchers())
}