use crate::{components::post::Post, model::PostData};

use reqwasm::http::{Request, RequestCredentials};
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
                        <Post
                            id=post.id
                            username=post.username.to_owned()
                            text=post.text.to_owned()
                            created_at=post.created_at
                    />
                    </div>
                })
            },
            Err(e) => html! { <p>{ e.as_str() }</p> },
        }
    }
}

impl Component for AllPosts {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetPosts);
        Self {
            link,
            posts: Err(String::from("fetching posts...")),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetPosts => {
                let cb = self.link.callback(Msg::ReceiveResponse);
                spawn_local(async move {
                    let res = match Request::get("http://127.0.0.1:8000/post/all")
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
        html! {
            <>
                { self.view_posts() }
            </>
        }
    }
}
