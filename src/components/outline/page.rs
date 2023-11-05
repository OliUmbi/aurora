use yew::prelude::*;
use crate::components::outline::*;
use crate::style::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component]
pub fn Page(props: &Props) -> Html {

    html! {
        <div style={inline(&[
            Style::from("background-color", current_theme().background.color()),
            Style::from("height", "100%"),
        ])}>
            <Navigation/>
            <Content>{props.children.clone()}</Content>
        </div>
    }
}
