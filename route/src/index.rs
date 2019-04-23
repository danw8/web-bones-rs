use maud::{html, Markup};
use rocket::get;

#[get("/")]
pub fn index() -> Markup {
    let html = html!{
        body{
            div id="app" {
                "Loading..."
            }
            script src="/app.js" {

            }
        }
    };
    html
}