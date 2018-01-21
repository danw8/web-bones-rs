
#[macro_use] extern crate yew;
extern crate transfer;
extern crate serde;
extern crate http;

use yew::html::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchHandle};
use http::{Request, Response}


use transfer::Creds;

struct Context {
    console: ConsoleService,
    web: FetchService
}

struct Model {
    email: String,
    password: String,
    login_attempts: i32,
    login_status: bool,
}

enum Msg{
    UpdateEmail(String),
    UpdatePassword(String),
    SubmitLogin,
    LoginFailed,
    LoginSuccess,
}

fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::UpdateEmail(val) => {
            model.email = val;
        },
        Msg::UpdatePassword(val) => {
            model.password = val;
        },
        Msg::SubmitLogin => {          
            let creds = Creds { 
                email: model.email,
                password: model.password,
            };
            let login_request = Request::post("/api/login")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&creds).unwrap());

            context.web.fetch(
                login_request,
                |response| {
                    if response.status().is_success() {
                        Msg::LoginSuccess
                    } else {
                        Msg::LoginFailed
                    }
                }
            )
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <h1>{"Login"}</h1>
        </div>
        <div>{ 
            if model.login_attempts > 0 && model.login_status == false { 
                "Login Failed" 
            } else {
                ""
            }
        }</div>
        <div>
            <label>{ "Email" }</label>
            <input type="text", 
                id="email",
                value=&model.email,
                oninput=|e: InputData| Msg::UpdateEmail(e.value),>
            </input>
            <label>{ "Password" }</label>
            <input type="password", 
                id="password",
                value=&model.password,
                oninput=|e: InputData| Msg::UpdatePassword(e.value),>
            </input>
            <button type="submit",
                id="submit",
                onclick=move |_| Msg::SubmitLogin,>
                {"Login"}
            </button>
        </div>
    }
}

fn main() {
    yew::initialize();
    let mut app = App::new();
    let context = Context {
        console: ConsoleService,
        web: FetchService::new(),
    };
    let model = Model {
        email: "".to_string(),
        password: "".to_string(),
        login_attempts: 0,
        login_status: false,
    };
    app.mount(context, model, update, view);
    yew::run_loop();
}
