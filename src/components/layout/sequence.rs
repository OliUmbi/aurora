use yew::prelude::*;
use crate::components::layout::*;
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
            &style("width", "100%"),
            &style("height", "100%"),
            &style("display", "flex"),
            &props.direction.style(),
            &props.wrap.style(),
            &style("gap", props.gap.size())
        ])}>
            {props.children.clone()}
        </div>
    }
}
