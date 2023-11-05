use yew::utils::print_node;

pub struct Style {
    pub property: String,
    pub value: String,
}

pub trait Styleable {
    fn style(&self) -> Style;
}

impl Style {
    pub fn from<P: Into<String>, V: Into<String>>(property: P, value: V) -> Self {
        Self {
            property: property.into(),
            value: value.into()
        }
    }

    pub fn render(&self) -> String {
        format!("{}: {};", self.property, self.value)
    }
}

pub fn inline(styles: &[Style]) -> String {
    styles.iter()
        .map(|style| style.render())
        .collect::<Vec<_>>()
        .join("")
        .to_string()
}
