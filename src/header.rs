use crate::{default_template, poem_template};

#[derive(Default)]
// _ 意味着打算实现
// __ 意味着暂时没打算实现
pub struct Header {
    separator: Option<String>,
    html: Option<String>,
    __tags: Option<String>,
    __columns: Option<String>,
    notetype: Option<String>,
    deck: Option<String>,
    _notetype_column: Option<i32>,
    _deck_column: Option<i32>,
    _tags_column: Option<i32>,
    __guid_column: Option<i32>,
}
pub trait ToHeader {
    fn separator(&self) -> Option<String>;
    fn html(&self) -> Option<String>;
    fn notetype(&self) -> Option<String>;
    fn deck(&self) -> Option<String>;
    fn to_header(&self) -> Header {
        Header {
            separator: self.separator(),
            html: self.html(),
            notetype: self.notetype(),
            deck: self.deck(),
            ..Default::default()
        }
    }
}
impl<T: ToHeader> From<T> for Header {
    fn from(value: T) -> Self {
        value.to_header()
    }
}
