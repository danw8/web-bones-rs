
#![feature(proc_macro)]
#[macro_use] extern crate stdweb;
extern crate maud;
extern crate fore;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate transfer;

use maud::html;
use maud::PreEscaped;
use stdweb::web::event::*;
use stdweb::web::*;
use stdweb::Value;
use stdweb::unstable::TryInto;
use std::clone::Clone;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::VecDeque;
use serde::Serialize;

use fore::*;
use fore::stdwebex::*;

struct State {
    email: String,
    password: String,
    login_attempts: i32,
    login_status: bool,
}

#[derive(Clone)]
enum Msg{
    EmailChanged,
    PasswordChanged,
    SubmitLogin,
}

type TestApp<'a> = ForeApp<'a, State>;

fn main() {
    stdweb::initialize();

    let state = State{
        email: "".to_string(),
        password: "".to_string(),
        login_attempts: 0,
        login_status: false,
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

        let state = state.lock().unwrap();
        let message = match state.login_status {
            true => {
                view_logged_in(&state)
            },
            false => {
                view_login(&state, msg_queue)
            }
        };
        message.into_string()
    }
}

fn update(msg: Arc<Mutex<Msg>>, state: Arc<Mutex<State>>, event_data: EventData, node: &Node ) {
    let msg = &*msg.lock().unwrap();
    match msg {
        &Msg::EmailChanged => {
            let state = state.clone();
            let mut state = &mut *state.lock().unwrap();
            match event_data {
                EventData::Change(_) => {
                    state.email = get_value(node.clone());
                },
                _ => {}
            }
            return;
        },
        &Msg::PasswordChanged => {
            let state = state.clone();
            let mut state = &mut *state.lock().unwrap();
            match event_data {
                EventData::Change(_) => {
                    state.password = get_value(node.clone());
                },
                _ => {}
            }
            return;
        },
        &Msg::SubmitLogin => {
            let state = state.clone();
            let mut state = &mut *state.lock().unwrap();
            let creds = transfer::Creds {
                email: state.email.clone(),
                password: state.password.clone(),
            };
            let result = http_post("api/login", &creds);
            // The returned string has qoutes... go figure.
            if result == "\"true\"" {
                state.login_status = true;
            } else {
                state.login_attempts += 1;
            }
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
fn http_post<I: Serialize>(url: &str, item: &I) -> String {
    let serialized = serde_json::to_string(item).unwrap();

    let result : Value = js!{
        var params = @{serialized};
        var xmlHttp = new XMLHttpRequest();
        xmlHttp.open("POST", @{url}, false);
        xmlHttp.setRequestHeader("Content-Type", "application/json");
        xmlHttp.send(params);
        var response = xmlHttp.responseText;
        console.log(response);
        return response;
    };
    return result.try_into().unwrap();
}

fn view_login(state: &State, msg_queue: &mut VecDeque<ForeEvent<Msg>>) -> PreEscaped<String> {
    let email_id = "email";
    let password_id = "password";
    let submit_id = "submit";
    html!{
        h1 {
            "Login"
        }
        @if state.login_attempts > 0 && state.login_status == false {
            "Login Failed"
        }
        div {
            label for=(email_id) { 
                "Email"
            }
            input type="text" id=(email_id) value=(state.email) {
                (on_change::<Msg>(&format!("#{}", email_id), Msg::EmailChanged, msg_queue))
            }
            label for=(password_id) { 
                "Password"
            }
            input type="password" id=(password_id) value=(state.email) {
                (on_change::<Msg>(&format!("#{}", password_id), Msg::PasswordChanged, msg_queue))
            }
            button type="submit" id=(submit_id) { 
                (click::<Msg>(&format!("#{}", submit_id), Msg::SubmitLogin, msg_queue))
                "submit"
            }
        }
    }
}

fn view_logged_in(state: &State) -> PreEscaped<String> {
    html!{
        h1 {
            "You have been logged in."
        }
    }
}