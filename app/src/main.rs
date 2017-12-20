
#![feature(proc_macro)]
#[macro_use] extern crate stdweb;
extern crate maud;
extern crate fore;

use maud::html;
use stdweb::web::event::*;
use stdweb::web::*;
use stdweb::Value;
use stdweb::unstable::TryInto;
use std::clone::Clone;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::VecDeque;

use fore::*;
use fore::stdwebex::*;

struct State{
    number: i32,
    input: String,
    input_focus: bool,
}

#[derive(Clone)]
enum Msg{
    Increment,
    Decrement,
    Key,
    InputChanged,
    GetBob,
    SendMessage,
}

type TestApp<'a> = ForeApp<'a, State>;

fn main() {
    stdweb::initialize();

    let state = State{
        number: 0,
        input: "".to_string(),
        input_focus: false,
    };
    let state = Arc::new(Mutex::new(state));
    let app = TestApp::new(state, "body");
    let component = TestComponent{};
    let mut queue = VecDeque::new();
    app.start(component, &update, &mut queue);

    stdweb::event_loop();
}

struct TestComponent{}
impl ForeComponent<State, Msg> for TestComponent{
    fn view(&self, state: Arc<Mutex<State>>, msg_queue: &mut VecDeque<ForeEvent<Msg>>) -> String {
        let button_up_id = "button-up";
        let button_down_id = "button-down";
        let input_id = "input-id";
        let get_bob_id = "get-bob";
        let state = state.lock().unwrap();
        let message = html!{
            h1 {
                (state.number)
            }
            button type="button" id=(button_down_id) { 
                (click::<Msg>(&format!("#{}", button_down_id), Msg::Decrement, msg_queue))
                "-"
            }
            button type="button" id=(button_up_id) {
                (click::<Msg>(&format!("#{}", button_up_id), Msg::Increment, msg_queue)) 
                "+"
            }
            div {
                @if state.input_focus {
                    input type="test" id=(input_id) value=(state.input) autofocus {
                        (on_keypress::<Msg>(&format!("#{}", input_id), Msg::Key, msg_queue))
                        (on_change::<Msg>(&format!("#{}", input_id), Msg::InputChanged, msg_queue))
                    }
                } @else {
                    input type="test" id=(input_id) value=(state.input) {
                        (on_keypress::<Msg>(&format!("#{}", input_id), Msg::Key, msg_queue))
                        (on_change::<Msg>(&format!("#{}", input_id), Msg::InputChanged, msg_queue))
                    }
                }
            }
            div {
                button type="button" id=(get_bob_id) {
                    (click::<Msg>(&format!("#{}", get_bob_id), Msg::GetBob, msg_queue))
                    "Get Bob!"
                }
            }
        };
        message.into_string()
    }
}

fn update(msg: Arc<Mutex<Msg>>, state: Arc<Mutex<State>>, event_data: EventData, node: &Node ) {
    let msg = &*msg.lock().unwrap();
    match msg {
        &Msg::Increment => {
            let state = state.clone();
            let mut state = &mut *state.lock().unwrap();
            state.number += 1;
        },
        &Msg::Decrement => {
            let state = state.clone();
            let mut state = &mut *state.lock().unwrap();
            state.number -= 1;
        },
        &Msg::Key => {
            return;
        },
        &Msg::InputChanged => {
            let state = state.clone();
            let mut state = &mut *state.lock().unwrap();
            match event_data {
                EventData::Change(_) => {
                    state.input = get_value(node.clone());
                },
                _ => {}
            }
            return;
        },
        &Msg::GetBob => {
            let bob = http_get("api/bob");
            alert(&bob);
        },
        &Msg::SendMessage => {

        },
        _ => {}
    }

    // fore doesn't do any diffing yet so we just restart the app with the state.
    // this can cause issues with things not updating...
    let app = TestApp::new(state, "body");
    let component = TestComponent{};
    let mut queue = VecDeque::new();
    app.start(component, &update, &mut queue);
}

/// This is not an async request
fn http_get(url: &str) -> String {
    let result : Value = js! {
        var xmlHttp = new XMLHttpRequest();
        xmlHttp.open("GET", @{url}, false);
        xmlHttp.send( null );
        var response = xmlHttp.responseText;
        return response;
    };

    return result.try_into().unwrap();
}

/// This is not an async request
fn http_post(url: &str, data: &str) -> String {
    let result = js!{
        var params = @{data};
        var xmlHttp = new XMLHttpRequest();
        xmlHttp.open("POST", @{url}, false);
        xmlHttp.setRequestHeader("Content-Type", "application/json;charset-UTF-8");
        xmlHttp.send(params);
        var response = xmlHttp.responseText;
        return response;
    };

    return result.try_into().unwrap();
}
