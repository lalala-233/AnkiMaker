pub mod headers;
pub use headers::Headers;

#[derive(Default)]
pub struct SingleFileHeader {
    html: bool,
    separator: String,
    notetype: String,
    deck: String,
}
impl SingleFileHeader {
    pub fn generate_header(&self) -> Vec<String> {
        vec![
            format!("#separator:{}", self.separator),
            format!("#html:{}", self.html),
            format!("#notetype:{}", self.notetype),
            format!("#deck:{}", self.deck),
        ]
    }
}
pub trait ToHeader {
    fn separator(&self) -> String {
        "|".to_string()
    }
    fn html(&self) -> bool {
        false
    }
    fn notetype(&self) -> String;
    fn deck(&self) -> String;
}
impl<T: ToHeader> From<&T> for SingleFileHeader {
    fn from(value: &T) -> Self {
        SingleFileHeader {
            separator: value.separator(),
            html: value.html(),
            notetype: value.notetype(),
            deck: value.deck(),
        }
    }
}
