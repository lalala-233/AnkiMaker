use crate::prelude::*;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Content {
    paragraph: Vec<Vec<String>>,
}
impl Content {
    pub fn len(&self) -> usize {
        self.paragraph.first().unwrap().len()
    }
    pub fn into_iter(self) -> std::vec::IntoIter<Vec<String>> {
        self.paragraph.into_iter()
    }
}
