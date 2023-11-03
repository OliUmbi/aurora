use yew::prelude::*;
use crate::style::{current_theme};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html
}

#[function_component]
pub fn Tile(props: &Props) -> Html {

    html! {
        <div style={format!("background-color: {}", current_theme().background.color())}>
            {props.children.clone()}
        </div>
    }
}
