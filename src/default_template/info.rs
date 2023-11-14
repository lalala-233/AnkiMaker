use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Info {
    card_template: String,
    deck: String,
    separator: Option<String>,
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
    const DEFAULT_SEPARATOR: &'static str = "|";
    pub fn generate_header(&self) -> Vec<String> {
        let header = vec![
            format!("#separator:{}", self.separator()),
            "#html:false".to_string(),
            format!("#notetype:{}", self.card_template),
            format!("#deck:{}", self.deck),
        ];
        header
    }
    pub fn separator(&self) -> &str {
        if let Some(separator) = &self.separator {
            separator
        } else {
            Self::DEFAULT_SEPARATOR
        }
    }
    pub fn new(card_template: String, deck: String, separator: Option<String>) -> Self {
        Self {
            card_template,
            deck,
            separator,
        }
    }
}
