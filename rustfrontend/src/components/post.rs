use chrono::{DateTime, Utc};
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
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
            <p>{ format!("{} by {} at {}", self.props.text, self.props.username, self.props.created_at) }</p>
        }
    }
}
