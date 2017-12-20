use rocket_contrib::Json;
use transfer::*;
use service::*;

#[post("/api/login", format = "application/json", data = "<creds>")]
fn login(creds: Json<Creds>) -> Json<String> {
    Json("true".to_string())
}