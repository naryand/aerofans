use reqwasm::http::{Request, RequestCredentials};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    handle_req,
    model::{LoginResponse, LoginUser},
};

#[function_component(Auth)]
pub fn auth() -> Html {
    let status = use_state_eq(String::new);
    let username = use_state_eq(String::new);
    let password = use_state_eq(String::new);

    let onchange = |text: UseStateHandle<String>| {
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                text.set(input.value())
            }
        })
    };

    let onclick = move |url,
                        username: UseStateHandle<String>,
                        password: UseStateHandle<String>,
                        status: UseStateHandle<String>| {
        Callback::from(move |_: MouseEvent| {
            let username = username.clone();
            let password = password.clone();
            let status = status.clone();

            status.set(String::from("..."));
            let credentials = LoginUser {
                username: (*username).to_owned(),
                password: (*password).to_owned(),
            };
            username.set(String::new());
            password.set(String::new());

            spawn_local(async move {
                let res = Request::post(url)
                    .body(serde_json::to_string(&credentials).unwrap())
                    .header("Content-Type", "application/json")
                    .credentials(RequestCredentials::Include)
                    .send()
                    .await;

                if let Some(res) = handle_req(res, &status) {
                    match res.json::<LoginResponse>().await {
                        Ok(o) => status.set(o.message),
                        Err(e) => status.set(e.to_string()),
                    }
                }
            });
        })
    };

    html! {
        <form onsubmit={Callback::from(|e: FocusEvent| e.prevent_default())}>
            <input type="text" autocomplete="username" placeholder="Username" value={ (*username).clone() }
                onchange={onchange(username.clone())}/>
            <br/>
            <input type="password" autocomplete="current-password" placeholder="Password" value={(*password).to_owned()}
                onchange={onchange(password.clone())}/>

            <br/>
            <br/>

            <button type="submit"
                onclick={onclick("http://127.0.0.1:8000/login", username.clone(), password.clone(), status.clone())}
            >{"Login"}</button>
            <button type="submit"
                onclick={onclick("http://127.0.0.1:8000/register", username, password, status.clone())}
            >{"Register"}</button>
            {&*status}
        </form>
    }
}
