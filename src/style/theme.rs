use crate::style::{Colorable, Rgb};

pub struct Theme<'a> {
    pub background: &'a (dyn Colorable + 'static),
    pub tile: &'a (dyn Colorable + 'static),
}

pub const LIGHT: Theme = Theme {
    background: &Rgb {
        red: 255,
        green: 255,
        blue: 255,
        alpha: None,
    },
    tile: &Rgb {
        red: 10,
        green: 10,
        blue: 10,
        alpha: Some(0.05),
    },
};

pub const DARK: Theme = Theme {
    background: &Rgb {
        red: 10,
        green: 10,
        blue: 10,
        alpha: None,
    },
    tile: &Rgb {
        red: 255,
        green: 255,
        blue: 255,
        alpha: Some(0.05),
    },
};

pub fn current_theme() -> Theme<'static> {
    DARK
}
