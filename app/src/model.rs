use http::{Request, Response};
use serde_json;
use yew::format::Json;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask};

use failure::Error;
use transfer::Creds;

pub struct Context {}

pub struct Model {
    creds: Creds,
    login_attempts: i32,
    login_status: bool,
    task: Option<FetchTask>,
    login_callback: Callback<Result<bool, Error>>,
    console: ConsoleService,
    web: FetchService,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    SubmitLogin,
    LoginReady(Result<bool, Error>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let creds = Creds {
            username: "".to_string(),
            password: "".to_string(),
        };
        Model {
            creds: creds,
            login_attempts: 0,
            login_status: false,
            task: None,
            login_callback: link.send_back(Msg::LoginReady),
            web: FetchService::new(),
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateUsername(val) => {
                self.creds.username = val;
            }
            Msg::UpdatePassword(val) => {
                self.creds.password = val;
            }
            Msg::SubmitLogin => {
                let login_request = Request::post("/api/login")
                    .header("Content-Type", "application/json")
                    .body(Json(&self.creds)) //serde_json::to_string(&self.creds).unwrap())
                    .unwrap();

                let callback = self.login_callback.clone();
                let handler = move |response: Response<Json<Result<bool, Error>>>| {
                    let (_, Json(data)) = response.into_parts();
                    callback.emit(data)
                };

                let task = self.web.fetch(login_request, handler.into());
                self.task = Some(task);
            }
            Msg::LoginReady(Ok(status)) => {
                let log = format!("Login Status: {}", status);
                self.console.log(&log);
                self.login_attempts += 1;
                self.login_status = status;
            }
            Msg::LoginReady(Err(_)) => {
                self.console.log("Improper response from log in.");
                self.login_attempts += 1;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        if self.login_status == true {
            html! {
                <div>
                    <h1>{"Members Area"}</h1>
                </div>
            }
        } else {
            html! {
                <div>
                    <h1>{ "Login" }</h1>
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
                        oninput=|e| Msg::UpdateUsername(e.value),>
                    </input>
                    <label>{ "Password" }</label>
                    <input type="password",
                        id="password",
                        value=&self.creds.password,
                        oninput=|e| Msg::UpdatePassword(e.value),>
                    </input>
                    <button type="submit",
                        id="submit",
                        onclick=|_| Msg::SubmitLogin,>
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
