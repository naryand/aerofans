use reqwasm::http::{Request, RequestCredentials};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::model::{LoginResponse, LoginUser};

pub enum Msg {
    InputUsername(String),
    InputPasword(String),
    Register,
    Login,
    ReceiveResponse(Result<LoginResponse, String>),
}

pub struct Auth {
    status: Result<LoginResponse, String>,
    username: String,
    password: String,
}

impl Auth {
    fn view_status(&self) -> Html {
        match &self.status {
            Ok(res) => html! {
                <>
                    {match res.status {
                        true => "success",
                        false => "failed",
                    }}<br/>
                    { res.message.as_str() }
                </>
            },
            Err(e) => html! { e.as_str() },
        }
    }
}

impl Component for Auth {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            status: Err(String::from("")),
            username: String::new(),
            password: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputUsername(username) => self.username = username,
            Msg::InputPasword(password) => self.password = password,
            Msg::Register => {
                self.status = Err(String::from("registering..."));
                let credentials = LoginUser {
                    username: std::mem::take(&mut self.username),
                    password: std::mem::take(&mut self.password),
                };
                let cb = ctx.link().callback(Msg::ReceiveResponse);
                spawn_local(async move {
                    let res = match Request::post("http://127.0.0.1:8000/register")
                        .body(serde_json::to_string(&credentials).unwrap())
                        .header("Content-Type", "application/json")
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await
                    {
                        Ok(res) => res,
                        Err(e) => {
                            cb.emit(Err(e.to_string()));
                            return;
                        }
                    };
                    let data: Result<LoginResponse, String> =
                        res.json().await.map_err(|x| x.to_string());
                    cb.emit(data);
                });
            }
            Msg::Login => {
                self.status = Err(String::from("logging in..."));
                let credentials = LoginUser {
                    username: std::mem::take(&mut self.username),
                    password: std::mem::take(&mut self.password),
                };
                let cb = ctx.link().callback(Msg::ReceiveResponse);
                spawn_local(async move {
                    let res = match Request::post("http://127.0.0.1:8000/login")
                        .body(serde_json::to_string(&credentials).unwrap())
                        .header("Content-Type", "application/json")
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await
                    {
                        Ok(res) => res,
                        Err(e) => {
                            cb.emit(Err(e.to_string()));
                            return;
                        }
                    };
                    let data: Result<LoginResponse, String> =
                        res.json().await.map_err(|x| x.to_string());
                    cb.emit(data);
                });
            }
            Msg::ReceiveResponse(data) => {
                self.status = data;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <form onsubmit={Callback::from(|e: FocusEvent| e.prevent_default())}>
                <input type="text" autocomplete="username" value={self.username.clone()} placeholder="Username"
                    onchange={ctx.link().batch_callback(|e: Event| {
                        let input: Option<HtmlTextAreaElement> = e.target_dyn_into::<HtmlTextAreaElement>();
                        input.map(|input| Msg::InputUsername(input.value()))
                })}/>
                <br/>
                <input type="password" autocomplete="current-password" value={self.password.clone()} placeholder="Password"
                    onchange={ctx.link().batch_callback(|e: Event| {
                        let input: Option<HtmlTextAreaElement> = e.target_dyn_into::<HtmlTextAreaElement>();
                        input.map(|input| Msg::InputPasword(input.value()))
                })}/>
                <br/>
                <br/>
                <button type="submit" onclick={ctx.link().callback(|_| Msg::Login)}>{"Login"}</button>
                <button type="submit" onclick={ctx.link().callback(|_| Msg::Register)}>{"Register"}</button>
                { self.view_status() }
            </form>
        }
    }
}
