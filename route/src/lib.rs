#![feature(plugin, custom_derive)]
#![feature(proc_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate maud;
extern crate bcrypt;
extern crate data;
extern crate service;
extern crate guard;

pub mod index;
pub use self::index::*;

pub mod files;
pub use self::files::*;
