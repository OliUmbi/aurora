use yew::prelude::*;
use crate::components::layout::*;
use crate::style::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub direction: Direction,
    pub alignment: Alignment,
    pub gap: u8
}

#[function_component]
pub fn Element(props: &Props) -> Html {

    html! {
        <div style={inline(&[
            &style("width", "fit-content"),
            &style("height", "fit-content"),
            &style("display", "flex"),
            &props.direction.style(),
            &props.alignment.style(),
            &style("gap", props.gap.size())
        ])}>
            {props.children.clone()}
        </div>
    }
}
