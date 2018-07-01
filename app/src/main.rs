#[macro_use]
extern crate yew;
extern crate failure;
extern crate http;
extern crate serde;
extern crate serde_json;
extern crate transfer;

mod model;

use model::*;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::FetchService;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
