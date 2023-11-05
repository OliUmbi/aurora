use crate::style::*;

#[derive(PartialEq)]
pub enum Direction {
    ROW,
    COLUMN,
}

impl Styleable for Direction {
    fn style(&self) -> Style {
        let mut value = Style {
            property: String::from("flex-direction"),
            value: String::from(""),
        };
        match self {
            Direction::ROW => value.value = String::from("row"),
            Direction::COLUMN => value.value = String::from("column")
        }

        value
    }
}