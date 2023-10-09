use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Info {
    card_template: String,
    deck: String,
    separator: Option<char>,
}
impl Default for Info {
    fn default() -> Self {
        let str = "".to_string();
        Self {
            card_template: str.clone(),
            deck: str.clone(),
            separator: None,
        }
    }
}
impl Info {
    const DEFAULT_SEPARATOR: char = '|';
    pub fn generate_header(&self) -> Vec<String> {
        let header = vec![
            format!("#separator:{}", self.separator()),
            "#html:true".to_string(),
            format!("#notetype:{}", self.card_template),
            format!("#deck:{}", self.deck),
        ];
        header
    }
    pub fn separator(&self) -> char {
        if let Some(separator) = self.separator {
            separator
        } else {
            Self::DEFAULT_SEPARATOR
        }
    }
}
