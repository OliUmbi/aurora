use crate::style::{Style, Styleable};

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

