
#[macro_use] extern crate yew;
extern crate transfer;
extern crate serde;
extern crate serde_json;
extern crate http;

use yew::prelude::*;
use yew::format::{ Json};
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask};
use http::{Request, Response};


use transfer::Creds;

struct Context {
    console: ConsoleService,
    web: FetchService,
}

struct Model {
    email: String,
    password: String,
    login_attempts: i32,
    login_status: bool,
    task: Option<FetchTask>,
}

enum Msg{
    UpdateEmail(String),
    UpdatePassword(String),
    SubmitLogin,
    LoginReady(Result<String, ()>),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            email: "".to_string(),
            password: "".to_string(),
            login_attempts: 0,
            login_status: false,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender{
        match msg {
            Msg::UpdateEmail(val) => {
                self.email = val;
            },
            Msg::UpdatePassword(val) => {
                self.password = val;
            },
            Msg::SubmitLogin => {          
                let creds = Creds { 
                    email: self.email.to_string(),
                    password: self.password.to_string(),
                };
                let login_request = Request::post("/api/login")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&creds).unwrap()).unwrap();
                
                let callback = context.send_back(Msg::LoginReady);
                let handler = move |response: Response<Json<Result<String, ()>>>| {
                    let (_, Json(data)) = response.into_parts();
                    callback.emit(data)
                };

                let task = context.web.fetch(
                    login_request,
                    handler.into()
                );

                self.task = Some(task);
            },
            Msg::LoginReady(Ok(status)) => {
                let log = format!("Login Status: {}", status);
                context.console.log(&log);
                self.login_attempts += 1;
                self.login_status = status == "true".to_string();
            },
            Msg::LoginReady(Err(_)) => {
                context.console.log("Improper response from log in.");
                self.login_attempts += 1;
            }
        }
        true
    }
}


impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <h1>{"Login"}</h1>
            </div>
            <div>{ 
                if self.login_attempts > 0 && self.login_status == false { 
                    "Login Failed" 
                } else {
                    ""
                }
            }</div>
            <div>
                <label>{ "Email" }</label>
                <input type="text", 
                    id="email",
                    value=&self.email,
                    oninput=|e: InputData| Msg::UpdateEmail(e.value),>
                </input>
                <label>{ "Password" }</label>
                <input type="password", 
                    id="password",
                    value=&self.password,
                    oninput=|e: InputData| Msg::UpdatePassword(e.value),>
                </input>
                <button type="submit",
                    id="submit",
                    onclick=move |_| Msg::SubmitLogin,>
                    {"Login"}
                </button>
            </div>
            <div>
                {"Attempts: "}{ &self.login_attempts }
            </div>
        }
    }
}

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
