mod components;
mod model;
mod pages;

use pages::{all_posts::AllPosts, not_found::NotFound, post_comments::PostComments};

use yew::prelude::*;
use yew_router::{prelude::*, switch::Permissive};

use crate::{
    components::{delete_post::DeletePost, header::Header, make_post::MakePost},
    pages::auth::Auth,
};

#[derive(Clone, Debug, Switch)]
enum AppRoute {
    #[to = "/!"]
    AllPosts,
    #[to = "/post/{id}"]
    PostComments(i64),
    #[to = "/404"]
    NotFound(Permissive<String>),
    #[to = "/auth"]
    Auth,
    #[to = "/create"]
    Create,
    #[to = "/edit/{id}"]
    Edit(i64),
    #[to = "/delete/{id}"]
    Delete(i64),
    #[to = "/edit_reply/{post_id}/{reply_id}"]
    EditReply(i64, i64),
    #[to = "/delete_reply/{post_id}/{reply_id}"]
    DeleteReply(i64, i64),
}

type AppRouter = Router<AppRoute>;
type AppAnchor = RouterAnchor<AppRoute>;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header/>
                <main>
                    <AppRouter
                        render=AppRouter::render(Self::switch)
                        redirect=AppRouter::redirect(|route: Route| {
                            AppRoute::NotFound(Permissive(Some(route.route)))
                        })
                   />
                </main>
            </>
        }
    }
}

impl Model {
    fn switch(switch: AppRoute) -> Html {
        match switch {
            AppRoute::AllPosts => html! { <AllPosts/> },
            AppRoute::PostComments(id) => html! { <PostComments id=id/> },
            
            AppRoute::NotFound(Permissive(route)) => html! { <NotFound route=route/> },
            AppRoute::Auth => html! { <Auth/> },

            AppRoute::Create => html! { <MakePost action="create"/> },
            AppRoute::Edit(id) => html! { <MakePost post_id=id action="edit"/> },
            AppRoute::Delete(id) => html! { <DeletePost post_id=id/> },

            AppRoute::EditReply(post_id, reply_id) => {
                html! { <MakePost post_id=post_id reply_id=reply_id action="edit_reply"/> }
            }
            AppRoute::DeleteReply(post_id, reply_id) => {
                html! { <DeletePost post_id=post_id reply_id=reply_id reply=Some(())/> }
            }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
