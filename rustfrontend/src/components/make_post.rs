use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlTextAreaElement, RequestCredentials};
use yew::{prelude::*, web_sys::HtmlFormElement};

use crate::model::{PostData, PostText};

pub enum Msg {
    Submit(String),
    ReceiveResponse(Result<PostData, String>),
}

#[derive(Clone, PartialEq, Eq)]
pub enum Action {
    Create,
    Edit,
    CreateReply,
    EditREply,
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub post_id: Option<i64>,
    pub reply_id: Option<i64>,
    pub action: Action,
}

pub struct MakePost {
    props: Props,
    link: ComponentLink<Self>,
    status: Result<PostData, String>,
}

impl MakePost {
    fn view_status(&self) -> Html {
        match &self.status {
            Ok(_) => html! {},
            Err(e) => html! { e.as_str() },
        }
    }
}

impl Component for MakePost {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            status: Err(String::from("")),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(text) => {
                self.status = Err(String::from("posting..."));
                let cb = self.link.callback(Msg::ReceiveResponse);
                let post = PostText { text };
                let post_id = self.props.post_id;
                let reply_id = self.props.reply_id;
                match self.props.action {
                    Action::Create => spawn_local(async move {
                        let res = match Request::post("http://127.0.0.1:8000/post")
                            .body(serde_json::to_string(&post).unwrap())
                            .header("Content-Type", "application/json")
                            .credentials(RequestCredentials::Include)
                            .send()
                            .await
                        {
                            Ok(res) => match res.status() {
                                200 => res,
                                _ => {
                                    cb.emit(Err(res.status_text()));
                                    return;
                                }
                            },
                            Err(e) => {
                                cb.emit(Err(e.to_string()));
                                return;
                            }
                        };
                        let data: Result<PostData, String> =
                            res.json().await.map_err(|x| x.to_string());
                        cb.emit(data);
                    }),
                    Action::Edit => spawn_local(async move {
                        let res = match Request::patch(&format!(
                            "http://127.0.0.1:8000/post/{}",
                            post_id.unwrap()
                        ))
                        .body(serde_json::to_string(&post).unwrap())
                        .header("Content-Type", "application/json")
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await
                        {
                            Ok(res) => match res.status() {
                                200 => res,
                                _ => {
                                    cb.emit(Err(res.status_text()));
                                    return;
                                }
                            },
                            Err(e) => {
                                cb.emit(Err(e.to_string()));
                                return;
                            }
                        };
                        let data: Result<PostData, String> =
                            res.json().await.map_err(|x| x.to_string());
                        cb.emit(data);
                    }),
                    Action::CreateReply => spawn_local(async move {
                        let res = match Request::post(&format!(
                            "http://127.0.0.1:8000/post/{}/reply",
                            post_id.unwrap()
                        ))
                        .body(serde_json::to_string(&post).unwrap())
                        .header("Content-Type", "application/json")
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await
                        {
                            Ok(res) => match res.status() {
                                200 => res,
                                _ => {
                                    cb.emit(Err(res.status_text()));
                                    return;
                                }
                            },
                            Err(e) => {
                                cb.emit(Err(e.to_string()));
                                return;
                            }
                        };
                        let data: Result<PostData, String> =
                            res.json().await.map_err(|x| x.to_string());
                        cb.emit(data);
                    }),
                    Action::EditREply => spawn_local(async move {
                        let res = match Request::patch(&format!(
                            "http://127.0.0.1:8000/post/{}/reply/{}",
                            post_id.unwrap(),
                            reply_id.unwrap(),
                        ))
                        .body(serde_json::to_string(&post).unwrap())
                        .header("Content-Type", "application/json")
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await
                        {
                            Ok(res) => match res.status() {
                                200 => res,
                                _ => {
                                    cb.emit(Err(res.status_text()));
                                    return;
                                }
                            },
                            Err(e) => {
                                cb.emit(Err(e.to_string()));
                                return;
                            }
                        };
                        let data: Result<PostData, String> =
                            res.json().await.map_err(|x| x.to_string());
                        cb.emit(data);
                    }),
                }
            }
            Msg::ReceiveResponse(data) => {
                self.status = data;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        match self.props == props {
            true => {
                self.props = props;
                true
            }
            false => false,
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <form onsubmit=self.link.callback(|e: FocusEvent| {
                    let e = Event::from(e);
                    e.prevent_default();

                    let collection = e.target().unwrap().dyn_into::<HtmlFormElement>().unwrap().elements();
                    let mut text = String::new();

                    for i in 0..collection.length() {
                        match collection.item(i) {
                            Some(e) => match e.id().as_str() {
                                "text" => {
                                    text = e.dyn_into::<HtmlTextAreaElement>().unwrap().value();
                                }
                                _ => {}
                            }
                            None => {}
                        }
                    }

                    Msg::Submit(text)
                })>
                <textarea id="text" placeholder="Post text"/><br/>
                <input type="submit" value="Post"/>
                </form>
                { self.view_status() }
            </>
        }
    }
}
