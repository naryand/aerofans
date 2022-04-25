use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlTextAreaElement, RequestCredentials};
use yew::prelude::*;

use crate::{
    handle_req,
    model::{PostData, PostText},
};

#[derive(Clone, PartialEq, Eq)]
pub enum Action {
    Create,
    Edit { post_id: i64 },
    CreateReply { post_id: i64 },
    EditReply { post_id: i64, reply_id: i64 },
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub action: Action,
}

#[function_component(MakePost)]
pub fn make_post(props: &Props) -> Html {
    let status = use_state_eq(String::new);
    let text = use_state_eq(String::new);

    let onchange = {
        let input_text = text.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlTextAreaElement>();
            if let Some(input) = input {
                input_text.set(input.value())
            }
        })
    };

    let onclick = {
        async fn opts(r: Request, post: PostText, status: UseStateHandle<String>) {
            let res = r
                .body(serde_json::to_string(&post).unwrap())
                .header("Content-Type", "application/json")
                .credentials(RequestCredentials::Include)
                .send()
                .await;

            if let Some(res) = handle_req(res, &status) {
                match res.json::<PostData>().await {
                    Ok(_) => status.set(String::from("success")),
                    Err(e) => status.set(e.to_string()),
                }
            }
        }

        let action = props.action.clone();
        let click_status = status.clone();
        Callback::from(move |_: MouseEvent| {
            let submit_text = text.clone();
            let click_status = click_status.clone();

            click_status.set(String::from("posting..."));

            let post = PostText {
                text: (*submit_text).to_owned(),
            };
            submit_text.set(String::new());

            match action {
                Action::Create => spawn_local(opts(
                    Request::post("http://127.0.0.1:8000/post"),
                    post,
                    click_status,
                )),
                Action::Edit { post_id } => spawn_local(opts(
                    Request::patch(&format!("http://127.0.0.1:8000/post/{}", post_id)),
                    post,
                    click_status,
                )),
                Action::CreateReply { post_id } => spawn_local(opts(
                    Request::post(&format!("http://127.0.0.1:8000/post/{}/reply", post_id)),
                    post,
                    click_status,
                )),
                Action::EditReply { post_id, reply_id } => spawn_local(opts(
                    Request::patch(&format!(
                        "http://127.0.0.1:8000/post/{}/reply/{}",
                        post_id, reply_id
                    )),
                    post,
                    click_status,
                )),
            }
        })
    };

    html! {
        <>
            <textarea placeholder="Post text" {onchange}>{ &*text }</textarea>
            <br/>
            <button type="submit" {onclick}>{"Post"}</button>
            {&*status}
        </>
    }
}
