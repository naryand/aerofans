use chrono::NaiveDateTime;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post_id: i64,
    pub reply_id: Option<i64>,
    pub username: String,
    pub text: String,
    pub created_at: NaiveDateTime,
}

#[function_component(Post)]
pub fn post(props: &Props) -> Html {
    html! {
        <p>
            if let Some(reply_id) = props.reply_id {
                {format!("{} by {} at {} ", &props.text, &props.username, &props.created_at)}
                <Link<Route> to={Route::EditReply { post_id: props.post_id, reply_id }}>
                    {"edit"}
                </Link<Route>>
                {" "}
                <Link<Route> to={Route::DeleteReply { post_id: props.post_id, reply_id }}>
                    {"delete"}
                </Link<Route>>
            } else {
                <Link<Route> to={Route::PostComments { id: props.post_id }}>
                    {&props.text}
                </Link<Route>>
                    {format!(" by {} at {} ", &props.username, &props.created_at) }
                <Link<Route> to={Route::Edit { id: props.post_id }}>
                    {"edit" }
                </Link<Route>>
                {" "}
                <Link<Route> to={Route::Delete { id: props.post_id }}>
                    {"delete"}
                </Link<Route>>
            }
        </p>
    }
}
