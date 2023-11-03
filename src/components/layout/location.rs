use yew::prelude::*;
use crate::components::layout::*;
use crate::style::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub horizontal: Horizontal,
    pub vertical: Vertical
}

#[function_component]
pub fn Location(props: &Props) -> Html {

    html! {
        <div style={inline(&[
            &style("display", "flex"),
            &style("flex-grow", "1"),
            &props.horizontal.style(),
            &props.vertical.style()
        ])}>
            {props.children.clone()}
        </div>
    }
}
