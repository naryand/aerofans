mod components;
mod pages;

use pages::{all_posts::AllPosts, not_found::NotFound};

use yew::prelude::*;
use yew_router::{prelude::*, switch::Permissive};

#[derive(Clone, Debug, Switch)]
enum AppRoute {
    #[to = "/!"]
    PostList,
    #[to = "/404"]
    NotFound(Permissive<String>),
}

type AppRouter = Router<AppRoute>;

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
            AppRoute::PostList => html! { < AllPosts /> },
            AppRoute::NotFound(Permissive(route)) => html! { < NotFound route=route /> },
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
