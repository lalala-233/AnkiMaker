use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Info {
    deck: String,
    mode: String,
    notetype: String,
    separator: Option<String>,
}

impl Default for Info {
    fn default() -> Self {
        let str = "".to_string();
        Self {
            deck: str.clone(),
            mode: "default".to_string(),
            notetype: str.clone(),
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
        self.separator.clone().unwrap_or("|".to_string())
    }
    pub fn _new(notetype: String, deck: String, separator: Option<String>) -> Self {
        Self {
            deck,
            mode: Default::default(),
            notetype,
            separator,
        }
    }
}
