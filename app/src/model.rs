use serde_json;
use yew::prelude::*;
use yew::format::{ Json};
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask};
use http::{Request, Response};

use transfer::Creds;

pub struct Context {
    pub console: ConsoleService,
    pub web: FetchService,
}

pub struct Model {
    creds: Creds,
    login_attempts: i32,
    login_status: bool,
    task: Option<FetchTask>,
}

pub enum Msg{
    UpdateUsername(String),
    UpdatePassword(String),
    SubmitLogin,
    LoginReady(Result<bool, ()>),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        let creds = Creds { 
            username: "".to_string(),
            password: "".to_string(),
        };
        Model {
            creds: creds,
            login_attempts: 0,
            login_status: false,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender{
        match msg {
            Msg::UpdateUsername(val) => {
                self.creds.username = val;
            },
            Msg::UpdatePassword(val) => {
                self.creds.password = val;
            },
            Msg::SubmitLogin => {
                let login_request = Request::post("/api/login")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&self.creds).unwrap()).unwrap();
                
                let callback = context.send_back(Msg::LoginReady);
                let handler = move |response: Response<Json<Result<bool, ()>>>| {
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
                self.login_status = status;
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
        if self.login_status == true {
            html! {
                <div>
                    <h1>{"Members Area"}</h1>
                </div>
            }
        } else {
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
                        id="username",
                        value=&self.creds.username,
                        oninput=|e: InputData| Msg::UpdateUsername(e.value),>
                    </input>
                    <label>{ "Password" }</label>
                    <input type="password", 
                        id="password",
                        value=&self.creds.password,
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
}