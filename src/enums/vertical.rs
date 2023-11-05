use crate::style::*;

#[derive(PartialEq)]
pub enum Vertical {
    TOP,
    CENTER,
    BOTTOM,
}

impl Styleable for Vertical {
    fn style(&self) -> Style {
        let mut value = Style {
            property: String::from("align-items"),
            value: String::from(""),
        };
        match self {
            Vertical::TOP => value.value = String::from("start"),
            Vertical::CENTER => value.value = String::from("center"),
            Vertical::BOTTOM => value.value = String::from("end")
        }

        value
    }
}
