use rocket_contrib::Json;
use rocket::http::Cookies;
use transfer::*;
use service::*;

#[post("/api/login", format = "application/json", data = "<creds>")]
fn login(cookies: Cookies, mut user_service: UserService, creds: Json<Creds>) -> Json<bool> {
    let logged_in = user_service.login(cookies, &creds.username, &creds.password);
    Json(logged_in)
}