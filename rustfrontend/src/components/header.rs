use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
        <Link<Route> to={Route::AllPosts}>
            { "home" }
        </Link<Route>>
        <br/>
        <Link<Route> to={Route::Auth}>
            { "login/register" }
        </Link<Route>>
        <br/>
        <Link<Route> to={Route::Create}>
            { "create post" }
        </Link<Route>>
        </>
    }
}
