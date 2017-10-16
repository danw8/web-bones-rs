use maud::{html, Markup};
use service::*;

#[get("/")]
fn index() -> Markup {
    html!{
        body{
            "Loading..."
        }
    }
}