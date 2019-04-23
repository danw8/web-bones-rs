#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]
extern crate bcrypt;
extern crate data;
extern crate guard;
extern crate maud;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_codegen;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate service;
extern crate transfer;

pub mod index;
pub use self::index::*;

pub mod files;
pub use self::files::*;

pub mod api;
pub use self::api::*;
