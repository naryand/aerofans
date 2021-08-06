use chrono::{DateTime, Utc};
use serde::Deserialize;
use yew::{format::{Json, Nothing}, prelude::*, services::{FetchService, fetch::{FetchTask, Request, Response}}};

#[derive(Debug)]
pub enum Msg {
    GetPosts,
    ReceiveResponse(Result<Vec<Post>, anyhow::Error>),
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
    fetch_task: Option<FetchTask>,
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
                let request = Request::get("https://127.0.0.1:8443/post/all")
                    .body(Nothing).expect("get fail");
                let callback = self.link.callback(|res: Response<Json<Result<Vec<Post>, anyhow::Error>>>| {
                    let Json(data) = res.into_body();
                    Msg::ReceiveResponse(data)
                });
                let task = FetchService::fetch(request, callback).expect("task fail");
                self.fetch_task = Some(task);
                true
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(posts) => self.posts = Some(posts),
                    Err(e) => self.error = Some(e.to_string()),
                }
                self.fetch_task = None;
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
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<PostList>();
}