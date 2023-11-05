use yew::prelude::*;
use crate::style::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub width: Option<u32>,
    #[prop_or_default]
    pub height: Option<u32>,
    pub background: bool,
    pub padding: u8
}

#[function_component]
pub fn Tile(props: &Props) -> Html {

    fn dimension(value: Option<u32>) -> String {
        match value {
            Some(value) => {value.size()}
            None => {String::from("100%")}
        }
    }

    html! {
        <div style={inline(&[
            Style::from("width", dimension(props.width)),
            Style::from("height", dimension(props.height)),
            Style::from("background-color", if props.background {current_theme().tile.color()} else {String::from("transparent")}),
            Style::from("padding", props.padding.size())
        ])}>
            {props.children.clone()}
        </div>
    }
}
