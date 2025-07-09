pub mod headers;

pub trait ToHeader {
    fn len(&self) -> usize;
    fn separator(&self) -> String {
        "|".to_string()
    }
    fn html(&self) -> bool {
        false
    }
    fn notetype(&self) -> String;
    fn deck(&self) -> String;
}
#[derive(Default)]
pub struct Header {
    html: bool,
    separator: String,
    notetype: String,
    deck: String,
    len: usize,
}
impl Header {
    pub fn generate_header(&self) -> Vec<String> {
        vec![
            format!("#separator:{}", self.separator),
            format!("#html:{}", self.html),
            format!("#notetype:{}", self.notetype),
            format!("#deck:{}", self.deck),
        ]
    }
    pub fn separator(&self) -> String {
        self.separator.clone()
    }
}
impl<T: ToHeader> From<&T> for Header {
    fn from(value: &T) -> Self {
        Self {
            separator: value.separator(),
            html: value.html(),
            notetype: value.notetype(),
            deck: value.deck(),
            len: value.len(),
        }
    }
}
