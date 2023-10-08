pub enum Mode {
    Poem,
    Default,
    Unknown,
}

impl From<&str> for Mode {
    fn from(value: &str) -> Self {
        match value {
            "poem" => Self::Poem,
            "default" => Self::Default,
            _ => Mode::Unknown,
        }
    }
}
impl From<String> for Mode {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}
