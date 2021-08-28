use crate::{components::post::Post, model::PostData};

use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Debug)]
pub enum Msg {
    GetPosts,
    ReceiveResponse(Result<Vec<PostData>, String>),
}

pub struct AllPosts {
    link: ComponentLink<Self>,
    posts: Result<Vec<PostData>, String>,
}

impl AllPosts {
    fn view_posts(&self) -> Html {
        match &self.posts {
            Ok(p) => html! {
                for p.iter().map(|post| html! {
                    <div>
                        < Post
                            id=post.id
                            username=post.username.clone()
                            text=post.text.clone()
                            created_at=post.created_at
                        />
                    </div>
                })
            },
            Err(e) => html! { <p>{ e.clone() }</p> },
        }
    }
}

impl Component for AllPosts {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetPosts);
        AllPosts {
            link,
            posts: Err(format!("fetching posts")),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetPosts => {
                let cb = self.link.callback(Msg::ReceiveResponse);
                spawn_local(async move {
                    let res = Request::get("http://127.0.0.1:8000/post/all")
                        .send()
                        .await
                        .unwrap();
                    let data: Result<Vec<PostData>, String> =
                        res.json().await.map_err(|x| x.to_string());
                    cb.emit(data);
                });
                true
            }
            Msg::ReceiveResponse(data) => {
                self.posts = data;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { self.view_posts() }
    }
}
