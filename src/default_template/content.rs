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
}
