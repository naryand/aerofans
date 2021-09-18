use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::RequestCredentials;
use yew::prelude::*;

pub enum Msg {
    Delete,
    ReceiveResponse(Result<String, String>),
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub post_id: i64,
    pub reply_id: Option<i64>,
    pub reply: Option<()>,
}

pub struct DeletePost {
    props: Props,
    link: ComponentLink<Self>,
    status: Result<String, String>,
}

impl DeletePost {
    fn view_status(&self) -> Html {
        match &self.status {
            Ok(s) => html! { s.as_str() },
            Err(e) => html! { e.as_str() },
        }
    }
}

impl Component for DeletePost {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::Delete);
        Self {
            props,
            link,
            status: Err(String::from("deleting...")),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let cb = self.link.callback(Msg::ReceiveResponse);
        let post_id = self.props.post_id;
        let reply_id = self.props.reply_id;
        match msg {
            Msg::Delete => match self.props.reply {
                None => spawn_local(async move {
                    match Request::delete(&format!("http://127.0.0.1:8000/post/{}", post_id))
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await
                    {
                        Ok(res) => match res.status() {
                            200 => {}
                            _ => {
                                cb.emit(Err(res.status_text()));
                                return;
                            }
                        },
                        Err(e) => {
                            cb.emit(Err(e.to_string()));
                            return;
                        }
                    }
                    cb.emit(Ok(String::from("Post sucessfully deleted")));
                }),
                Some(_) => spawn_local(async move {
                    match Request::delete(&format!(
                        "http://127.0.0.1:8000/post/{}/reply/{}",
                        post_id,
                        reply_id.unwrap()
                    ))
                    .credentials(RequestCredentials::Include)
                    .send()
                    .await
                    {
                        Ok(res) => match res.status() {
                            200 => {}
                            _ => {
                                cb.emit(Err(res.status_text()));
                                return;
                            }
                        },
                        Err(e) => {
                            cb.emit(Err(e.to_string()));
                            return;
                        }
                    }
                    cb.emit(Ok(String::from("Post sucessfully deleted")));
                }),
            },
            Msg::ReceiveResponse(status) => self.status = status,
        };
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
        html! { self.view_status() }
    }
}
