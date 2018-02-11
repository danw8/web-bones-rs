#![feature(plugin, custom_derive)]
#![feature(proc_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate maud;
extern crate bcrypt;
extern crate data;
extern crate service;
extern crate guard;
extern crate transfer;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod index;
pub use self::index::*;

pub mod files;
pub use self::files::*;

pub mod api;
pub use self::api::*;