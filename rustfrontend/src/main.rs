mod components;
mod pages;

use pages::{not_found::NotFound, post_list::PostList};

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

struct Model {
    #[allow(dead_code)]
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
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
            AppRoute::PostList => html! { < PostList /> },
            AppRoute::NotFound(Permissive(route)) => html! { < NotFound route=route /> },
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
