use crate::style::*;

#[derive(PartialEq)]
pub enum Alignment {
    START,
    CENTER,
    END,
}

impl Styleable for Alignment {
    fn style(&self) -> Style {
        let mut value = Style {
            property: String::from("align-items"),
            value: String::from(""),
        };
        match self {
            Alignment::START => value.value = String::from("start"),
            Alignment::CENTER => value.value = String::from("center"),
            Alignment::END => value.value = String::from("end")
        }

        value
    }
}
