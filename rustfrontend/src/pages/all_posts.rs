use crate::{components::post::Post, handle_req, model::PostData};

use reqwasm::http::{Request, RequestCredentials};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(AllPosts)]
pub fn all_posts() -> Html {
    let posts = use_state_eq(Vec::<PostData>::new);
    let status = use_state_eq(|| String::from("fetching posts..."));

    {
        let posts_data = posts.clone();
        let state = status.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let res = Request::get("http://127.0.0.1:8000/post/all")
                        .credentials(RequestCredentials::Include)
                        .send()
                        .await;

                    if let Some(res) = handle_req(res, &state) {
                        match res.json::<Vec<PostData>>().await {
                            Ok(o) => posts_data.set(o),
                            Err(e) => state.set(e.to_string()),
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
            {&*status}
            {
                for (*posts).iter().map(|post| html! {
                    <div>
                        <Post
                            post_id={post.id}
                            username={post.username.to_owned()}
                            text={post.text.to_owned()}
                            created_at={post.created_at}
                        />
                    </div>
                })
            }
        </>
    }
}
