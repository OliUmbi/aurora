use crate::style::{Style, Styleable};

#[derive(PartialEq)]
pub enum Weight {
    W100,
    W200,
    W300,
    W400,
    W500,
    W600,
    W700,
    W800,
    W900
}

impl Styleable for Weight {
    fn style(&self) -> Style {
        let mut value = Style {
            property: String::from("font-weight"),
            value: String::from(""),
        };

        match self {
            Weight::W100 => value.value = String::from("100"),
            Weight::W200 => value.value = String::from("200"),
            Weight::W300 => value.value = String::from("300"),
            Weight::W400 => value.value = String::from("400"),
            Weight::W500 => value.value = String::from("500"),
            Weight::W600 => value.value = String::from("600"),
            Weight::W700 => value.value = String::from("700"),
            Weight::W800 => value.value = String::from("800"),
            Weight::W900 => value.value = String::from("900")
        }

        value
    }
}
