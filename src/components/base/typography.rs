use yew::prelude::*;
use crate::components::base::*;
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
            &style("font-size", props.size.size()),
            &style("line-height", (props.size as f32 * 1.2).size()),
            &props.weight.style(),
            &style("overflow", if props.ellipsis {"hidden"} else {"inherit"}),
            &style("white-space", if props.ellipsis {"nowrap"} else {"inherit"}),
            &style("text-overflow", if props.ellipsis {"ellipsis"} else {"inherit"}),
        ])}>
            {props.children.clone()}
        </p>
    }
}
