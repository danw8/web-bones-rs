use maud::{html, Markup};

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