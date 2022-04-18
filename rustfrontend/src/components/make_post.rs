use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlTextAreaElement, RequestCredentials};
use yew::prelude::*;

use crate::model::{PostData, PostText};

pub enum Msg {
    Submit,
    ReceiveResponse(Result<PostData, String>),
    Input(String),
}

#[derive(Clone, PartialEq, Eq)]
pub enum Action {
    Create,
    Edit {post_id: i64},
    CreateReply {post_id: i64},
    EditReply {post_id: i64, reply_id: i64},
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub action: Action,
}

pub struct MakePost {
    status: Result<PostData, String>,
    text: String,
}

impl MakePost {
    fn view_status(&self) -> Html {
        match &self.status {
            Ok(_) => html! {"success"},
            Err(e) => html! { e },
        }
    }
}

impl Component for MakePost {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            status: Err(String::from("")),
            text: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Input(text) => self.text = text,

            Msg::Submit => {
                self.status = Err(String::from("posting..."));
                let cb = ctx.link().callback(Msg::ReceiveResponse);
                let post = PostText {
                    text: std::mem::take(&mut self.text),
                };

                match ctx.props().action {
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
                    Action::Edit { post_id } => spawn_local(async move {
                        let res = match Request::patch(&format!(
                            "http://127.0.0.1:8000/post/{}",
                            post_id
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
                    Action::CreateReply { post_id } => spawn_local(async move {
                        let res = match Request::post(&format!(
                            "http://127.0.0.1:8000/post/{}/reply",
                            post_id
                        ))
                        .body(serde_json::to_string(&post).unwrap())
                        .header("Content-Type", "application/json")
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await
                        {
                            Ok(res) => match res.status() {
                                200 => res,
                                _ => return cb.emit(Err(res.status_text())),
                            },
                            Err(e) => return cb.emit(Err(e.to_string())),
                        };
                        let data: Result<PostData, String> =
                            res.json().await.map_err(|x| x.to_string());
                        cb.emit(data);
                    }),
                    Action::EditReply { post_id, reply_id } => spawn_local(async move {
                        let res = match Request::patch(&format!(
                            "http://127.0.0.1:8000/post/{}/reply/{}",
                            post_id,
                            reply_id
                        ))
                        .body(serde_json::to_string(&post).unwrap())
                        .header("Content-Type", "application/json")
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await
                        {
                            Ok(res) => match res.status() {
                                200 => res,
                                _ => return cb.emit(Err(res.status_text())),
                            },
                            Err(e) => return cb.emit(Err(e.to_string())),
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <textarea placeholder="Post text" value={self.text.clone()}
                    onchange={ctx.link().batch_callback(|e: Event| {
                        let input: Option<HtmlTextAreaElement> = e.target_dyn_into::<HtmlTextAreaElement>();
                        input.map(|input| Msg::Input(input.value()))
                    })}
                />
                <br/>
                <button type="submit" onclick={ctx.link().callback(|_| Msg::Submit)}>{"Post"}</button>
                { self.view_status() }
            </>
        }
    }
}
