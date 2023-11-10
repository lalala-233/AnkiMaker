use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Text(Vec<String>);
impl Default for Text {
    fn default() -> Self {
        Self(vec!["".to_string(); 3])
    }
}
impl Text {
    pub fn into_iter(self) -> std::vec::IntoIter<String> {
        let text = self.0;
        text.into_iter()
    }
}
impl From<Vec<String>> for Text {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}
