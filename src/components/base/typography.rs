use yew::prelude::*;
use crate::enums::Weight;
use crate::style::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub size: u8,
    pub weight: Weight,
    pub ellipsis: bool
}

#[function_component]
pub fn Typography(props: &Props) -> Html {

    html! {
        <p style={inline(&[
            Style::from("font-size", props.size.size()),
            Style::from("line-height", (props.size as f32 * 1.2).size()),
            Style::from("overflow", if props.ellipsis {"hidden"} else {"inherit"}),
            Style::from("white-space", if props.ellipsis {"nowrap"} else {"inherit"}),
            Style::from("text-overflow", if props.ellipsis {"ellipsis"} else {"inherit"}),
            props.weight.style(),
        ])}>
            {props.children.clone()}
        </p>
    }
}
