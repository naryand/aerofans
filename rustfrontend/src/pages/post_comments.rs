use crate::{
    components::{
        make_post::{Action, MakePost},
        post::Post,
    },
    handle_req,
    model::{CommentData, PostData},
};

use reqwasm::http::{Request, RequestCredentials};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub id: i64,
}

#[function_component(PostComments)]
pub fn post_comments(props: &Props) -> Html {
    let post = use_state_eq(|| None);
    let comments = use_state_eq(Vec::<CommentData>::new);

    let post_status = use_state_eq(|| String::from("fetching posts..."));
    let comment_status = use_state_eq(|| String::from("fetching comments..."));

    let id = props.id;

    {
        let post_data = post.clone();
        let post_state = post_status.clone();

        let comment_data = comments.clone();
        let comment_state = comment_status.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let res = Request::get(&format!("http://127.0.0.1:8000/post/{}", id))
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await;

                    if let Some(res) = handle_req(res, &post_state) {
                        match res.json::<PostData>().await {
                            Ok(o) => post_data.set(Some(o)),
                            Err(e) => post_state.set(e.to_string()),
                        }
                    }
                });
                spawn_local(async move {
                    let res = Request::get(&format!("http://127.0.0.1:8000/post/{}/reply/all", id))
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await;

                    if let Some(res) = handle_req(res, &comment_state) {
                        match res.json::<Vec<CommentData>>().await {
                            Ok(o) => comment_data.set(o),
                            Err(e) => comment_state.set(e.to_string()),
                        }
                    }
                });
                || {}
            },
            (),
        );
    }

    html! {
        <>
            {&*post_status}
            <div>
                if let Some(post) = &*post {
                    <Post
                    post_id={post.id}
                    username={post.username.to_owned()}
                    text={post.text.to_owned()}
                    created_at={post.created_at}
                    />
                }
            </div>
            <hr/>
            <MakePost action={Action::CreateReply { post_id: id }}/>
            {&*comment_status}
            {
                for (*comments).iter().map(|c| html! {
                    <div>
                        <Post
                            post_id={c.post_id}
                            reply_id={c.id}
                            username={c.username.to_owned()}
                            text={c.text.to_owned()}
                            created_at={c.created_at}
                        />
                    </div>
                })
            }
        </>
    }
}
