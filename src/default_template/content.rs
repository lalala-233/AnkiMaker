use super::Text;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Content {
    paragraph: Vec<Text>,
}
impl Content {
    pub fn into_iter(self) -> std::vec::IntoIter<Text> {
        self.paragraph.into_iter()
    }
    pub fn _new(paragraph: Vec<Vec<String>>) -> Self {
        Self {
            paragraph: paragraph.into_iter().map(|text| text.into()).collect(),
        }
    }
}
