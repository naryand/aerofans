mod components;
mod model;
mod pages;

use crate::components::{
    delete_post::DeletePost,
    header::Header,
    make_post::{Action, MakePost},
};
use crate::pages::{
    all_posts::AllPosts, auth::Auth, not_found::NotFound, post_comments::PostComments,
};

use reqwasm::{http::Response, Error};
use wasm_logger;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    AllPosts,
    #[at("/post/:id")]
    PostComments { id: i64 },
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/auth")]
    Auth,
    #[at("/create")]
    Create,
    #[at("/edit/:id")]
    Edit { id: i64 },
    #[at("/delete/:id")]
    Delete { id: i64 },
    #[at("/edit_reply/:post_id/:reply_id")]
    EditReply { post_id: i64, reply_id: i64 },
    #[at("/delete_reply/:post_id/:reply_id")]
    DeleteReply { post_id: i64, reply_id: i64 },
}

fn switch(switch: &Route) -> Html {
    match switch {
        Route::AllPosts => html! { <AllPosts/> },
        Route::PostComments { id } => html! { <PostComments id={*id}/> },

        Route::NotFound => html! { <NotFound/> },
        Route::Auth => html! { <Auth/> },

        Route::Create => html! { <MakePost action={Action::Create}/> },
        Route::Edit { id } => html! { <MakePost action={Action::Edit {post_id: *id}}/> },
        Route::Delete { id } => html! { <DeletePost post_id={*id}/> },

        Route::EditReply { post_id, reply_id } => {
            html! { <MakePost action={Action::EditReply { post_id: *post_id, reply_id: *reply_id }}/> }
        }
        Route::DeleteReply { post_id, reply_id } => {
            html! { <DeletePost post_id={*post_id} reply_id={*reply_id}/> }
        }
    }
}

fn handle_req(r: Result<Response, Error>, state: &UseStateHandle<String>) -> Option<Response> {
    match r {
        Ok(res) => match res.status() {
            200 => {
                state.set(String::new());
                Some(res)
            }
            _ => {
                state.set(res.status_text());
                None
            }
        },
        Err(e) => {
            state.set(e.to_string());
            None
        }
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <>
            <main>
                <BrowserRouter>
                    <Header/>
                    <br/>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </main>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Main>();
}
