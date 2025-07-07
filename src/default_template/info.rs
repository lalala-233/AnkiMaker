use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone)]
pub struct Info {
    deck: String,
    mode: String,
    notetype: String,
    separator: Option<String>,
}

impl Default for Info {
    fn default() -> Self {
        let str = String::new();
        Self {
            deck: str.clone(),
            mode: "default".to_string(),
            notetype: str,
            separator: None,
        }
    }
}
impl Info {
    pub fn deck(&self) -> String {
        self.deck.clone()
    }
    pub fn notetype(&self) -> String {
        self.notetype.clone()
    }
    pub fn separator(&self) -> String {
        self.separator.clone().unwrap_or_else(|| "|".to_string())
    }
    pub const fn _new(notetype: String, deck: String, separator: Option<String>) -> Self {
        Self {
            deck,
            mode: String::new(),
            notetype,
            separator,
        }
    }
}
