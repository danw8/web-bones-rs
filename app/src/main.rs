
#[macro_use] extern crate yew;
extern crate transfer;
extern crate serde;
extern crate serde_json;
extern crate http;

mod model;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService};
use model::*;

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService,
        web: FetchService::new(),
    };
    
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
