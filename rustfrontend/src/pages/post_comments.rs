use crate::{
    components::{comment::Comment, post::Post},
    model::{CommentData, PostData},
};

use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub enum Msg {
    GetPost,
    GetComments,
    ReceivePost(Result<PostData, String>),
    ReceiveComments(Result<Vec<CommentData>, String>),
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub id: i64,
}

pub struct PostComments {
    props: Props,
    link: ComponentLink<Self>,
    post: Result<PostData, String>,
    comments: Result<Vec<CommentData>, String>,
}

impl PostComments {
    fn view_posts(&self) -> Html {
        match &self.post {
            Ok(p) => html! {
                <div>
                    < Post id=p.id username=p.username.clone() text=p.text.clone() created_at=p.created_at />
                </div>
            },
            Err(e) => html! { <p>{ e.clone() }</p> },
        }
    }

    fn view_comments(&self) -> Html {
        match &self.comments {
            Ok(c) => html! {
                for c.iter().map(|comm| html! {
                    <div>
                        < Comment username=comm.username.clone() text=comm.text.clone() created_at=comm.created_at />
                    </div>
                })
            },
            Err(e) => html! { <p> { e.clone() }</p> },
        }
    }
}

impl Component for PostComments {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetPost);
        link.send_message(Msg::GetComments);
        PostComments {
            props,
            link,
            post: Err(format!("fetching posts")),
            comments: Err(format!("fetching comments")),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetPost => {
                let cb = self.link.callback(Msg::ReceivePost);
                let id = self.props.id;
                spawn_local(async move {
                    let res = Request::get(&format!("http://127.0.0.1:8000/post/{}", id))
                        .send()
                        .await
                        .unwrap();
                    let data: Result<PostData, String> =
                        res.json().await.map_err(|x| x.to_string());
                    cb.emit(data);
                });
                true
            }
            Msg::GetComments => {
                let cb = self.link.callback(Msg::ReceiveComments);
                let id = self.props.id;
                spawn_local(async move {
                    let res = Request::get(&format!("http://127.0.0.1:8000/post/{}/reply/all", id))
                        .send()
                        .await
                        .unwrap();
                    let data: Result<Vec<CommentData>, String> =
                        res.json().await.map_err(|x| x.to_string());
                    cb.emit(data);
                });
                true
            }
            Msg::ReceivePost(data) => {
                self.post = data;
                true
            }
            Msg::ReceiveComments(data) => {
                self.comments = data;
                true
            }
        }
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
                { self.view_posts() }
                <hr/>
                { self.view_comments() }
            </>
        }
    }
}
