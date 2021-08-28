use yew::prelude::*;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub route: Option<String>,
}

pub struct NotFound {
    props: Props,
}

impl Component for NotFound {
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
            <>
                <p>{"404 Not Found"}</p>
            </>
        }
    }
}
