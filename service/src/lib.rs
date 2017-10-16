// External Crates
extern crate rocket;
extern crate diesel;
extern crate r2d2;
extern crate bcrypt;

// Internal Crates
extern crate data;

pub mod user;
pub use self::user::*;