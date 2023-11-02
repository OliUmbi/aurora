use crate::style::{Color, Rgb};

pub struct Theme<'a> {
    pub background: &'a (dyn Color + 'static)
}

pub const LIGHT: Theme = Theme {
    background: &(Rgb {
        red: 255,
        green: 255,
        blue: 255,
        alpha: None,
    }),
};

pub const DARK: Theme = Theme {
    background: &(Rgb {
        red: 10,
        green: 10,
        blue: 10,
        alpha: None,
    }),
};

pub fn current_theme() -> Theme<'static>  {
    DARK
}
