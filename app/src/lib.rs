#![recursion_limit="256"]

use wasm_bindgen::prelude::*;
mod model;

use model::*;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::FetchService;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
