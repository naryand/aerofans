use js_sys::Reflect;
use reqwasm::http::{Request, RequestCredentials};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{prelude::*, web_sys::HtmlFormElement};

use crate::model::{LoginResponse, LoginUser};

pub enum Msg {
    Register(String, String),
    Login(String, String),
    ReceiveResponse(Result<LoginResponse, String>),
}

pub struct Auth {
    link: ComponentLink<Self>,
    status: Result<LoginResponse, String>,
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

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Auth {
            link,
            status: Err(format!("")),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Register(username, password) => {
                self.status = Err(format!("registering..."));
                let credentials = LoginUser { username, password };
                let cb = self.link.callback(Msg::ReceiveResponse);
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
            Msg::Login(username, password) => {
                self.status = Err(format!("logging in..."));
                let credentials = LoginUser { username, password };
                let cb = self.link.callback(Msg::ReceiveResponse);
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <form onsubmit=self.link.callback(|e: FocusEvent| {
                    let e = Event::from(e);
                    e.prevent_default();

                    let mut user = String::new();
                    let mut pass = String::new();

                    let collection = e.target().unwrap().dyn_into::<HtmlFormElement>().unwrap().elements();

                    for i in 0..collection.length() {
                        match collection.item(i) {
                            Some(e) => match e.id().as_str() {
                                "user" => {
                                    user = e.dyn_into::<HtmlInputElement>().unwrap().value();
                                }
                                "pass" => {
                                    pass = e.dyn_into::<HtmlInputElement>().unwrap().value();

                                }
                                _ => {}
                            }
                            None => {}
                        }
                    }

                    let submitter_element = Reflect::get(&e, &JsValue::from_str("submitter")).unwrap();
                    let submitter_value = submitter_element.dyn_into::<HtmlInputElement>().unwrap().value();

                    match submitter_value.as_str() {
                        "Login" => {
                            Msg::Login(user, pass)
                        }
                        "Register" => {
                            Msg::Register(user, pass)
                        }
                        _ => unimplemented!()
                    }
                })>
                <input id="user" type="text" placeholder="Username"/><br/>
                <input id="pass" type="password" placeholder="Password"/><br/><br/>
                <input type="submit" value="Login"/>
                <input type="submit" value="Register"/>
                </form>
                { self.view_status() }
            </>
        }
    }
}
