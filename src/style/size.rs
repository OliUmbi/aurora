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

impl Sizable for i8 {
    fn size(&self) -> String {
        self.to_string() + "rem"
    }
}

impl Sizable for i16 {
    fn size(&self) -> String {
        self.to_string() + "rem"
    }
}

impl Sizable for i32 {
    fn size(&self) -> String {
        self.to_string() + "rem"
    }
}

impl Sizable for f32 {
    fn size(&self) -> String {
        self.to_string() + "rem"
    }
}

impl Sizable for f64 {
    fn size(&self) -> String {
        self.to_string() + "rem"
    }
}
