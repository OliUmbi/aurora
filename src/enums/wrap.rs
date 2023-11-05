use crate::style::*;

#[derive(PartialEq)]
pub enum Wrap {
    NONE,
    NORMAL,
    REVERSE,
}

impl Styleable for Wrap {
    fn style(&self) -> Style {
        let mut value = Style {
            property: String::from("flex-wrap"),
            value: String::from(""),
        };
        match self {
            Wrap::NONE => value.value = String::from("nowrap"),
            Wrap::NORMAL => value.value = String::from("wrap"),
            Wrap::REVERSE => value.value = String::from("wrap-reverse")
        }

        value
    }
}
