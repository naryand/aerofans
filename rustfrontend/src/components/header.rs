use yew::prelude::*;

use crate::{AppAnchor, AppRoute};

pub struct Header {}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <AppAnchor route=AppRoute::AllPosts>
                { String::from("home") }
            </AppAnchor>
            <br/>
            <AppAnchor route=AppRoute::Auth>
                { String::from("login/register") }
            </AppAnchor>
            <br/>
            <AppAnchor route=AppRoute::Create>
                { String::from("create post") }
            </AppAnchor>
            </>
        }
    }
}
