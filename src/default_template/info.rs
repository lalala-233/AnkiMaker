use crate::header::{SingleFileHeader, ToHeader};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Info {
    deck: String,
    mode: String,
    notetype: String,
    separator: Option<String>,
}
impl ToHeader for Info {
    fn separator(&self) -> String {
        if let Some(separator) = self.separator.clone() {
            separator
        } else {
            Self::DEFAULT_SEPARATOR.to_string()
        }
    }
    fn notetype(&self) -> String {
        self.notetype.clone()
    }
    fn deck(&self) -> String {
        self.deck.clone()
    }
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
    const DEFAULT_SEPARATOR: &'static str = "|";
    pub fn generate_header(&self) -> Vec<String> {
        SingleFileHeader::from(self.clone()).generate_header()
    }
    pub fn separator(&self) -> String {
        <Self as ToHeader>::separator(self)
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
