use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html
}

#[function_component]
pub fn Content(props: &Props) -> Html {

    html! {
        <div>
            {props.children.clone()}
        </div>
    }
}
