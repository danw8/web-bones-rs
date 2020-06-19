use maud::{html, Markup, PreEscaped};
use rocket::get;

#[get("/")]
pub fn index() -> Markup {
	let init = r#"
		<script type="module">
			import init from "./app.js";
			init();
		</script>
	"#;
    let html = html!{
        body{
            div id="app" {
                "Loading..."
            }
            (PreEscaped(init))        }
    };
    html
}
