
pub struct Style {
    pub property: String,
    pub value: String,
}

impl Style {
    pub fn render(&self) -> String {
        format!("{}: {};", self.property, self.value)
    }
}

pub fn style<P: Into<String>, V: Into<String>>(property: P, value: V) -> Style {
    Style {
        property: String::from(property.into()),
        value: String::from(value.into())
    }
}

pub trait Styleable {
    fn style(&self) -> Style;
}

pub trait Sizable {
    fn size(&self) -> String;
}

impl Sizable for u8 {
    fn size(&self) -> String {
        self.to_string() + "rem"
    }
}

impl Sizable for u16 {
    fn size(&self) -> String {
        self.to_string() + "rem"
    }
}

impl Sizable for u32 {
    fn size(&self) -> String {
        self.to_string() + "rem"
    }
}

pub fn inline(styles: &[&Style]) -> String {
    styles.iter()
        .map(|style| style.render())
        .collect::<Vec<_>>()
        .join("")
        .to_string()
}
