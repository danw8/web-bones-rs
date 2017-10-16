#![feature(plugin)]
#![feature(const_atomic_bool_new)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate route;
use route::*;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                files,
            ],
        )
        .launch();
}
