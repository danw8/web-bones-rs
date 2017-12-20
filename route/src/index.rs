use maud::{html, Markup};
use service::*;

#[get("/")]
fn index() -> Markup {
    html!{
        body{
            div id="app" {
                "Loading..."
            }
            script src="/app.js" {

            }
        }
    }
}

#[get("/api/bob")]
fn bob() -> String {
    "I am Bob.".to_string()
}