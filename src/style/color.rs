pub trait Colorable {
    fn color(&self) -> String;
}

pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: Option<f32>
}

impl Colorable for Rgb {
    fn color(&self) -> String {
        let mut value = String::from("rgba(");

        value.push_str(&self.red.to_string());
        value.push_str(", ");
        value.push_str(&self.green.to_string());
        value.push_str(", ");
        value.push_str(&self.blue.to_string());
        value.push_str(", ");
        value.push_str(&self.alpha.unwrap_or(1.0).to_string());
        value.push_str(")");

        value
    }
}

pub struct Hsl {
    pub hue: u8,
    pub saturation: u8,
    pub lightness: u8,
    pub alpha: Option<f32>
}
