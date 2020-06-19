#![feature(plugin)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate r2d2;
extern crate dotenv;
extern crate rocket;

pub mod model;
pub mod schema;

use diesel::PgConnection;
use r2d2::{ Pool, PooledConnection};
use diesel::r2d2::ConnectionManager;
use rocket::request::{Outcome, FromRequest, Request};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;
use dotenv::dotenv;
use std::env;



lazy_static! {
	pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool().unwrap();
}

pub struct DataBase(pub PooledConnection<ConnectionManager<PgConnection>>);

impl DataBase {
    pub fn connection(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DataBase {
    type Error = r2d2::Error;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(db_connection) => Success(DataBase(db_connection)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
}

fn create_db_pool() -> Result<Pool<ConnectionManager<PgConnection>>, String> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    // let pool_config = Config::default();
	let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
    match r2d2::Pool::builder().build(manager) {
        Ok(p) => Ok(p),
        Err(_) => Err(format!("Error creating connection pool with connection string '{}'", database_url))
    }
}
