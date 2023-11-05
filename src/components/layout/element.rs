use yew::prelude::*;
use crate::enums::{Alignment, Direction};
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
            Style::from("width", "fit-content"),
            Style::from("height", "fit-content"),
            Style::from("display", "flex"),
            Style::from("gap", props.gap.size()),
            props.direction.style(),
            props.alignment.style(),
        ])}>
            {props.children.clone()}
        </div>
    }
}
