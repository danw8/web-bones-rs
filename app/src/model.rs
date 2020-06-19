use http::{Request, Response};
use yew::format::Json;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::FetchService;

use anyhow::Error;
use transfer::Creds;

pub struct Model {
    creds: Creds,
    login_attempts: i32,
    login_status: bool,
    //task: Option<FetchTask>,
    console: ConsoleService,
    web: FetchService,
    link: ComponentLink<Self>,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    SubmitLogin,
    LoginReady(bool),
    LoginFailed,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let creds = Creds {
            username: "".to_string(),
            password: "".to_string(),
        };
        Model {
            creds: creds,
            login_attempts: 0,
            login_status: false,
            //task: None,
            link: link,
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

                let callback = self.link.callback(|response: Response<Json<Result<bool, Error>>>| {
					if let (meta, Json(Ok(body))) = response.into_parts() {
						if meta.status.is_success() {
							return Msg::LoginReady(body);
						}
					}
					Msg::LoginFailed
				});

                let _task = self.web.fetch(login_request,callback);
            }
            Msg::LoginReady(status) => {
                let log = format!("Login Status: {}", status);
                self.console.log(&log);
                self.login_attempts += 1;
                self.login_status = status;
            }
            Msg::LoginFailed => {
                self.console.log("Improper response from log in.");
                self.login_attempts += 1;
            }
        }
        true
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool { todo!() }

    fn view(&self) -> Html {
    	let username_updated = self.link.callback(|e: InputData| Msg::UpdateUsername(e.value));
    	let password_updated = self.link.callback(|e: InputData| Msg::UpdatePassword(e.value));
    	let submit_clicked = self.link.callback(|_: MouseEvent| Msg::SubmitLogin);
        if self.login_status == true {
            html! {
                <div>
                    <h1>{"Members Area"}</h1>
                </div>
            }
        } else {
            html! {
            	<div>
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
		                    oninput=username_updated>
		                </input>
		                <label>{ "Password" }</label>
		                <input type="password",
		                    id="password",
		                    value=&self.creds.password,
		                    oninput=password_updated>
		                </input>
		                <button type="submit",
		                    id="submit",
		                    onclick=submit_clicked>
		                    {"Login"}
		                </button>
		            </div>
		            <div>
		                {"Attempts: "}{ &self.login_attempts }
		            </div>
		        </div>
            }
        }
    }
}
