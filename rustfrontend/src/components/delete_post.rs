use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::RequestCredentials;
use yew::prelude::*;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub post_id: i64,
    pub reply_id: Option<i64>,
}

async fn delete(url: &str, state: UseStateHandle<String>) {
    match Request::delete(url)
        .credentials(RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => match res.status() {
            200 => state.set(String::from("Sucessfully deleted")),
            _ => state.set(res.status_text()),
        },
        Err(e) => state.set(e.to_string()),
    }
}

#[function_component(DeletePost)]
pub fn delete_post(props: &Props) -> Html {
    let status = use_state_eq(|| String::from("deleting..."));

    {
        let post_id = props.post_id;
        let reply_id = props.reply_id;
        let state = status.clone();

        use_effect_with_deps(
            move |_| {
                match reply_id {
                    None => spawn_local(async move {
                        delete(&format!("http://127.0.0.1:8000/post/{}", post_id), state).await
                    }),

                    Some(reply_id) => spawn_local(async move {
                        delete(
                            &format!("http://127.0.0.1:8000/post/{}/reply/{}", post_id, reply_id),
                            state,
                        )
                        .await
                    }),
                }
                || {}
            },
            (),
        );
    }

    html! { &*status }
}
