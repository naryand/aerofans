use chrono::{DateTime, Utc};
use reqwasm::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Debug)]
pub enum Msg {
    GetPosts,
    ReceiveResponse(Option<Vec<Post>>),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    pub id: i64,
    pub author: i64,
    pub username: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
struct PostList {
    link: ComponentLink<Self>,
    fetch_task: Option<()>,
    posts: Option<Vec<Post>>,
    error: Option<String>,
}

impl PostList {
    fn view_posts(&self) -> Html {
        match &self.posts {
            Some(p) => html! {
                <div>
                    { for p.iter().map(|post| html! { <p>{ format!("{} by {} at {}", post.text, post.username, post.created_at) }</p> }) }
                </div>
            },
            None => html! {},
        }
    }

    fn view_fetch(&self) -> Html {
        match &self.fetch_task {
            Some(_) => html! { <p>{ "fetching posts" }</p> },
            None => html! {},
        }
    }

    fn view_error(&self) -> Html {
        match &self.error {
            Some(e) => html! {
                <p>{ e.clone() }</p>
            },
            None => html! {},
        }
    }
}

impl Component for PostList {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetPosts);
        PostList {
            link,
            fetch_task: None,
            posts: None,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetPosts => {
                self.fetch_task = Some(());
                let cb = self.link.callback(Msg::ReceiveResponse);
                spawn_local(async move {
                    let res = Request::get("http://127.0.0.1:8000/post/all")
                        .send()
                        .await
                        .unwrap();
                    let data: Option<Vec<Post>> = res.json().await.ok();
                    cb.emit(data);
                });
                true
            }
            Msg::ReceiveResponse(data) => {
                self.fetch_task = None;
                self.posts = data;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_posts() }
                { self.view_fetch() }
                { self.view_error() }
            </>
        }
    }
}

fn main() {
    yew::start_app::<PostList>();
}
