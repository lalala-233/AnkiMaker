pub enum Mode {
    Default,
    Poem,
    Unknown(String),
}
impl From<String> for Mode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "default" => Self::Default,
            "poem" => Self::Poem,
            _ => Self::Unknown(value),
        }
    }
}
