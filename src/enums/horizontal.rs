use crate::style::*;

#[derive(PartialEq)]
pub enum Horizontal {
    LEFT,
    CENTER,
    RIGHT,
}

impl Styleable for Horizontal {
    fn style(&self) -> Style {
        let mut value = Style {
            property: String::from("justify-content"),
            value: String::from(""),
        };
        match self {
            Horizontal::LEFT => value.value = String::from("start"),
            Horizontal::CENTER => value.value = String::from("center"),
            Horizontal::RIGHT => value.value = String::from("end")
        }

        value
    }
}