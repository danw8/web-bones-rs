use rocket_contrib::json::Json;
use rocket::http::Cookies;
use transfer::*;
use service::*;
use rocket::post;

#[post("/api/login", format = "application/json", data = "<creds>")]
pub fn login(cookies: Cookies, mut user_service: UserService, creds: Json<Creds>) -> Json<bool> {
    let logged_in = user_service.login(cookies, &creds.username, &creds.password);
    Json(logged_in)
}