use chrono::NaiveDateTime;
use yew::prelude::*;

use crate::{AppAnchor, AppRoute};

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub id: i64,
    pub post_id: i64,
    pub username: String,
    pub text: String,
    pub created_at: NaiveDateTime,
}

pub struct Comment {
    props: Props,
}

impl Component for Comment {
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
                { format!("{} by {} at {} ", self.props.text, self.props.username, self.props.created_at) }
                <AppAnchor route=AppRoute::EditReply(self.props.post_id, self.props.id)>
                    { "edit" }
                </AppAnchor>
                { " " }
                <AppAnchor route=AppRoute::DeleteReply(self.props.post_id, self.props.id)>
                    { "delete" }
                </AppAnchor>
            </p>
        }
    }
}
