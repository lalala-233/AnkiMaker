#[derive(Default)]
// _ 意味着打算实现
// __ 意味着暂时没打算实现
pub struct Header {
    html: bool,
    separator: String,
    _tags: String,
    __columns: Option<String>,
    notetype: String,
    deck: String,
    _notetype_column: Option<i32>,
    _deck_column: Option<i32>,
    _tags_column: Option<i32>,
    __guid_column: Option<i32>,
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
impl<T: ToHeader> From<T> for Header {
    fn from(value: T) -> Self {
        Header {
            separator: value.separator(),
            html: value.html(),
            notetype: value.notetype(),
            deck: value.deck(),
            ..Default::default()
        }
    }
}
