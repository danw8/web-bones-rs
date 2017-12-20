#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct Creds {
    pub email: String,
    pub password: String,
}