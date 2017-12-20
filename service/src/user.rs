use r2d2::{ Error };
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::{Cookies, Cookie, Status};
use rocket::Request;
use diesel;
use diesel::prelude::*;
use bcrypt::verify;

use data::*;
use data::schema::*;
use data::model::*;
//use logic::Permission as e_Permission;

pub struct UserService{
    pub db: DataBase,
	current_user: Option<User>,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserService {
	type Error = Error;
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		let mut service = match DB_POOL.get() {
			Ok(db_connection) => UserService {
				db: DataBase(db_connection),
				current_user: None
			},
			Err(e) => return Failure((Status::InternalServerError, e)),
		};
		service.current_user = service.get_session_user(request.cookies());
		Success(service)
	}
}

impl UserService {
	fn user_from_id(&self, id: i32) -> Option<User> {
		match users::table.filter(users::id.eq(id)).limit(1).load::<User>(self.db.connection()) {
			Ok(u) => u,
			Err(_) => return None,
		}.pop()
	}

	fn user_from_name(&self, username: &str) -> Option<User>{
		match users::table.filter(users::username.eq(username)).limit(1).load::<User>(self.db.connection()) {
			Ok(u) => u,
			Err(_) => return None,
		}.pop()
	}

	pub fn get_session_user(&self, mut cookies: Cookies ) -> Option<User> {
		if let Some(user_id) = cookies.get_private("user_id")
			.and_then(|cookie| cookie.value().parse::<i32>().ok()) {
				return self.user_from_id(user_id);
		};
		return None;
	}

	pub fn login(&mut self, cookies: Cookies, username: &str, password: &str) -> bool {
		if let Some(user) = self.user_from_name(username) {
			return match verify(password, &user.password){
				Ok(valid) => {
					if valid {
						self.create_session(cookies, &user);
						self.current_user = Some(user);
					}
					valid
				},
				Err(_) => false
			};
		}
		false
	}

	pub fn logout(&self, cookies: Cookies) {
		self.delete_session(cookies);
	}

	pub fn is_authenticated(&self) -> bool {
		self.current_user.is_some() 
	}

	pub fn get_current_user(self) -> Option<User> {
		self.current_user
	}

	pub fn create_user(&self, new_user: &NewUser) -> Result<User, diesel::result::Error> {
		diesel::insert_into(users::table)
			.values(new_user)
			.get_result(self.db.connection())
	}

	fn create_session(&self, mut cookies: Cookies, user: &User) {
		let cookie = Cookie::build("user_id", user.id.to_string())
			.path("/")
			.secure(true)
			.http_only(true)
			.finish();
		cookies.add_private(cookie);
	}



	fn delete_session(&self, mut cookies: Cookies) {
		cookies.remove_private(Cookie::named("user_id"));
	}
}