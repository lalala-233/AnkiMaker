use super::{Content, Info};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    info: Info,
    content: Content,
}
impl Config {
    pub fn generate(self) -> Vec<String> {
        let separator = self.info.separator();
        let paragraph = self.content.into_iter();
        let mut result = Vec::new();
        result.extend(self.info.generate_header());
        let lines = paragraph.map(|text| {
            let text = text.into_iter();
            text.map(|content| format!("{}{}", content, separator))
                .collect()
        });
        result.extend(lines);
        result
    }
}
