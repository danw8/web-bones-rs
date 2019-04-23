#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_codegen;
extern crate route;



fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                route::index,
                route::files,
                route::login,
            ],
        )
        .launch();
}
