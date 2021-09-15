use crate::{AppAnchor, AppRoute};

use chrono::{DateTime, Utc};
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub id: i64,
    pub username: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

pub struct Post {
    props: Props,
}

impl Component for Post {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        match self.props == props {
            true => {
                self.props = props;
                true
            }
            false => false,
        }
    }

    fn view(&self) -> Html {
        html! {
            <p>
                <AppAnchor route=AppRoute::PostComments(self.props.id)>
                    { format!("{}", self.props.text) }
                </AppAnchor>
                { format!(" by {} at {} ", self.props.username, self.props.created_at) }
                <AppAnchor route=AppRoute::Edit(self.props.id)>
                    { String::from("edit") }
                </AppAnchor>
                { String::from(" ") }
                <AppAnchor route=AppRoute::Delete(self.props.id)>
                    { String::from("delete") }
                </AppAnchor>
            </p>
        }
    }
}
