use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Info {
    notetype: String,
    deck: String,
    separator: Option<String>,
}
impl Default for Info {
    fn default() -> Self {
        let str = "".to_string();
        Self {
            notetype: str.clone(),
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
            format!("#notetype:{}", self.notetype),
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
    pub fn _new(notetype: String, deck: String, separator: Option<String>) -> Self {
        Self {
            notetype,
            deck,
            separator,
        }
    }
}
