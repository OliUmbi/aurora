use yew::prelude::*;
use crate::enums::{Direction, Wrap};
use crate::style::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub direction: Direction,
    pub wrap: Wrap,
    pub gap: u8
}

#[function_component]
pub fn Sequence(props: &Props) -> Html {
    html! {
        <div style={inline(&[
            Style::from("width", "100%"),
            Style::from("height", "100%"),
            Style::from("display", "flex"),
            Style::from("gap", props.gap.size()),
            props.direction.style(),
            props.wrap.style()
        ])}>
            {props.children.clone()}
        </div>
    }
}
