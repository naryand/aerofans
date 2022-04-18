use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
            <p>{"404 Not Found"}</p>
        </>
    }
}
